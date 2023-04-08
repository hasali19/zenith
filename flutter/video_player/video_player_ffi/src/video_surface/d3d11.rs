use std::ffi::c_void;
use std::sync::{Arc, Mutex};

use windows::core::ComInterface;
use windows::Win32::Graphics::Direct3D::D3D_DRIVER_TYPE_HARDWARE;
use windows::Win32::Graphics::Direct3D11::{
    D3D11CreateDevice, ID3D11DeviceContext, ID3D11Texture2D, D3D11_BIND_RENDER_TARGET,
    D3D11_BIND_SHADER_RESOURCE, D3D11_CPU_ACCESS_FLAG, D3D11_CREATE_DEVICE_FLAG,
    D3D11_RESOURCE_MISC_SHARED, D3D11_SDK_VERSION, D3D11_TEXTURE2D_DESC, D3D11_USAGE_DEFAULT,
};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_SAMPLE_DESC};
use windows::Win32::Graphics::Dxgi::IDXGIResource;

pub struct D3d11Context {
    context: ID3D11DeviceContext,
    video_texture: ID3D11Texture2D,
    display_texture: ID3D11Texture2D,
    texture_mutex: Arc<Mutex<()>>,
}

impl D3d11Context {
    pub fn new(texture_mutex: Arc<Mutex<()>>) -> windows::core::Result<D3d11Context> {
        let (device, context) = unsafe {
            let mut device = None;
            let mut context = None;

            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_HARDWARE,
                None,
                D3D11_CREATE_DEVICE_FLAG::default(),
                None,
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                Some(&mut context),
            )?;

            (device.unwrap(), context.unwrap())
        };

        let texture_desc = D3D11_TEXTURE2D_DESC {
            Width: 1920,
            Height: 1080,
            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
            MipLevels: 1,
            ArraySize: 1,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_RENDER_TARGET | D3D11_BIND_SHADER_RESOURCE,
            CPUAccessFlags: D3D11_CPU_ACCESS_FLAG::default(),
            MiscFlags: D3D11_RESOURCE_MISC_SHARED,
        };

        let create_texture = || unsafe {
            let mut texture = None;
            device.CreateTexture2D(&texture_desc, None, Some(&mut texture))?;
            windows::core::Result::Ok(texture.unwrap())
        };

        let video_texture = create_texture()?;
        let display_texture = create_texture()?;

        Ok(D3d11Context {
            context,
            video_texture,
            display_texture,
            texture_mutex,
        })
    }

    pub fn video_texture(&self) -> &ID3D11Texture2D {
        &self.video_texture
    }

    pub fn display_texture_handle(&self) -> *mut c_void {
        unsafe {
            self.display_texture
                .cast::<IDXGIResource>()
                .unwrap()
                .GetSharedHandle()
                .unwrap()
                .0 as *mut c_void
        }
    }

    pub fn copy_frame(&self) {
        let _guard = self.texture_mutex.lock().unwrap();
        unsafe {
            self.context
                .CopyResource(&self.display_texture, &self.video_texture);
            self.context.Flush();
        }
    }
}

impl Drop for D3d11Context {
    fn drop(&mut self) {
        println!("destroying d3d11 resources");
    }
}
