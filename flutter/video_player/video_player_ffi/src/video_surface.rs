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

use crate::mpv_player::MpvPlayer;

use self::d3d11::D3d11Context;
use self::egl::EglContext;
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

unsafe impl Send for VideoSurface {}

impl VideoSurface {
    pub fn new(
        mpv: &MpvPlayer,
        texture_registrar: Arc<FlutterDesktopTextureRegistrar>,
    ) -> VideoSurface {
        let mutex = Arc::new(Mutex::new(()));
        let d3d11_context = Arc::new(D3d11Context::new(mutex.clone()).unwrap());
        let egl = EglContext::for_d3d11_texture(d3d11_context.video_texture()).unwrap();

        let texture_id = register_flutter_texture(d3d11_context, &texture_registrar);

        let (event_tx, event_rx) = mpsc::channel();

        let render_callback = Box::new({
            let tx = event_tx.clone();
            move || tx.send(RenderEvent::Render).unwrap()
        });

        let mpv_render_ctx = create_mpv_render_context(mpv, &egl, render_callback).unwrap();

        let (exit_tx, exit_rx) = mpsc::channel();
        std::thread::spawn(move || {
            while let Ok(event) = event_rx.recv() {
                match event {
                    RenderEvent::Render => {
                        if mpv_render_ctx.update() {
                            let _guard = mutex.lock().unwrap();

                            egl.make_current();
                            mpv_render_ctx.render(1920, 1080);

                            unsafe { gl::Finish() };

                            texture_registrar.mark_external_texture_frame_available(texture_id);
                        }
                    }
                    RenderEvent::Exit => break,
                }
            }

            drop(mpv_render_ctx);

            texture_registrar.unregister_external_texture(texture_id);

            exit_tx.send(()).unwrap();
        });

        VideoSurface {
            texture_id,
            event_sender: event_tx,
            exit_receiver: exit_rx,
        }
    }

    pub fn texture_id(&self) -> i64 {
        self.texture_id
    }

    pub fn destroy(self) {
        self.event_sender.send(RenderEvent::Exit).unwrap();
        self.exit_receiver.recv().unwrap();
    }
}

fn register_flutter_texture(
    d3d11_context: Arc<D3d11Context>,
    texture_registrar: &FlutterDesktopTextureRegistrar,
) -> i64 {
    let texture = Box::new(FlutterDesktopGpuSurfaceDescriptor {
        struct_size: std::mem::size_of::<FlutterDesktopGpuSurfaceDescriptor>(),
        handle: d3d11_context.display_texture_handle(),
        width: 1920,
        visible_width: 1920,
        height: 1080,
        visible_height: 1080,
        format: FlutterDesktopPixelFormat_kFlutterDesktopPixelFormatBGRA8888,
        release_callback: None,
        release_context: ptr::null_mut(),
    });

    texture_registrar.register_external_texture(Box::new(FlutterDesktopTextureInfo::GpuSurface {
        surface_type: FlutterDesktopGpuSurfaceType::DxgiSharedHandle,
        callback: Box::new(move |_width, _height| unsafe {
            d3d11_context.copy_frame();
            // Extend the lifetime of texture to 'static. This should be safe since
            // texture will only be dropped when this closure is dropped, which happens
            // when the texture is unregistered from flutter.
            std::mem::transmute(&*texture)
        }),
    }))
}

fn create_mpv_render_context(
    mpv: &MpvPlayer,
    egl: &EglContext,
    render_callback: Box<dyn Fn()>,
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

    gl::GenTextures::load_with(loader);
    gl::BindTexture::load_with(loader);
    gl::TexParameteri::load_with(loader);
    gl::ClearColor::load_with(loader);
    gl::Clear::load_with(loader);
    gl::Flush::load_with(loader);
    gl::Finish::load_with(loader);
    gl::Viewport::load_with(loader);

    egl.make_current();

    unsafe {
        let mut texture = 0;
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        egl.bind_tex_image();
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
    }

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
        Box::new(render_callback),
    ))
}
