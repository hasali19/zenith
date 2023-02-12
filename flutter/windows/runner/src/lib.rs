mod flutter_window_binding;
mod flutter_windows;
mod window_placement;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{LoadIconW, SendMessageW, ICON_SMALL, WM_SETICON};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowBuilder;

use crate::flutter_window_binding::FlutterWindowBinding;
use crate::flutter_windows::FlutterDesktopViewController;

/// See resource.h
const IDI_APP_ICON: PCWSTR = PCWSTR(101usize as _);

fn set_window_icon(window: HWND) {
    unsafe {
        let hinstance = GetModuleHandleW(None).unwrap();
        let icon = LoadIconW(hinstance, IDI_APP_ICON).unwrap();
        SendMessageW(window, WM_SETICON, WPARAM(ICON_SMALL as _), LPARAM(icon.0));
    }
}

#[no_mangle]
extern "C" fn rust_main() {
    let mut event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Zenith")
        .with_inner_size(LogicalSize::new(1280, 720))
        .with_min_inner_size(LogicalSize::new(300, 300))
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    let window_handle = match window.raw_window_handle() {
        RawWindowHandle::Win32(handle) => HWND(handle.hwnd as isize),
        _ => unreachable!(),
    };

    set_window_icon(window_handle);

    let window_placement_path = dirs::data_local_dir()
        .unwrap()
        .join("Zenith/windowplacement");

    if let Err(e) = window_placement::try_restore(&window_placement_path, window_handle) {
        eprintln!("failed to restore window placement state: {e}");
    }

    window.set_visible(true);

    let window_size = window.inner_size();

    let view_controller = Box::new(FlutterDesktopViewController::new(
        window_size.width as i32,
        window_size.height as i32,
    ));

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

    if let Err(e) = window_placement::try_save(&window_placement_path, window_handle) {
        eprintln!("failed to save window placement state: {e}");
    }
}
