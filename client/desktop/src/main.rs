mod composition;
mod webview;
mod window;

use std::ptr;

use composition::Composition;
use webview::WebView;
use window::Window;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};
use windows::Win32::UI::HiDpi;

fn main() {
    unsafe { CoInitializeEx(ptr::null_mut(), COINIT_APARTMENTTHREADED).unwrap() };
    unsafe { HiDpi::SetProcessDpiAwareness(HiDpi::PROCESS_PER_MONITOR_DPI_AWARE).unwrap() };

    let window = Window::new("Zenith");
    let hwnd = window.hwnd();

    let compositor = Composition::new(&window);
    let mut webview = WebView::new(hwnd, window.inner_size());
    webview.set_visual_target(&compositor.root_visual());
    webview.navigate_to_url("https://zenith.hasali.uk");

    window::run(&window, Handler { webview });
}

struct Handler {
    webview: WebView,
}

impl window::Handler for Handler {
    fn on_resize(&mut self, window: &Window) -> bool {
        let (width, height) = window.inner_size();
        self.webview.set_size(width, height);
        true
    }

    fn on_mouse_move(&mut self, window: &Window, event: window::MouseMoveEvent) -> bool {
        self.webview.send_mouse_move_event(window, event);
        true
    }

    fn on_mouse_button(&mut self, window: &Window, event: window::MouseButtonEvent) -> bool {
        self.webview.send_mouse_button_event(window, event);
        true
    }

    fn on_mouse_wheel(&mut self, window: &Window, event: window::MouseWheelEvent) -> bool {
        self.webview.send_mouse_wheel_event(window, event);
        true
    }

    fn on_pointer(&mut self, window: &Window, event: window::PointerEvent) -> bool {
        self.webview.send_pointer_event(window, event)
    }
}
