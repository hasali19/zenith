use std::ffi::CStr;
use std::sync::atomic::{self, AtomicU64};
use std::sync::{Arc, Mutex};

use windows::Media::SystemMediaTransportControlsButton;
use windows::Win32::Foundation::HWND;

use crate::cstr;
use crate::mpv_player::{self, MpvFormat, MpvPlayer, MpvStr};
use crate::system_media_controls::SystemMediaControls;

pub struct MediaPlayer {
    mpv: Arc<MpvPlayer>,
    start_position: AtomicU64,
    playlist: Mutex<Vec<VideoItem>>,
    system_media_controls: SystemMediaControls,
}

#[derive(Debug)]
pub struct VideoItem {
    pub url: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
}

#[allow(clippy::enum_variant_names)]
pub enum MediaPlayerEvent {
    DurationChanged(f64),
    PauseChanged(bool),
    IdleChanged(bool),
    SpeedChanged(f64),
    PlaylistPosChanged(u32),
    VideoEnded,
}

unsafe impl Send for MediaPlayer {}
unsafe impl Sync for MediaPlayer {}

impl MediaPlayer {
    pub fn new(hwnd: HWND) -> MediaPlayer {
        let mpv = Arc::new(MpvPlayer::new());

        mpv.set_property("alang", cstr!("eng,en"));
        mpv.set_property("sid", cstr!("no"));
        mpv.set_property("sub-font-size", 40);

        mpv.observe_property("duration", MpvFormat::Double, 0);
        mpv.observe_property("pause", MpvFormat::Flag, 0);
        mpv.observe_property("core-idle", MpvFormat::Flag, 0);
        mpv.observe_property("speed", MpvFormat::Double, 0);
        mpv.observe_property("playlist-playing-pos", MpvFormat::Int64, 0);

        let mut controls = SystemMediaControls::new(hwnd);

        controls.set_button_handler({
            let mpv = mpv.clone();
            move |event| {
                let button = event.Button().unwrap();

                let pause = match button {
                    SystemMediaTransportControlsButton::Play => false,
                    SystemMediaTransportControlsButton::Pause => true,
                    _ => return Ok(()),
                };

                mpv.set_property("pause", pause);

                Ok(())
            }
        });

        MediaPlayer {
            mpv,
            start_position: AtomicU64::new(0),
            playlist: Mutex::new(Vec::new()),
            system_media_controls: controls,
        }
    }

    pub fn mpv(&self) -> &MpvPlayer {
        &self.mpv
    }

    pub fn run_event_loop(self: &MediaPlayer, event_handler: impl Fn(f64, MediaPlayerEvent)) {
        let mut is_start = false;
        loop {
            let event = self.mpv.wait_event(-1.0);
            let position: f64 = self.mpv.get_property("time-pos");

            match event {
                mpv_player::Event::Shutdown => break,
                mpv_player::Event::StartFile => is_start = true,
                mpv_player::Event::EndFile => {
                    self.system_media_controls.set_stopped();
                    event_handler(position, MediaPlayerEvent::VideoEnded);
                }
                mpv_player::Event::PlaybackRestart => {
                    if is_start {
                        is_start = false;
                        self.system_media_controls.set_playing();
                        self.mpv.seek(f64::from_bits(
                            self.start_position.swap(0, atomic::Ordering::SeqCst),
                        ));
                    }
                }
                mpv_player::Event::PropertyChange { name, data } => match name.to_str().unwrap() {
                    "duration" => {
                        let value = unsafe { data.cast::<f64>().as_ref() };
                        if let Some(duration) = value {
                            event_handler(position, MediaPlayerEvent::DurationChanged(*duration));
                        }
                    }
                    "pause" => {
                        let value = unsafe { data.cast::<bool>().as_ref() };
                        if let Some(paused) = value {
                            self.system_media_controls.set_paused(*paused);
                            event_handler(position, MediaPlayerEvent::PauseChanged(*paused));
                        }
                    }
                    "core-idle" => {
                        let value = unsafe { data.cast::<bool>().as_ref() };
                        if let Some(idle) = value {
                            event_handler(position, MediaPlayerEvent::IdleChanged(*idle));
                        }
                    }
                    "speed" => {
                        let value = unsafe { data.cast::<f64>().as_ref() };
                        if let Some(value) = value {
                            event_handler(position, MediaPlayerEvent::SpeedChanged(*value));
                        }
                    }
                    "playlist-playing-pos" => {
                        let value = unsafe { data.cast::<i64>().as_ref() };
                        if let Some(value) = value && *value > 0 {
                            println!("playlist-playing-pos {value}");

                            let playlist = self.playlist.lock().unwrap();

                            if let Some(item) = playlist.get(*value as usize) {
                                self.system_media_controls.update_media_display(
                                    item.title.as_deref(),
                                    item.subtitle.as_deref(),
                                );
                            }

                            event_handler(
                                position,
                                MediaPlayerEvent::PlaylistPosChanged(*value as u32),
                            );
                        }
                    }
                    _ => {}
                },
                mpv_player::Event::LogMessage { text } => {
                    print!("{}", text.to_str().unwrap());
                }
                _ => {}
            }
        }
    }

    pub fn set_http_headers(&self, headers: &[&CStr]) {
        self.mpv.set_property("http-header-fields", headers);
    }

    pub fn load(&self, items: Vec<VideoItem>, start_index: u32, start_position: f64) {
        self.start_position
            .store(start_position.to_bits(), atomic::Ordering::SeqCst);

        for item in &items {
            self.mpv.load_file(&item.url);
        }

        *self.playlist.lock().unwrap() = items;

        self.mpv.set_property_async("playlist-pos", start_index, 0);
    }

    pub fn set_audio_track(&self, index: i32) {
        let value: MpvStr = self.mpv.get_property(&format!("track-list/{index}/id"));
        self.mpv.set_property_async("aid", value, 0);
    }

    pub fn set_subtitle_file(&self, url: Option<&str>) {
        if let Some(url) = url {
            self.mpv.add_sub_async(url);
        } else {
            self.mpv.set_property_async("sid", cstr!("no"), 0);
        }
    }

    pub fn set_paused(&self, paused: bool) {
        self.mpv.set_property_async("pause", paused, 0);
    }

    pub fn playlist_next(&self) {
        self.mpv.playlist_next();
    }

    pub fn playlist_prev(&self) {
        self.mpv.playlist_prev();
    }

    pub fn seek_to(&self, position: f64) {
        self.mpv.seek(position);
    }

    pub fn set_speed(&self, speed: f64) {
        self.mpv.set_property_async("speed", speed, 0);
    }

    pub fn quit(&self) {
        self.mpv.quit();
    }
}
