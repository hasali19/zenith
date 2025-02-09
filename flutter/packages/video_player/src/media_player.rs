use std::ffi::CStr;
use std::slice;
use std::sync::atomic::{self, AtomicU64};
use std::sync::{Arc, Mutex};

use mpv_sys::{mpv_format_MPV_FORMAT_NODE_ARRAY, mpv_format_MPV_FORMAT_NODE_MAP, mpv_node};
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
    pub external_subtitles: Vec<ExternalSubtitle>,
}

#[derive(Debug)]
pub struct ExternalSubtitle {
    pub url: String,
    pub title: Option<String>,
    pub language: Option<String>,
}

pub enum MediaTrackType {
    Video,
    Audio,
    Subtitle,
}

pub struct MediaTrack {
    pub id: i64,
    pub track_type: MediaTrackType,
    pub title: Option<String>,
    pub language: Option<String>,
    pub selected: bool,
    pub codec: Option<String>,
}

pub struct SubtitleStyle {
    pub size: u32,
}

pub enum MediaPlayerEvent {
    DurationChanged(f64),
    PauseChanged(bool),
    IdleChanged(bool),
    SpeedChanged(f64),
    PlaylistPosChanged(u32),
    VideoEnded,
    TracksChanged(Vec<MediaTrack>),
    SubTrackChanged(Option<i64>),
    SubStyleChanged(SubtitleStyle),
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
        mpv.observe_property("track-list", MpvFormat::Node, 0);
        mpv.observe_property("current-tracks/sub", MpvFormat::Node, 0);
        mpv.observe_property("sub-font-size", MpvFormat::Int64, 0);

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
                        if let Some(&value) = value {
                            let playlist = self.playlist.lock().unwrap();

                            if let Ok(pos) = usize::try_from(value)
                                && let Some(item) = playlist.get(pos)
                            {
                                self.system_media_controls.update_media_display(
                                    item.title.as_deref(),
                                    item.subtitle.as_deref(),
                                );

                                for sub in &item.external_subtitles {
                                    self.mpv.add_sub_async(
                                        &sub.url,
                                        sub.title.as_deref(),
                                        sub.language.as_deref(),
                                    );
                                }

                                event_handler(
                                    position,
                                    MediaPlayerEvent::PlaylistPosChanged(value as u32),
                                );
                            }
                        }
                    }
                    "track-list" => {
                        let value = unsafe { data.cast::<mpv_node>().as_ref() };
                        if let Some(value) = value {
                            assert_eq!(value.format, mpv_format_MPV_FORMAT_NODE_ARRAY);

                            let list = unsafe { value.u.list.as_ref().unwrap() };
                            let values = if list.num == 0 {
                                &[]
                            } else {
                                unsafe { slice::from_raw_parts(list.values, list.num as usize) }
                            };

                            let mut tracks = vec![];

                            for track in values {
                                assert_eq!(track.format, mpv_format_MPV_FORMAT_NODE_MAP);

                                let map = unsafe { track.u.list.as_ref().unwrap() };

                                let keys =
                                    unsafe { slice::from_raw_parts(map.keys, map.num as usize) };

                                let values =
                                    unsafe { slice::from_raw_parts(map.values, map.num as usize) };

                                let mut id = None;
                                let mut track_type = None;
                                let mut title = None;
                                let mut lang = None;
                                let mut selected = false;
                                let mut codec = None;

                                for (&key, value) in keys.iter().zip(values) {
                                    unsafe {
                                        let key = CStr::from_ptr(key);
                                        if key == c"id" {
                                            id = Some(value.u.int64);
                                        } else if key == c"type" {
                                            track_type = Some(CStr::from_ptr(value.u.string));
                                        } else if key == c"title" {
                                            title = Some(CStr::from_ptr(value.u.string));
                                        } else if key == c"lang" {
                                            lang = Some(CStr::from_ptr(value.u.string));
                                        } else if key == c"selected" {
                                            selected = value.u.flag == 1;
                                        } else if key == c"codec" {
                                            codec = if value.u.string.is_null() {
                                                None
                                            } else {
                                                Some(CStr::from_ptr(value.u.string))
                                            };
                                        }
                                    }
                                }

                                let id = id.unwrap();
                                let track_type = track_type.unwrap();

                                tracks.push(MediaTrack {
                                    id,
                                    track_type: match track_type.to_str() {
                                        Ok("video") => MediaTrackType::Video,
                                        Ok("audio") => MediaTrackType::Audio,
                                        Ok("sub") => MediaTrackType::Subtitle,
                                        _ => unreachable!(),
                                    },
                                    title: title
                                        .and_then(|title| title.to_str().ok())
                                        .map(|title| title.to_owned()),
                                    language: lang
                                        .and_then(|lang| lang.to_str().ok())
                                        .map(|lang| lang.to_owned()),
                                    selected,
                                    codec: codec
                                        .and_then(|codec| codec.to_str().ok())
                                        .map(|codec| codec.to_owned()),
                                });
                            }

                            event_handler(position, MediaPlayerEvent::TracksChanged(tracks));
                        }
                    }
                    "current-tracks/sub" => {
                        let value = unsafe { data.cast::<mpv_node>().as_ref() };
                        let mut id = None;
                        if let Some(track) = value {
                            assert_eq!(track.format, mpv_format_MPV_FORMAT_NODE_MAP);

                            let map = unsafe { track.u.list.as_ref().unwrap() };

                            let keys = unsafe { slice::from_raw_parts(map.keys, map.num as usize) };
                            let values =
                                unsafe { slice::from_raw_parts(map.values, map.num as usize) };

                            for (&key, value) in keys.iter().zip(values) {
                                unsafe {
                                    let key = CStr::from_ptr(key);
                                    if key == c"id" {
                                        id = Some(value.u.int64);
                                    }
                                }
                            }
                        }
                        event_handler(position, MediaPlayerEvent::SubTrackChanged(id));
                    }
                    "sub-font-size" => {
                        let value = unsafe { data.cast::<i64>().as_ref() };
                        if let Some(&value) = value
                            && let Ok(value) = value.try_into()
                        {
                            event_handler(
                                position,
                                MediaPlayerEvent::SubStyleChanged(SubtitleStyle { size: value }),
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

    pub fn set_subtitle_track(&self, id: Option<i64>) {
        if let Some(id) = id {
            self.mpv.set_property_async("sid", id, 0);
        } else {
            self.mpv.set_property_async("sid", c"no", 0);
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

    pub fn set_subtitle_font_size(&self, size: u32) {
        self.mpv.set_property_async("sub-font-size", size, 0);
    }

    pub fn quit(&self) {
        self.mpv.quit();
    }
}
