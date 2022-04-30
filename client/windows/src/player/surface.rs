use std::ffi::{c_void, CString};
use std::mem::size_of;
use std::ptr::{self, null, null_mut};
use std::sync::atomic::{AtomicBool, Ordering};

use parking_lot::Mutex;
use windows::core::Interface;
use windows::Foundation::Numerics::Vector2;
use windows::Win32::Foundation::PSTR;
use windows::Win32::Graphics::Direct3D::Fxc::D3DCompile;
use windows::Win32::Graphics::Direct3D::{
    D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST, D3D_DRIVER_TYPE_HARDWARE,
};
use windows::Win32::Graphics::Direct3D11::{
    D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext, ID3D11RenderTargetView, ID3D11Texture2D,
    D3D11_APPEND_ALIGNED_ELEMENT, D3D11_BIND_INDEX_BUFFER, D3D11_BIND_VERTEX_BUFFER,
    D3D11_BUFFER_DESC, D3D11_COMPARISON_ALWAYS, D3D11_CPU_ACCESS_WRITE,
    D3D11_CREATE_DEVICE_VIDEO_SUPPORT, D3D11_FILTER_MIN_MAG_LINEAR_MIP_POINT, D3D11_FLOAT32_MAX,
    D3D11_INPUT_ELEMENT_DESC, D3D11_INPUT_PER_VERTEX_DATA, D3D11_MAP_WRITE_DISCARD,
    D3D11_SAMPLER_DESC, D3D11_SDK_VERSION, D3D11_TEXTURE_ADDRESS_CLAMP, D3D11_USAGE_DYNAMIC,
    D3D11_VIEWPORT,
};
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_ALPHA_MODE_PREMULTIPLIED, DXGI_FORMAT_R16_UINT, DXGI_FORMAT_R32G32B32_FLOAT,
    DXGI_FORMAT_R32G32_FLOAT, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_UNKNOWN, DXGI_SAMPLE_DESC,
};
use windows::Win32::Graphics::Dxgi::{
    IDXGIDevice, IDXGIFactory2, IDXGISwapChain1, DXGI_SCALING_STRETCH, DXGI_SWAP_CHAIN_DESC1,
    DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH, DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
    DXGI_USAGE_RENDER_TARGET_OUTPUT,
};
use windows::Win32::System::WinRT::Composition::ICompositorInterop;
use windows::UI::Composition::ContainerVisual;

use super::VideoPlayer;

const BORDER_TOP: f32 = 1.0;
const BORDER_LEFT: f32 = -1.0;
const BORDER_RIGHT: f32 = 1.0;
const BORDER_BOTTOM: f32 = -1.0;

// Combination of function pointer and opaque data pointer
type ResizeCallback = (unsafe extern "C" fn(*mut c_void, u32, u32), *mut c_void);

pub struct PlayerRenderContext {
    size: Mutex<(u32, u32)>,
    size_changed: AtomicBool,
    d3ddevice: ID3D11Device,
    d3dctx: ID3D11DeviceContext,
    swapchain: IDXGISwapChain1,
    swapchain_render_target: Option<ID3D11RenderTargetView>,
    resize_cb: Mutex<Option<ResizeCallback>>,
}

#[repr(C)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
struct TextureCoordinates {
    x: f32,
    y: f32,
}

#[repr(C)]
struct ShaderInput {
    position: Position,
    texture: TextureCoordinates,
}

pub struct D3d11Device {
    handle: ID3D11Device,
    ctx: ID3D11DeviceContext,
}

pub fn create_d3d11_device() -> D3d11Device {
    unsafe {
        let mut device = None;
        let mut ctx = None;

        D3D11CreateDevice(
            &None,
            D3D_DRIVER_TYPE_HARDWARE,
            &None,
            D3D11_CREATE_DEVICE_VIDEO_SUPPORT,
            null(),
            0,
            D3D11_SDK_VERSION,
            &mut device,
            null_mut(),
            &mut ctx,
        )
        .unwrap();

        D3d11Device {
            handle: device.unwrap(),
            ctx: ctx.unwrap(),
        }
    }
}

pub fn create_swapchain_for_composition(
    device: &D3d11Device,
    width: u32,
    height: u32,
) -> IDXGISwapChain1 {
    unsafe {
        let dxgi_device = device.handle.cast::<IDXGIDevice>().unwrap();
        let adapter = dxgi_device.GetAdapter().unwrap();
        let factory = adapter.GetParent::<IDXGIFactory2>().unwrap();

        let scd = DXGI_SWAP_CHAIN_DESC1 {
            Width: width,
            Height: height,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            BufferCount: 2,
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            AlphaMode: DXGI_ALPHA_MODE_PREMULTIPLIED,
            Scaling: DXGI_SCALING_STRETCH,
            Stereo: false.into(),
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
            Flags: DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH as _,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
        };

        factory
            .CreateSwapChainForComposition(&device.handle, &scd, None)
            .unwrap()
    }
}

impl PlayerRenderContext {
    fn new(
        device: &D3d11Device,
        swapchain: &IDXGISwapChain1,
        width: u32,
        height: u32,
    ) -> PlayerRenderContext {
        let d3ddevice = &device.handle;
        let d3dctx = &device.ctx;

        let viewport = D3D11_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: width as _,
            Height: height as _,
            MinDepth: 0.0,
            MaxDepth: 0.0,
        };

        unsafe {
            d3dctx.RSSetViewports(1, &viewport);
        }

        let back_buffer: ID3D11Texture2D = unsafe { swapchain.GetBuffer(0).unwrap() };
        let swapchain_render_target = unsafe {
            d3ddevice
                .CreateRenderTargetView(back_buffer, null())
                .unwrap()
        };

        let shader = include_str!("shader.hlsl");
        let shader_len = shader.len();
        let shader = CString::new(shader).unwrap();

        let shaders_input_layout;
        let vertex_shader;
        let pixel_shader;

        unsafe {
            let mut vs = None;
            let mut ps = None;
            let mut error = None;

            D3DCompile(
                shader.as_ptr() as _,
                shader_len,
                None,
                null(),
                None,
                "VShader",
                "vs_4_0",
                0,
                0,
                &mut vs,
                &mut error,
            )
            .unwrap();

            D3DCompile(
                shader.as_ptr() as _,
                shader_len,
                None,
                null(),
                None,
                "PShader",
                "ps_4_0",
                0,
                0,
                &mut ps,
                &mut error,
            )
            .unwrap();

            let vs = vs.unwrap();
            let ps = ps.unwrap();

            vertex_shader = d3ddevice
                .CreateVertexShader(vs.GetBufferPointer(), vs.GetBufferSize(), None)
                .unwrap();

            pixel_shader = d3ddevice
                .CreatePixelShader(ps.GetBufferPointer(), ps.GetBufferSize(), None)
                .unwrap();

            let mut position = b"POSITION\0".to_vec();
            let mut texcoord = b"TEXCOORD\0".to_vec();

            let ied = [
                D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: PSTR(position.as_mut_ptr()),
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32G32B32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
                    InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
                D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: PSTR(texcoord.as_mut_ptr()),
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32G32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
                    InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
            ];

            shaders_input_layout = d3ddevice
                .CreateInputLayout(
                    ied.as_ptr(),
                    ied.len() as _,
                    vs.GetBufferPointer(),
                    vs.GetBufferSize(),
                )
                .unwrap();
        }

        let vertices = [
            ShaderInput {
                position: Position {
                    x: BORDER_LEFT,
                    y: BORDER_BOTTOM,
                    z: 0.0,
                },
                texture: TextureCoordinates { x: 0.0, y: 1.0 },
            },
            ShaderInput {
                position: Position {
                    x: BORDER_RIGHT,
                    y: BORDER_BOTTOM,
                    z: 0.0,
                },
                texture: TextureCoordinates { x: 1.0, y: 1.0 },
            },
            ShaderInput {
                position: Position {
                    x: BORDER_RIGHT,
                    y: BORDER_TOP,
                    z: 0.0,
                },
                texture: TextureCoordinates { x: 1.0, y: 0.0 },
            },
            ShaderInput {
                position: Position {
                    x: BORDER_LEFT,
                    y: BORDER_TOP,
                    z: 0.0,
                },
                texture: TextureCoordinates { x: 0.0, y: 0.0 },
            },
        ];

        let bd = D3D11_BUFFER_DESC {
            Usage: D3D11_USAGE_DYNAMIC,
            ByteWidth: size_of::<[ShaderInput; 4]>() as _,
            BindFlags: D3D11_BIND_VERTEX_BUFFER,
            CPUAccessFlags: D3D11_CPU_ACCESS_WRITE,
            ..Default::default()
        };

        let vertex_buffer = unsafe { d3ddevice.CreateBuffer(&bd, null()).unwrap() };
        let vertex_buffer_stride = size_of::<ShaderInput>() as _;

        unsafe {
            let ms = d3dctx
                .Map(&vertex_buffer, 0, D3D11_MAP_WRITE_DISCARD, 0)
                .unwrap();
            std::ptr::copy(vertices.as_ptr(), ms.pData as _, vertices.len());
            d3dctx.Unmap(&vertex_buffer, 0);
        }

        let quad_index_count = 6;

        let quad_desc = D3D11_BUFFER_DESC {
            Usage: D3D11_USAGE_DYNAMIC,
            ByteWidth: size_of::<usize>() as u32 * quad_index_count,
            BindFlags: D3D11_BIND_INDEX_BUFFER,
            CPUAccessFlags: D3D11_CPU_ACCESS_WRITE,
            ..Default::default()
        };

        let index_buffer = unsafe { d3ddevice.CreateBuffer(&quad_desc, null()).unwrap() };

        unsafe {
            let ms = d3dctx
                .Map(&index_buffer, 0, D3D11_MAP_WRITE_DISCARD, 0)
                .unwrap();

            let triangle_pos =
                std::slice::from_raw_parts_mut(ms.pData as *mut usize, quad_index_count as _);

            triangle_pos[0] = 3;
            triangle_pos[1] = 1;
            triangle_pos[2] = 0;

            triangle_pos[3] = 2;
            triangle_pos[4] = 1;
            triangle_pos[5] = 3;

            d3dctx.Unmap(&index_buffer, 0);
        }

        unsafe {
            d3dctx.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            d3dctx.IASetInputLayout(&shaders_input_layout);
            d3dctx.IASetVertexBuffers(0, 1, &Some(vertex_buffer), &vertex_buffer_stride, &0);
            d3dctx.IASetIndexBuffer(&index_buffer, DXGI_FORMAT_R16_UINT, 0);
            d3dctx.VSSetShader(&vertex_shader, null(), 0);
            d3dctx.PSSetShader(&pixel_shader, null(), 0);
        }

        let samp_desc = D3D11_SAMPLER_DESC {
            Filter: D3D11_FILTER_MIN_MAG_LINEAR_MIP_POINT,
            AddressU: D3D11_TEXTURE_ADDRESS_CLAMP,
            AddressV: D3D11_TEXTURE_ADDRESS_CLAMP,
            AddressW: D3D11_TEXTURE_ADDRESS_CLAMP,
            ComparisonFunc: D3D11_COMPARISON_ALWAYS,
            MinLOD: 0.0,
            MaxLOD: D3D11_FLOAT32_MAX,
            ..Default::default()
        };

        let sampler_state = unsafe { d3ddevice.CreateSamplerState(&samp_desc).unwrap() };
        unsafe {
            d3dctx.PSSetSamplers(0, 1, &Some(sampler_state));
        };

        PlayerRenderContext {
            size: Mutex::new((width, height)),
            size_changed: AtomicBool::new(false),
            d3ddevice: d3ddevice.clone(),
            d3dctx: d3dctx.clone(),
            swapchain: swapchain.clone(),
            swapchain_render_target: Some(swapchain_render_target),
            resize_cb: Mutex::new(None),
        }
    }
}

unsafe extern "C" fn setup_cb(
    opaque: *mut *mut c_void,
    _cfg: *const vlc::libvlc_video_setup_device_cfg_t,
    out: *mut vlc::libvlc_video_setup_device_info_t,
) -> bool {
    log::debug!("setup");
    let ctx: *const PlayerRenderContext = *opaque as _;
    (*out).__bindgen_anon_1.d3d11.device_context = std::mem::transmute_copy(&(*ctx).d3dctx);
    true
}

unsafe extern "C" fn cleanup_cb(_opaque: *mut c_void) {
    log::debug!("cleanup");
}

unsafe extern "C" fn resize_cb(
    opaque: *mut c_void,
    report_size_change: Option<unsafe extern "C" fn(*mut c_void, u32, u32)>,
    report_opaque: *mut c_void,
) {
    let ctx: &PlayerRenderContext = &*(opaque as *mut _);

    if let Some(report_size_change) = report_size_change {
        let (width, height) = *ctx.size.lock();
        report_size_change(report_opaque, width, height);
    }

    *ctx.resize_cb.lock() =
        report_size_change.map(|report_size_change| (report_size_change, report_opaque));
}

unsafe extern "C" fn update_output_cb(
    _opaque: *mut c_void,
    _cfg: *const vlc::libvlc_video_render_cfg_t,
    out: *mut vlc::libvlc_video_output_cfg_t,
) -> bool {
    (*out).__bindgen_anon_1.dxgi_format = DXGI_FORMAT_R8G8B8A8_UNORM as _;
    (*out).full_range = true;
    (*out).colorspace = vlc::libvlc_video_color_space_t_libvlc_video_colorspace_BT709;
    (*out).primaries = vlc::libvlc_video_color_primaries_t_libvlc_video_primaries_BT709;
    (*out).transfer = vlc::libvlc_video_transfer_func_t_libvlc_video_transfer_func_SRGB;
    true
}

unsafe extern "C" fn swap_cb(opaque: *mut c_void) {
    let ctx: &PlayerRenderContext = &*(opaque as *const _);
    ctx.swapchain.Present(0, 0).unwrap();
}

unsafe extern "C" fn start_rendering_cb(opaque: *mut c_void, enter: bool) -> bool {
    let ctx: &mut PlayerRenderContext = &mut *(opaque as *mut _);

    if enter {
        // If the size has been updated, recreate swapchain buffers
        if ctx.size_changed.swap(false, Ordering::SeqCst) {
            let (width, height) = *ctx.size.lock();

            ctx.swapchain_render_target.take();
            ctx.swapchain
                .ResizeBuffers(0, width, height, DXGI_FORMAT_UNKNOWN, 0)
                .unwrap();

            let back_buffer = ctx.swapchain.GetBuffer::<ID3D11Texture2D>(0).unwrap();
            let render_target_view = ctx
                .d3ddevice
                .CreateRenderTargetView(back_buffer, ptr::null())
                .unwrap();

            ctx.swapchain_render_target = Some(render_target_view);

            let viewport = D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: width as _,
                Height: height as _,
                MinDepth: 0.0,
                MaxDepth: 0.0,
            };

            ctx.d3dctx.RSSetViewports(1, &viewport);
        }

        ctx.d3dctx
            .OMSetRenderTargets(1, &ctx.swapchain_render_target.clone(), &None);

        ctx.d3dctx
            .ClearRenderTargetView(&ctx.swapchain_render_target, [0.0, 0.0, 0.0, 1.0].as_ptr());
    }

    true
}

unsafe extern "C" fn select_plane_cb(_opaque: *mut c_void, plane: u64, _out: *mut c_void) -> bool {
    plane == 0
}

pub struct VideoPlayerSurface {
    device: D3d11Device,
    swapchain: IDXGISwapChain1,
    width: u32,
    height: u32,
    ctx: Option<Box<PlayerRenderContext>>,
}

impl VideoPlayerSurface {
    pub fn new(width: u32, height: u32) -> VideoPlayerSurface {
        let device = create_d3d11_device();
        let swapchain = create_swapchain_for_composition(&device, width, height);

        VideoPlayerSurface {
            device,
            swapchain,
            width,
            height,
            ctx: None,
        }
    }

    pub fn set_player(&mut self, player: &mut VideoPlayer) {
        let render_context = Box::new(PlayerRenderContext::new(
            &self.device,
            &self.swapchain,
            self.width,
            self.height,
        ));

        unsafe {
            vlc::libvlc_video_set_output_callbacks(
                player.media_player,
                vlc::libvlc_video_engine_t_libvlc_video_engine_d3d11,
                Some(setup_cb),
                Some(cleanup_cb),
                Some(resize_cb),
                Some(update_output_cb),
                Some(swap_cb),
                Some(start_rendering_cb),
                None,
                None,
                Some(select_plane_cb),
                render_context.as_ref() as *const PlayerRenderContext as _,
            );
        }

        self.ctx = Some(render_context);
    }

    pub fn set_visual_target(&mut self, visual: &ContainerVisual) {
        unsafe {
            let compositor = visual.Compositor().unwrap();

            let video_visual = compositor.CreateSpriteVisual().unwrap();
            let surface = compositor
                .cast::<ICompositorInterop>()
                .unwrap()
                .CreateCompositionSurfaceForSwapChain(&self.swapchain)
                .unwrap();

            let brush = compositor.CreateSurfaceBrushWithSurface(surface).unwrap();

            video_visual.SetBrush(brush).unwrap();
            video_visual
                .SetRelativeSizeAdjustment(Vector2 { X: 1.0, Y: 1.0 })
                .unwrap();

            visual
                .Children()
                .unwrap()
                .InsertAtTop(video_visual)
                .unwrap();
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        if let Some(ctx) = self.ctx.as_deref_mut() {
            // Update rendering context size
            {
                let mut size = ctx.size.lock();
                if width != size.0 || height != size.1 {
                    *size = (width, height);
                    // Notify renderer that size has been changed
                    // Swapchain will be updated on the next frame
                    ctx.size_changed.store(true, Ordering::SeqCst);
                }
            }

            // Report size change to vlc
            {
                let resize_cb = ctx.resize_cb.lock();
                if let Some((resize_cb, opaque)) = *resize_cb {
                    unsafe { resize_cb(opaque, width, height) };
                }
            }
        }
    }
}
