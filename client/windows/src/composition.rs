use std::mem;

use windows::core::Interface;
use windows::Foundation::Numerics::Vector2;
use windows::System::DispatcherQueueController;
use windows::Win32::System::WinRT::Composition::ICompositorDesktopInterop;
use windows::Win32::System::WinRT::{
    CreateDispatcherQueueController, DispatcherQueueOptions, DQTAT_COM_NONE, DQTYPE_THREAD_CURRENT,
};
use windows::UI::Composition::Desktop::DesktopWindowTarget;
use windows::UI::Composition::{Compositor, ContainerVisual};

use crate::window::Window;

pub struct Composition {
    #[allow(unused)]
    controller: DispatcherQueueController,
    target: DesktopWindowTarget,
}

impl Composition {
    pub fn new(window: &Window) -> Composition {
        let dispatcher_queue_opts = DispatcherQueueOptions {
            dwSize: mem::size_of::<DispatcherQueueOptions>() as _,
            threadType: DQTYPE_THREAD_CURRENT,
            apartmentType: DQTAT_COM_NONE,
        };

        let controller = unsafe { CreateDispatcherQueueController(dispatcher_queue_opts) }.unwrap();
        let compositor = Compositor::new().unwrap();

        let target = unsafe {
            compositor
                .cast::<ICompositorDesktopInterop>()
                .unwrap()
                .CreateDesktopWindowTarget(window.hwnd(), true)
                .unwrap()
        };

        let root = compositor.CreateContainerVisual().unwrap();
        let (width, height) = window.inner_size();

        root.SetSize(Vector2 {
            X: width as f32,
            Y: height as f32,
        })
        .unwrap();

        target.SetRoot(&root).unwrap();

        let brush = compositor.TryCreateBlurredWallpaperBackdropBrush().unwrap();
        let background = compositor.CreateSpriteVisual().unwrap();

        background.SetBrush(brush).unwrap();
        root.Children().unwrap().InsertAtTop(background).unwrap();

        Composition { controller, target }
    }

    pub fn root_visual(&self) -> ContainerVisual {
        self.target.Root().unwrap().cast().unwrap()
    }
}
