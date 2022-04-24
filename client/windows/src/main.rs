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
    let compositor = Composition::new(&window);

    let webview = WebView::new(&window);
    let registration = webview.set_message_handler(WebViewHandler);

    webview.set_visual_target(&compositor.root_visual());

    if let Ok(server) = std::env::var("ZENITH_SERVER") {
        webview.navigate_to_url(&server);
    } else {
        webview.navigate_to_string(include_str!("index.html"));
    }

    let handler = WindowHandler {
        compositor,
        webview,
    };

    window::run(&window, handler);

    drop(registration);
}

struct WindowHandler {
    compositor: Composition,
    webview: WebView,
}

impl window::Handler for WindowHandler {
    fn on_resize(&mut self, window: &Window) -> bool {
        let (width, height) = window.inner_size();
        self.compositor.set_size(width, height);
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

struct WebViewHandler;

impl webview::Handler for WebViewHandler {
    fn on_message_received(&mut self, webview: &WebView, message: String) {
        if message.starts_with("server:") {
            webview.navigate_to_url(message.strip_prefix("server:").unwrap());
        }
    }
}
