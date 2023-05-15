use windows::Win32::Graphics::Direct3D::D3D_DRIVER_TYPE_HARDWARE;
use windows::Win32::Graphics::Direct3D11::{
    D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext, ID3D11Texture2D,
    D3D11_BIND_RENDER_TARGET, D3D11_BIND_SHADER_RESOURCE, D3D11_CPU_ACCESS_FLAG,
    D3D11_CREATE_DEVICE_FLAG, D3D11_RESOURCE_MISC_SHARED, D3D11_SDK_VERSION, D3D11_TEXTURE2D_DESC,
    D3D11_USAGE_DEFAULT,
};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_SAMPLE_DESC};

pub struct D3d11Context {
    device: ID3D11Device,
    context: ID3D11DeviceContext,
}

impl D3d11Context {
    pub fn new() -> windows::core::Result<D3d11Context> {
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

        Ok(D3d11Context { device, context })
    }

    pub fn create_texture(
        &self,
        width: u32,
        height: u32,
    ) -> windows::core::Result<ID3D11Texture2D> {
        let texture_desc = D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
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

        let mut texture = None;
        unsafe {
            self.device
                .CreateTexture2D(&texture_desc, None, Some(&mut texture))?;
        }

        Ok(texture.unwrap())
    }

    pub fn copy_texture(&self, src: &ID3D11Texture2D, dst: &ID3D11Texture2D) {
        unsafe {
            self.context
                .CopySubresourceRegion(dst, 0, 0, 0, 0, src, 0, None);
            self.context.Flush();
        }
    }
}
