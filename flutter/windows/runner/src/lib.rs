mod flutter_window_binding;
mod flutter_windows;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::HWND;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowBuilder;

use crate::flutter_window_binding::FlutterWindowBinding;
use crate::flutter_windows::FlutterDesktopViewController;

#[no_mangle]
extern "C" fn rust_main() {
    let mut event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Zenith")
        .with_inner_size(LogicalSize::new(1280, 720))
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();

    let view_controller = Box::new(FlutterDesktopViewController::new(
        window_size.width as i32,
        window_size.height as i32,
    ));

    let window_handle = match window.raw_window_handle() {
        RawWindowHandle::Win32(handle) => HWND(handle.hwnd as isize),
        _ => unreachable!(),
    };

    let _binding = FlutterWindowBinding::new(&view_controller, window_handle);

    event_loop.run_return(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            window_id: _,
            event: WindowEvent::CloseRequested,
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
