#![feature(let_chains)]

mod ffi;
mod media_player;
mod mpv_player;
mod system_media_controls;

use std::collections::BTreeMap;
use std::error::Error;
use std::ffi::c_void;
use std::io::Cursor;
use std::mem;
use std::rc::Rc;

use flion::codec::EncodableValue;
use flion::standard_method_channel::StandardMethodHandler;
use flion::{
    BinaryMessenger, FlionEngineEnvironment, PlatformTask, PlatformView, TaskRunnerExecutor,
    include_plugins,
};
use media_player::{MediaPlayer, MediaPlayerEvent, MediaTrack, MediaTrackType};
use windows::UI::Composition::{Compositor, Visual};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Dwm::{
    DWM_SYSTEMBACKDROP_TYPE, DWMSBT_MAINWINDOW, DWMWA_SYSTEMBACKDROP_TYPE, DwmSetWindowAttribute,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, HTTRANSPARENT, IDC_ARROW, LoadCursorW,
    MoveWindow, RegisterClassExW, SW_SHOW, ShowWindow, WINDOW_EX_STYLE, WM_NCHITTEST, WNDCLASSEXW,
    WS_CHILD, WS_CLIPSIBLINGS,
};
use windows::core::{Interface, w};
use winit::dpi::LogicalSize;
use winit::event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy};
use winit::platform::windows::WindowBuilderExtWindows;
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::WindowBuilder;

include_plugins!();

enum AppEvent {
    EngineTask(PlatformTask),
    UpdateMpvWindow(usize, flion::PlatformViewUpdateArgs),
}

unsafe impl Send for AppEvent {}

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        use tracing_subscriber::fmt::format::FmtSpan;
        tracing_subscriber::fmt()
            .with_span_events(FmtSpan::ENTER)
            .with_thread_names(true)
            .with_max_level(tracing::Level::DEBUG)
            .init();
    }

    let event_loop = EventLoopBuilder::<AppEvent>::with_user_event().build()?;

    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1280, 720))
        .with_no_redirection_bitmap(true)
        .build(&event_loop)?;

    let window = Rc::new(window);

    let hwnd = match window.window_handle()?.as_raw() {
        RawWindowHandle::Win32(handle) => HWND(handle.hwnd.get() as _),
        _ => unreachable!(),
    };

    unsafe {
        let backdrop_type = DWMSBT_MAINWINDOW;
        DwmSetWindowAttribute(
            hwnd,
            DWMWA_SYSTEMBACKDROP_TYPE,
            &raw const backdrop_type as *const c_void,
            mem::size_of::<DWM_SYSTEMBACKDROP_TYPE>() as u32,
        )?;
    }

    let env = FlionEngineEnvironment::init()?;
    let parent_hwnd = hwnd.0 as usize;

    unsafe {
        RegisterClassExW(&WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
            lpfnWndProc: Some(wnd_proc),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            // hbrBackground: mem::transmute::<HGDIOBJ, HBRUSH>(GetStockObject(GRAY_BRUSH)),
            lpszClassName: w!("MpvWindow"),
            ..Default::default()
        })
    };

    unsafe extern "system" fn wnd_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        if msg == WM_NCHITTEST {
            return LRESULT(HTTRANSPARENT as isize);
        }
        unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
    }

    let mut engine = env
        .new_engine_builder()
        .with_plugins(PLUGINS)
        .with_platform_message_handler("zenith.hasali.uk/windowing", Box::new(WindowingPlugin))
        .with_platform_view_factory("video", {
            let event_loop = event_loop.create_proxy();
            move |compositor: &Compositor,
                  _id: i32,
                  args: EncodableValue<'_>|
                  -> eyre::Result<Box<dyn PlatformView>> {
                let player = *args.as_i64().unwrap() as *const MediaPlayer;

                Ok(Box::new(VideoPlayerView::new(
                    unsafe { &*player },
                    compositor,
                    HWND(parent_hwnd as _),
                    event_loop.clone(),
                )?))
            }
        })
        .build(window.clone(), {
            let event_loop = event_loop.create_proxy();
            move |task| {
                if event_loop.send_event(AppEvent::EngineTask(task)).is_err() {
                    tracing::error!("failed to post task to event loop");
                }
            }
        })?;

    engine.set_platform_message_handler(
        "video_player",
        VideoPlayerPlugin {
            window: hwnd,
            messenger: Messenger(engine.messenger()),
        },
    );

    let mut task_executor = TaskRunnerExecutor::default();

    event_loop.run(move |event, target| {
        match event {
            winit::event::Event::UserEvent(event) => match event {
                AppEvent::EngineTask(task) => {
                    task_executor.enqueue(task);
                }
                AppEvent::UpdateMpvWindow(window, args) => unsafe {
                    MoveWindow(
                        HWND(window as _),
                        args.x as i32,
                        args.y as i32,
                        args.width as i32,
                        args.height as i32,
                        false,
                    )
                    .unwrap();
                },
            },

            winit::event::Event::WindowEvent { window_id, event } if window_id == window.id() => {
                if let Err(e) = engine.handle_window_event(&event, target) {
                    tracing::error!("{e:?}");
                }
            }

            _ => {}
        }

        if let Some(next_task_target_time) = engine.process_tasks(&mut task_executor) {
            target.set_control_flow(ControlFlow::WaitUntil(next_task_target_time));
        }
    })?;

    Ok(())
}

struct WindowingPlugin;

impl StandardMethodHandler for WindowingPlugin {
    fn handle(
        &self,
        method: &str,
        _args: flion::codec::EncodableValue,
        reply: flion::standard_method_channel::StandardMethodReply,
    ) {
        match method {
            "isWindowed" => reply.success(&EncodableValue::Bool(true)),
            _ => reply.not_implemented(),
        }
    }
}

#[derive(Clone)]
struct Messenger(BinaryMessenger);

// TODO: This is not right
unsafe impl Send for Messenger {}

struct VideoPlayerPlugin {
    window: HWND,
    messenger: Messenger,
}

impl StandardMethodHandler for VideoPlayerPlugin {
    fn handle(
        &self,
        method: &str,
        args: EncodableValue,
        reply: flion::standard_method_channel::StandardMethodReply,
    ) {
        match method {
            "createPlayer" => {
                let player = Box::new(MediaPlayer::new(self.window));
                let player_ref = player.as_ref() as *const MediaPlayer;

                player.mpv().set_property("vo", c"gpu-next");

                let messenger = self.messenger.clone();

                std::thread::spawn(move || {
                    player.run_event_loop(move |position, event| {
                        let mut res = BTreeMap::new();

                        res.insert(
                            EncodableValue::Str("position"),
                            EncodableValue::F64(position.into()),
                        );

                        match &event {
                            &MediaPlayerEvent::DurationChanged(v) => {
                                res.insert(
                                    EncodableValue::Str("duration"),
                                    EncodableValue::F64(v.into()),
                                );
                            }
                            &MediaPlayerEvent::PauseChanged(v) => {
                                res.insert(EncodableValue::Str("paused"), EncodableValue::Bool(v));
                            }
                            &MediaPlayerEvent::IdleChanged(v) => {
                                res.insert(EncodableValue::Str("idle"), EncodableValue::Bool(v));
                            }
                            &MediaPlayerEvent::VideoEnded => {
                                res.insert(
                                    EncodableValue::Str("state"),
                                    EncodableValue::Str("ended"),
                                );
                            }
                            &MediaPlayerEvent::SpeedChanged(v) => {
                                res.insert(
                                    EncodableValue::Str("speed"),
                                    EncodableValue::F64(v.into()),
                                );
                            }
                            &MediaPlayerEvent::PlaylistPosChanged(v) => {
                                res.insert(
                                    EncodableValue::Str("playlist-pos"),
                                    EncodableValue::I64(v.into()),
                                );
                            }
                            MediaPlayerEvent::TracksChanged(tracks) => {
                                res.insert(
                                    EncodableValue::Str("tracks"),
                                    build_tracks_list(tracks),
                                );
                            }
                            &MediaPlayerEvent::SubTrackChanged(track_id) => {
                                res.insert(
                                    EncodableValue::Str("selected-sub-track"),
                                    track_id
                                        .map(EncodableValue::I64)
                                        .unwrap_or(EncodableValue::Null),
                                );
                            }
                            MediaPlayerEvent::SubStyleChanged(style) => {
                                res.insert(
                                    EncodableValue::Str("subtitle-style"),
                                    EncodableValue::Map({
                                        let mut map = BTreeMap::new();
                                        map.insert(
                                            EncodableValue::Str("size"),
                                            EncodableValue::I64(style.size.into()),
                                        );
                                        map
                                    }),
                                );
                            }
                        }

                        let mut message_bytes = vec![];
                        let mut cursor = Cursor::new(&mut message_bytes);
                        flion::codec::write_value(&mut cursor, &EncodableValue::Str("event"))
                            .unwrap();
                        flion::codec::write_value(&mut cursor, &EncodableValue::Map(res)).unwrap();

                        let messenger = &messenger;

                        messenger
                            .0
                            .send_platform_message(c"video_player", &message_bytes)
                            .unwrap();
                    })
                });

                reply.success(&EncodableValue::I64(player_ref as i64));
            }
            "destroyPlayer" => {
                let args = args.as_map().unwrap();

                let player = *args
                    .get(&EncodableValue::Str("player"))
                    .unwrap()
                    .as_i64()
                    .unwrap();

                let player = unsafe { (player as *const MediaPlayer).as_ref().unwrap() };

                player.quit();

                reply.success(&EncodableValue::Null);
            }
            "getProcs" => {
                macro_rules! proc {
                    ($sym:ident) => {{
                        let p = ffi::$sym as i64;
                        (
                            EncodableValue::Str(stringify!($sym)),
                            EncodableValue::I64(p),
                        )
                    }};
                }

                #[allow(clippy::fn_to_numeric_cast)]
                reply.success(&EncodableValue::Map(BTreeMap::from([
                    (
                        EncodableValue::Str("get_texture_id"),
                        EncodableValue::I64(0),
                    ),
                    proc!(set_http_headers),
                    proc!(load),
                    proc!(set_audio_track),
                    proc!(set_subtitle_track),
                    proc!(pause),
                    proc!(play),
                    proc!(playlist_next),
                    proc!(playlist_prev),
                    proc!(seek_to),
                    proc!(set_speed),
                    proc!(set_subtitle_font_size),
                ])));
            }
            _ => {
                reply.not_implemented();
            }
        }
    }
}

fn build_tracks_list(tracks: &[MediaTrack]) -> EncodableValue<'_> {
    EncodableValue::List(
        tracks
            .iter()
            .map(|track| {
                EncodableValue::Map({
                    let mut map = BTreeMap::new();
                    map.insert(EncodableValue::Str("id"), EncodableValue::I64(track.id));
                    map.insert(
                        EncodableValue::Str("type"),
                        EncodableValue::I64(match track.track_type {
                            MediaTrackType::Video => 1,
                            MediaTrackType::Audio => 2,
                            MediaTrackType::Subtitle => 3,
                        }),
                    );
                    map.insert(
                        EncodableValue::Str("title"),
                        track
                            .title
                            .as_deref()
                            .map(EncodableValue::Str)
                            .unwrap_or(EncodableValue::Null),
                    );
                    map.insert(
                        EncodableValue::Str("lang"),
                        track
                            .language
                            .as_deref()
                            .map(EncodableValue::Str)
                            .unwrap_or(EncodableValue::Null),
                    );
                    map.insert(
                        EncodableValue::Str("selected"),
                        EncodableValue::Bool(track.selected),
                    );
                    map.insert(
                        EncodableValue::Str("codec"),
                        track
                            .codec
                            .as_deref()
                            .map(EncodableValue::Str)
                            .unwrap_or(EncodableValue::Null),
                    );
                    map
                })
            })
            .collect(),
    )
}

struct VideoPlayerView {
    visual: Visual,
    event_loop: EventLoopProxy<AppEvent>,
    window: usize,
}

impl VideoPlayerView {
    pub fn new(
        player: &MediaPlayer,
        compositor: &Compositor,
        parent: HWND,
        event_loop: EventLoopProxy<AppEvent>,
    ) -> eyre::Result<VideoPlayerView> {
        let visual = compositor.CreateSpriteVisual()?;

        let mpv_window = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                w!("MpvWindow"),
                w!("mpv window"),
                WS_CHILD | WS_CLIPSIBLINGS,
                0,
                0,
                300,
                300,
                Some(parent),
                None,
                None,
                None,
            )?
        };

        unsafe {
            let _ = ShowWindow(mpv_window, SW_SHOW);
        }

        player.mpv().set_property("wid", mpv_window.0 as isize);

        Ok(VideoPlayerView {
            visual: visual.cast()?,
            event_loop,
            window: mpv_window.0 as usize,
        })
    }
}

impl PlatformView for VideoPlayerView {
    fn visual(&mut self) -> &windows::UI::Composition::Visual {
        &self.visual
    }

    fn update(&mut self, args: &flion::PlatformViewUpdateArgs) -> eyre::Result<()> {
        let _ = self
            .event_loop
            .send_event(AppEvent::UpdateMpvWindow(self.window, args.clone()));

        Ok(())
    }
}

impl Drop for VideoPlayerView {
    fn drop(&mut self) {
        unsafe {
            DestroyWindow(HWND(self.window as _)).unwrap();
        }
    }
}
