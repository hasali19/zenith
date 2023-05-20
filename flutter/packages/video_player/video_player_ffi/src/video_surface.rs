mod d3d11;
mod egl;
mod mpv_render_context;

use std::ffi::CStr;
use std::ptr;
use std::sync::{mpsc, Arc, Mutex};

use flutter_plugin::sys::{
    FlutterDesktopGpuSurfaceDescriptor,
    FlutterDesktopPixelFormat_kFlutterDesktopPixelFormatBGRA8888,
};
use flutter_plugin::texture_registrar::{
    FlutterDesktopGpuSurfaceType, FlutterDesktopTextureInfo, FlutterDesktopTextureRegistrar,
};
use libloading::Library;
use windows::core::ComInterface;
use windows::Win32::Graphics::Direct3D11::ID3D11Texture2D;
use windows::Win32::Graphics::Dxgi::IDXGIResource;

use crate::mpv_player::MpvPlayer;

use self::d3d11::D3d11Context;
use self::egl::{EglContext, EglSurface};
use self::mpv_render_context::MpvRenderContext;

enum RenderEvent {
    Render,
    Exit,
}

pub struct VideoSurface {
    texture_id: i64,
    event_sender: mpsc::Sender<RenderEvent>,
    exit_receiver: mpsc::Receiver<()>,
}

impl VideoSurface {
    pub fn new(
        mpv: &MpvPlayer,
        texture_registrar: Arc<FlutterDesktopTextureRegistrar>,
    ) -> Result<VideoSurface, Box<dyn std::error::Error>> {
        let d3d11 = D3d11Context::new()?;
        let egl = Arc::new(EglContext::new()?);

        let (event_tx, event_rx) = mpsc::channel();
        let mpv_render_ctx = create_mpv_render_context(mpv, &egl, event_tx.clone())?;

        // Defer actually creating the texture until we get the view size from the flutter
        // framework. The texture will be created by the texture callback passed to flutter.
        let video_texture = Arc::new(Mutex::new(None));

        let texture_id = register_flutter_texture(
            d3d11,
            egl.clone(),
            &texture_registrar,
            video_texture.clone(),
        );

        let renderer = Renderer {
            mpv_render_ctx,
            egl,
            video_texture,
            event_rx,
        };

        let (exit_tx, exit_rx) = mpsc::channel();

        std::thread::spawn(move || {
            renderer
                .run_loop(|| texture_registrar.mark_external_texture_frame_available(texture_id));
            texture_registrar.unregister_external_texture(texture_id);
            exit_tx.send(()).unwrap();
        });

        Ok(VideoSurface {
            texture_id,
            event_sender: event_tx,
            exit_receiver: exit_rx,
        })
    }

    pub fn texture_id(&self) -> i64 {
        self.texture_id
    }

    pub fn destroy(self) {
        self.event_sender.send(RenderEvent::Exit).unwrap();
        self.exit_receiver.recv().unwrap();
    }
}

struct Renderer {
    mpv_render_ctx: MpvRenderContext,
    video_texture: Arc<Mutex<Option<VideoTexture>>>,
    egl: Arc<EglContext>,
    event_rx: mpsc::Receiver<RenderEvent>,
}

impl Renderer {
    fn run_loop(self, frame_callback: impl Fn()) {
        while let Ok(event) = self.event_rx.recv() {
            match event {
                RenderEvent::Render => {
                    if self.mpv_render_ctx.update() {
                        self.render(&frame_callback);
                    }
                }
                RenderEvent::Exit => break,
            }
        }
    }

    fn render(&self, frame_callback: &impl Fn()) {
        let video_texture = self.video_texture.lock().unwrap();
        if let Some(video_texture) = &*video_texture {
            self.egl.make_surface_current(&video_texture.egl_surface);

            self.mpv_render_ctx
                .render(video_texture.width as i32, video_texture.height as i32);

            unsafe { gl::Finish() };

            frame_callback();
        }
    }
}

struct VideoTexture {
    texture: ID3D11Texture2D,
    egl_surface: EglSurface,
    width: u32,
    height: u32,
}

impl VideoTexture {
    fn new(
        d3d11_context: &D3d11Context,
        egl: &EglContext,
        width: u32,
        height: u32,
    ) -> Result<VideoTexture, Box<dyn std::error::Error>> {
        let video_texture = d3d11_context.create_texture(width, height).unwrap();
        let egl_surface = egl.create_surface(&video_texture).unwrap();
        Ok(VideoTexture {
            texture: video_texture,
            egl_surface,
            width,
            height,
        })
    }
}

struct DisplayTexture {
    texture: ID3D11Texture2D,
    descriptor: FlutterDesktopGpuSurfaceDescriptor,
}

impl DisplayTexture {
    fn new(d3d11: &D3d11Context, width: u32, height: u32) -> DisplayTexture {
        let texture = d3d11.create_texture(width, height).unwrap();

        let texture_handle = unsafe {
            texture
                .cast::<IDXGIResource>()
                .unwrap()
                .GetSharedHandle()
                .unwrap()
        };

        let descriptor = FlutterDesktopGpuSurfaceDescriptor {
            struct_size: std::mem::size_of::<FlutterDesktopGpuSurfaceDescriptor>(),
            handle: texture_handle.0 as _,
            width: width as usize,
            visible_width: width as usize,
            height: height as usize,
            visible_height: height as usize,
            format: FlutterDesktopPixelFormat_kFlutterDesktopPixelFormatBGRA8888,
            release_callback: None,
            release_context: ptr::null_mut(),
        };

        DisplayTexture {
            texture,
            descriptor,
        }
    }
}

fn register_flutter_texture(
    d3d11: D3d11Context,
    egl: Arc<EglContext>,
    texture_registrar: &FlutterDesktopTextureRegistrar,
    video_texture: Arc<Mutex<Option<VideoTexture>>>,
) -> i64 {
    let display_texture = Mutex::new(None::<DisplayTexture>);

    texture_registrar.register_external_texture(Box::new(FlutterDesktopTextureInfo::GpuSurface {
        surface_type: FlutterDesktopGpuSurfaceType::DxgiSharedHandle,
        callback: Box::new(move |width, height| unsafe {
            let mut display_texture = display_texture.lock().unwrap();

            let recreate_textures = match display_texture.as_mut() {
                None => true,
                Some(display_texture) => {
                    display_texture.descriptor.width != width
                        || display_texture.descriptor.height != height
                }
            };

            if recreate_textures {
                *display_texture = Some(DisplayTexture::new(&d3d11, width as u32, height as u32));
            }

            // We make use of two textures: the video texture and the display texture. The video
            // texture is rendered to by libmpv, and is shared between the mpv render thread and
            // this callback. The texture is copied into the display texture below, which is then
            // passed to flutter. Using two textures appears to eliminate synchronization issues
            // caused by mpv writing to the texture while it is being used by flutter.

            let display_texture = display_texture.as_mut().unwrap();
            let mut video_texture = video_texture.lock().unwrap();

            if let Some(video_texture) = &*video_texture {
                d3d11.copy_texture(&video_texture.texture, &display_texture.texture);
            }

            // If the view was resized, we recreate the video texture after copying the original
            // video frame to the resized display texture. Although the size will not match, at
            // least the user will see something.
            if recreate_textures {
                *video_texture =
                    Some(VideoTexture::new(&d3d11, &egl, width as u32, height as u32).unwrap());
            }

            // Extend the lifetime of texture to 'static. This should be safe since
            // texture will only be dropped when this closure is dropped, which happens
            // when the texture is unregistered from flutter.
            std::mem::transmute(&display_texture.descriptor)
        }),
    }))
}

fn create_mpv_render_context(
    mpv: &MpvPlayer,
    egl: &EglContext,
    event_tx: mpsc::Sender<RenderEvent>,
) -> Result<MpvRenderContext, Box<dyn std::error::Error>> {
    let gl_lib = unsafe { Library::new("libGLESv2.dll") }?;

    let loader = |s: &str| unsafe {
        gl_lib
            .get::<unsafe extern "C" fn()>(s.as_bytes())
            .unwrap()
            .into_raw()
            .into_raw()
            .cast_const()
            .cast()
    };

    gl::Finish::load_with(loader);

    egl.make_context_current();

    let get_proc_address = Box::new(move |name: &CStr| unsafe {
        gl_lib
            .get::<unsafe extern "C" fn()>(name.to_bytes_with_nul())
            .unwrap()
            .into_raw()
            .into_raw()
            .cast()
    });

    Ok(MpvRenderContext::new(
        mpv,
        get_proc_address,
        Box::new(move || event_tx.send(RenderEvent::Render).unwrap()),
    ))
}
