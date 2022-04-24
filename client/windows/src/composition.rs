use std::mem;

use windows::core::{Interface, Result};
use windows::Foundation::Numerics::Vector2;
use windows::System::DispatcherQueueController;
use windows::Win32::System::WinRT::Composition::ICompositorDesktopInterop;
use windows::Win32::System::WinRT::{
    CreateDispatcherQueueController, DispatcherQueueOptions, DQTAT_COM_NONE, DQTYPE_THREAD_CURRENT,
};
use windows::UI::Color;
use windows::UI::Composition::Desktop::DesktopWindowTarget;
use windows::UI::Composition::{Compositor, ContainerVisual};
use winit::window::Window;

use crate::ext::WindowExt;

pub struct Composition {
    #[allow(unused)]
    controller: DispatcherQueueController,
    target: DesktopWindowTarget,
}

impl Composition {
    pub fn new(window: &Window) -> Result<Composition> {
        let dispatcher_queue_opts = DispatcherQueueOptions {
            dwSize: mem::size_of::<DispatcherQueueOptions>() as _,
            threadType: DQTYPE_THREAD_CURRENT,
            apartmentType: DQTAT_COM_NONE,
        };

        let controller = unsafe { CreateDispatcherQueueController(dispatcher_queue_opts)? };
        let compositor = Compositor::new()?;

        let target = unsafe {
            compositor
                .cast::<ICompositorDesktopInterop>()?
                .CreateDesktopWindowTarget(window.hwnd(), true)?
        };

        let root = compositor.CreateContainerVisual()?;
        let size = window.inner_size();

        root.SetSize(Vector2 {
            X: size.width as f32,
            Y: size.height as f32,
        })?;

        target.SetRoot(&root)?;

        let background = compositor.CreateSpriteVisual()?;
        let background_brush = compositor.CreateColorBrushWithColor(Color {
            R: 51,
            G: 51,
            B: 51,
            A: 255,
        })?;

        background.SetBrush(background_brush)?;
        background.SetRelativeSizeAdjustment(Vector2 { X: 1.0, Y: 1.0 })?;

        root.Children()?.InsertAtTop(background)?;

        Ok(Composition { controller, target })
    }

    pub fn root_visual(&self) -> ContainerVisual {
        self.target.Root().unwrap().cast().unwrap()
    }

    pub fn set_size(&self, width: u32, height: u32) {
        self.root_visual()
            .SetSize(Vector2 {
                X: width as f32,
                Y: height as f32,
            })
            .unwrap();
    }
}
