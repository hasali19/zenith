use std::cell::RefCell;

use flutter_plugin::codec::EncodableValue;
use flutter_plugin::messenger::FlutterDesktopMessengerReply;
use flutter_plugin::registrar::FlutterDesktopPluginRegistrar;
use flutter_plugin::{flutter_plugin, FlutterDesktopPlugin};
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetAncestor, GetWindowLongPtrW, GetWindowRect, SetWindowLongPtrW, SetWindowPos, GA_ROOT,
    GWL_STYLE, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOZORDER, WS_CAPTION, WS_THICKFRAME,
};

struct VideoPlayerFfiPlugin {
    window_state: RefCell<WindowState>,
}

#[derive(Default)]
struct WindowState {
    hwnd: HWND,
    is_full_screen: bool,
    saved_style: isize,
    saved_rect: RECT,
}

impl FlutterDesktopPlugin for VideoPlayerFfiPlugin {
    fn register_with_registrar(registrar: &FlutterDesktopPluginRegistrar) {
        let flutter_window = registrar.view().hwnd();
        let root_window = unsafe { GetAncestor(flutter_window, GA_ROOT) };

        let plugin = VideoPlayerFfiPlugin {
            window_state: RefCell::new(WindowState {
                hwnd: root_window,
                ..Default::default()
            }),
        };

        registrar
            .messenger()
            .set_callback("video_player_ffi", move |name, args, reply| {
                plugin.handle_method_call(name, args, reply)
            });
    }
}

impl VideoPlayerFfiPlugin {
    fn handle_method_call(
        &self,
        name: &str,
        args: EncodableValue,
        reply: FlutterDesktopMessengerReply,
    ) {
        match name {
            "setFullScreen" => {
                let args = args.as_map().unwrap();
                let is_full_screen = args
                    .get(&EncodableValue::String("isFullScreen"))
                    .unwrap()
                    .as_bool()
                    .unwrap();

                self.set_full_screen(is_full_screen);

                reply.success(&EncodableValue::Null);
            }
            _ => reply.not_implemented(),
        }
    }

    fn set_full_screen(&self, is_full_screen: bool) {
        unsafe {
            let mut window_state = self.window_state.borrow_mut();
            let root_window = GetAncestor(window_state.hwnd, GA_ROOT);

            if !window_state.is_full_screen {
                window_state.saved_style = GetWindowLongPtrW(root_window, GWL_STYLE);
                GetWindowRect(root_window, &mut window_state.saved_rect);
            }

            window_state.is_full_screen = is_full_screen;

            unsafe fn get_monitor_size(window: HWND) -> RECT {
                let mut monitor_info = MONITORINFO {
                    cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                    ..Default::default()
                };

                GetMonitorInfoW(
                    MonitorFromWindow(window, MONITOR_DEFAULTTONEAREST),
                    &mut monitor_info,
                );

                monitor_info.rcMonitor
            }

            unsafe fn set_window_rect(window: HWND, rect: &RECT) {
                SetWindowPos(
                    window,
                    HWND::default(),
                    rect.left,
                    rect.top,
                    rect.right - rect.left,
                    rect.bottom - rect.top,
                    SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED,
                );
            }

            if window_state.is_full_screen {
                SetWindowLongPtrW(
                    root_window,
                    GWL_STYLE,
                    window_state.saved_style & !(WS_CAPTION.0 as isize | WS_THICKFRAME.0 as isize),
                );
                let monitor_size = get_monitor_size(root_window);
                set_window_rect(root_window, &monitor_size);
            } else {
                SetWindowLongPtrW(root_window, GWL_STYLE, window_state.saved_style);
                set_window_rect(root_window, &window_state.saved_rect);
            }
        }
    }
}

flutter_plugin!(VideoPlayerFfiPlugin);
