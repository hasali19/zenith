mod composition;
mod ext;
mod player;
mod webview;

use std::ptr;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

use composition::Composition;
use player::{VideoPlayer, VideoPlayerSurface};
use serde::{Deserialize, Serialize};
use webview::WebView;
use windows::core::Result;
use windows::Foundation::Numerics::Vector2;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};
use windows::Win32::UI::HiDpi;
use windows::UI::Composition::ContainerVisual;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};
use winit::platform::windows::WindowBuilderExtWindows;
use winit::window::WindowBuilder;

fn main() -> Result<()> {
    env_logger::init_from_env(
        env_logger::Env::new().default_filter_or("info,zenith_windows=debug"),
    );

    unsafe { CoInitializeEx(ptr::null_mut(), COINIT_APARTMENTTHREADED)? };
    unsafe { HiDpi::SetProcessDpiAwareness(HiDpi::PROCESS_PER_MONITOR_DPI_AWARE)? };

    let event_loop = EventLoop::with_user_event();
    let window = Rc::new(
        WindowBuilder::new()
            .with_no_redirection_bitmap(true)
            .with_inner_size(LogicalSize::new(1280, 800))
            .build(&event_loop)
            .unwrap(),
    );

    let composition = Composition::new(&window)?;
    let webview = WebView::new(window.clone())?;

    let composition_root = composition.root_visual();
    let compositor = composition_root.Compositor()?;

    let video_container = compositor.CreateContainerVisual()?;
    let webview_container = compositor.CreateContainerVisual()?;

    video_container.SetRelativeSizeAdjustment(Vector2 { X: 1.0, Y: 1.0 })?;
    webview_container.SetRelativeSizeAdjustment(Vector2 { X: 1.0, Y: 1.0 })?;

    composition_root.Children()?.InsertAtTop(&video_container)?;
    composition_root
        .Children()?
        .InsertAtTop(&webview_container)?;

    let video_player = Rc::new(parking_lot::RwLock::new(None));

    webview.set_message_handler(WebViewHandler {
        event_loop: event_loop.create_proxy(),
        video_container,
        video_player: video_player.clone(),
    });

    webview.set_visual_target(&webview_container);

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

                    if let Some(video_player) = video_player.write().take() {
                        video_player.stop();
                    }

                    webview.close();
                }
                WindowEvent::Resized(_) => {
                    let size = window.inner_size();

                    composition.set_size(size.width, size.height);
                    webview.set_size(size.width, size.height);

                    if let Some(video_player) = video_player.read().as_ref() {
                        video_player.set_size(size.width, size.height);
                    }
                }
                _ => {}
            },
            Event::UserEvent(message) => {
                let json = serde_json::to_string(&message).unwrap();
                if let Err(e) = webview.send_json_message(&json) {
                    log::warn!("{e}");
                };
            }
            _ => {}
        }
    });
}

struct WebViewHandler {
    event_loop: EventLoopProxy<WebViewMessage>,
    video_container: ContainerVisual,
    video_player: Rc<parking_lot::RwLock<Option<PlayerThread>>>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum WebViewRequest {
    #[serde(rename = "core.set_server")]
    CoreSetServer { address: String },

    #[serde(rename = "player.init")]
    PlayerInit { src: String },

    #[serde(rename = "player.stop")]
    PlayerStop,

    #[serde(rename = "player.set_playing")]
    PlayerSetPlaying { value: bool },

    #[serde(rename = "player.set_position")]
    PlayerSetPosition { position: f32 },
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum WebViewMessage {
    #[serde(rename = "player.duration_changed")]
    PlayerDurationChanged { value: f32 },

    #[serde(rename = "player.is_playing_changed")]
    PlayerIsPlayingChanged { value: bool },

    #[serde(rename = "player.position_changed")]
    PlayerPositionChanged { value: f32 },
}

impl webview::Handler for WebViewHandler {
    fn on_message_received(&mut self, webview: &WebView, message: String) {
        let message = match serde_json::from_str::<WebViewRequest>(&message) {
            Ok(message) => message,
            Err(_) => {
                eprintln!("unrecognised message: {message}");
                return;
            }
        };

        match message {
            WebViewRequest::CoreSetServer { address } => webview.navigate_to_url(&address),
            WebViewRequest::PlayerInit { src } => {
                *self.video_player.write() = Some(PlayerThread::spawn(
                    src,
                    self.video_container.clone(),
                    self.event_loop.clone(),
                ));
            }
            WebViewRequest::PlayerStop => {
                self.video_player.write().take();
            }
            WebViewRequest::PlayerSetPlaying { value } => {
                if let Some(player) = &*self.video_player.read() {
                    player.set_playing(value);
                }
            }
            WebViewRequest::PlayerSetPosition { position } => {
                if let Some(player) = &*self.video_player.read() {
                    player.set_position(position);
                }
            }
        }
    }
}

struct PlayerThread(Sender<PlayerMessage>, JoinHandle<()>);

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
enum PlayerMessage {
    SetPlaying { value: bool },
    SetPosition { position: f32 },
    SetSize { width: u32, height: u32 },
    Stop,
}

impl PlayerThread {
    fn spawn(
        src: String,
        container: ContainerVisual,
        event_loop: EventLoopProxy<WebViewMessage>,
    ) -> PlayerThread {
        let (tx, rx) = std::sync::mpsc::channel();

        let thread = std::thread::spawn(move || {
            log::info!("starting player message thread");

            let mut player = VideoPlayer::new();

            player.add_duration_changed_callback({
                let event_loop = event_loop.clone();
                &mut move |duration| {
                    event_loop
                        .send_event(WebViewMessage::PlayerDurationChanged {
                            value: duration as f32 / 1000.0,
                        })
                        .unwrap();
                }
            });

            player.add_playing_callback({
                let event_loop = event_loop.clone();
                &mut move || {
                    event_loop
                        .send_event(WebViewMessage::PlayerIsPlayingChanged { value: true })
                        .unwrap();
                }
            });

            player.add_paused_callback({
                let event_loop = event_loop.clone();
                &mut move || {
                    event_loop
                        .send_event(WebViewMessage::PlayerIsPlayingChanged { value: false })
                        .unwrap();
                }
            });

            player.add_position_changed_callback({
                let event_loop = event_loop.clone();
                &mut move |position| {
                    event_loop
                        .send_event(WebViewMessage::PlayerPositionChanged {
                            value: position / 1000.0,
                        })
                        .unwrap();
                }
            });

            log::info!("playing media url: {src}");

            player.set_media_url(&src);
            player.play();

            log::info!("creating video surface");

            let mut surface = VideoPlayerSurface::new(1280, 800);

            surface.set_visual_target(&container);
            surface.set_player(&mut player);

            log::info!("waiting for messages");

            for msg in rx {
                match msg {
                    PlayerMessage::SetPlaying { value } => {
                        if value {
                            player.play();
                        } else {
                            player.pause();
                        }
                    }
                    PlayerMessage::SetPosition { position } => {
                        player.seek_to(position * 1000.0);
                    }
                    PlayerMessage::SetSize { width, height } => {
                        surface.set_size(width, height);
                    }
                    PlayerMessage::Stop => break,
                }
            }

            log::info!("destroying player");

            // Remove player surface from composition
            container.Children().unwrap().RemoveAll().unwrap();

            // Destroy player resources
            // TODO: Investigate why dropping order matter here
            drop(player);
            drop(surface);

            log::info!("exiting player message thread");
        });

        PlayerThread(tx, thread)
    }

    fn set_playing(&self, value: bool) {
        self.0.send(PlayerMessage::SetPlaying { value }).unwrap();
    }

    fn set_position(&self, position: f32) {
        self.0
            .send(PlayerMessage::SetPosition { position })
            .unwrap();
    }

    fn set_size(&self, width: u32, height: u32) {
        self.0
            .send(PlayerMessage::SetSize { width, height })
            .unwrap();
    }

    fn stop(self) {
        self.0.send(PlayerMessage::Stop).unwrap();
        self.1.join().unwrap();
    }
}
