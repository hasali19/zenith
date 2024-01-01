#![windows_subsystem = "windows"]

mod flutter_window_binding;
mod flutter_windows;
mod window_placement;

use flutter_desktop_messenger::codec::EncodableValue;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{LoadIconW, SendMessageW, ICON_SMALL, WM_SETICON};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;
use winit::window::{Fullscreen, WindowBuilder};

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

pub fn main() {
    let mut event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Zenith")
        .with_inner_size(LogicalSize::new(1280, 720))
        .with_min_inner_size(LogicalSize::new(300, 300))
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    let window_handle = match window.window_handle().unwrap().as_raw() {
        RawWindowHandle::Win32(handle) => HWND(handle.hwnd.get()),
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

    let messenger = &view_controller.engine().create_messenger();

    messenger.set_callback("zenith.hasali.uk/windowing", move |name, args, reply| {
        if name == "isWindowed" {
            reply.success(&EncodableValue::Bool(true));
        } else if name == "setFullscreen" {
            window.set_fullscreen(
                args.as_bool()
                    .expect("setFullscreen arg must be bool")
                    .then_some(Fullscreen::Borderless(None)),
            );
            reply.success(&EncodableValue::Null);
        } else {
            reply.not_implemented();
        }
    });

    let _binding = FlutterWindowBinding::new(&view_controller, window_handle);

    event_loop
        .run_on_demand(|event, t| {
            if let Event::WindowEvent {
                window_id: _,
                event: WindowEvent::CloseRequested,
            } = event
            {
                t.exit();
            }
        })
        .unwrap();

    if let Err(e) = window_placement::try_save(&window_placement_path, window_handle) {
        eprintln!("failed to save window placement state: {e}");
    }
}
