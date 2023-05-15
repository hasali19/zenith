use std::ffi::c_void;

use egl::ClientBuffer;
use khronos_egl as egl;
use libloading::Library;
use windows::core::ComInterface;
use windows::Win32::Graphics::Direct3D11::ID3D11Texture2D;
use windows::Win32::Graphics::Dxgi::IDXGIResource;

type EglInstance = egl::DynamicInstance<egl::EGL1_5>;

pub struct EglContext {
    egl: EglInstance,
    config: egl::Config,
    display: egl::Display,
    context: egl::Context,
}

unsafe impl Send for EglContext {}
unsafe impl Sync for EglContext {}

impl EglContext {
    pub fn new() -> Result<EglContext, Box<dyn std::error::Error>> {
        let lib = unsafe { Library::new("libEGL.dll")? };
        let egl = unsafe { EglInstance::load_required_from(lib)? };

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

        let config = configs[0];
        let context_attribs = [egl::CONTEXT_CLIENT_VERSION, 2, egl::NONE];
        let context = egl.create_context(display, config, None, &context_attribs)?;

        Ok(EglContext {
            egl,
            config,
            display,
            context,
        })
    }

    pub fn create_surface(
        &self,
        texture: &ID3D11Texture2D,
    ) -> Result<EglSurface, Box<dyn std::error::Error>> {
        let mut desc = Default::default();
        unsafe { texture.GetDesc(&mut desc) };

        let surface_attribs = [
            egl::WIDTH,
            desc.Width as i32,
            egl::HEIGHT,
            desc.Height as i32,
            egl::TEXTURE_TARGET,
            egl::TEXTURE_2D,
            egl::TEXTURE_FORMAT,
            egl::TEXTURE_RGBA,
            egl::NONE,
        ];

        let texture_handle = unsafe { texture.cast::<IDXGIResource>()?.GetSharedHandle()? };
        let client_buffer = unsafe { ClientBuffer::from_ptr(texture_handle.0 as *mut c_void) };

        let surface = self.egl.create_pbuffer_from_client_buffer(
            self.display,
            0x3200,
            client_buffer,
            self.config,
            &surface_attribs,
        )?;

        Ok(EglSurface(surface))
    }

    pub fn make_context_current(&self) {
        self.egl
            .make_current(self.display, None, None, Some(self.context))
            .unwrap();
    }

    pub fn make_surface_current(&self, surface: &EglSurface) {
        self.egl
            .make_current(
                self.display,
                Some(surface.0),
                Some(surface.0),
                Some(self.context),
            )
            .unwrap();
    }
}

pub struct EglSurface(egl::Surface);

unsafe impl Send for EglSurface {}
