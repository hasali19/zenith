mod composition;
mod ext;
mod webview;

use std::ptr;
use std::rc::Rc;

use composition::Composition;
use serde::Deserialize;
use webview::WebView;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};
use windows::Win32::UI::HiDpi;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::windows::WindowBuilderExtWindows;
use winit::window::WindowBuilder;

fn main() {
    unsafe { CoInitializeEx(ptr::null_mut(), COINIT_APARTMENTTHREADED).unwrap() };
    unsafe { HiDpi::SetProcessDpiAwareness(HiDpi::PROCESS_PER_MONITOR_DPI_AWARE).unwrap() };

    let event_loop = EventLoop::new();
    let window = Rc::new(
        WindowBuilder::new()
            .with_no_redirection_bitmap(true)
            .build(&event_loop)
            .unwrap(),
    );

    let compositor = Composition::new(&window).unwrap();
    let webview = WebView::new(window.clone()).unwrap();

    webview.set_message_handler(WebViewHandler);
    webview.set_visual_target(&compositor.root_visual());

    if let Ok(server) = std::env::var("ZENITH_SERVER") {
        webview.navigate_to_url(&server);
    } else {
        webview.navigate_to_string(include_str!("index.html"));
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    webview.close();
                }
                WindowEvent::Resized(_) => {
                    let size = window.inner_size();
                    compositor.set_size(size.width, size.height);
                    webview.set_size(size.width, size.height);
                }
                _ => {}
            },
            Event::UserEvent(_message) => {}
            _ => {}
        }
    });
}

struct WebViewHandler;

#[derive(Deserialize)]
#[serde(tag = "type")]
enum WebViewRequest {
    #[serde(rename = "core.set_server")]
    CoreSetServer { address: String },
}

impl webview::Handler for WebViewHandler {
    fn on_message_received(&mut self, webview: &WebView, message: String) {
        if let Ok(message) = serde_json::from_str::<WebViewRequest>(&message) {
            match message {
                WebViewRequest::CoreSetServer { address } => webview.navigate_to_url(&address),
            }
        } else {
            println!("unrecognised message: {message}");
        }
    }
}
