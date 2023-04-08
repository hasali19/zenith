use std::ffi::c_void;

use egl::ClientBuffer;
use khronos_egl as egl;
use libloading::Library;
use windows::core::ComInterface;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Graphics::Direct3D11::ID3D11Texture2D;
use windows::Win32::Graphics::Dxgi::IDXGIResource;

type EglInstance = egl::DynamicInstance<egl::EGL1_5>;

pub struct EglContext {
    egl: EglInstance,
    surface: EglSurface,
}

unsafe impl Send for EglContext {}

impl EglContext {
    pub fn for_d3d11_texture(
        texture: &ID3D11Texture2D,
    ) -> Result<EglContext, Box<dyn std::error::Error>> {
        let lib = unsafe { Library::new("libEGL.dll")? };
        let egl = unsafe { EglInstance::load_required_from(lib)? };

        let texture_handle = unsafe { texture.cast::<IDXGIResource>()?.GetSharedHandle()? };
        let surface = EglSurface::new(&egl, texture_handle)?;

        Ok(EglContext { egl, surface })
    }

    pub fn make_current(&self) {
        self.egl
            .make_current(
                self.surface.display,
                Some(self.surface.surface),
                Some(self.surface.surface),
                Some(self.surface.context),
            )
            .unwrap();
    }

    pub fn bind_tex_image(&self) {
        self.egl
            .bind_tex_image(self.surface.display, self.surface.surface, egl::BACK_BUFFER)
            .unwrap();
    }
}

struct EglSurface {
    display: egl::Display,
    context: egl::Context,
    surface: egl::Surface,
}

impl EglSurface {
    pub fn new(
        egl: &EglInstance,
        dxgi_handle: HANDLE,
    ) -> Result<EglSurface, Box<dyn std::error::Error>> {
        let display = egl.get_platform_display(
            0x3202,
            egl::DEFAULT_DISPLAY,
            &[0x3203, 0x3208, 0x320f, 1, 0x3038],
        )?;

        egl.initialize(display)?;

        let mut configs = Vec::with_capacity(1);
        let config_attribs = [
            egl::RED_SIZE,
            8,
            egl::GREEN_SIZE,
            8,
            egl::BLUE_SIZE,
            8,
            egl::ALPHA_SIZE,
            8,
            egl::DEPTH_SIZE,
            8,
            egl::STENCIL_SIZE,
            8,
            egl::NONE,
        ];

        egl.choose_config(display, &config_attribs, &mut configs)?;

        let context_attribs = [egl::CONTEXT_CLIENT_VERSION, 2, egl::NONE];
        let context = egl.create_context(display, configs[0], None, &context_attribs)?;

        let surface_attribs = [
            egl::WIDTH,
            1920,
            egl::HEIGHT,
            1080,
            egl::TEXTURE_TARGET,
            egl::TEXTURE_2D,
            egl::TEXTURE_FORMAT,
            egl::TEXTURE_RGBA,
            egl::NONE,
        ];

        let client_buffer = unsafe { ClientBuffer::from_ptr(dxgi_handle.0 as *mut c_void) };

        let surface = egl.create_pbuffer_from_client_buffer(
            display,
            0x3200,
            client_buffer,
            configs[0],
            &surface_attribs,
        )?;

        Ok(EglSurface {
            display,
            context,
            surface,
        })
    }
}
