#![allow(clippy::missing_safety_doc)]

mod media_player;
mod mpv_player;
mod plugin;
mod system_media_controls;
mod video_surface;

use std::ffi::CStr;

use media_player::MediaPlayer;
use video_surface::VideoSurface;

#[macro_export]
macro_rules! cstr {
    ($s:literal) => {
        unsafe { std::ffi::CStr::from_ptr(concat!($s, "\0").as_ptr().cast()) }
    };
}

#[no_mangle]
pub unsafe extern "C" fn get_texture_id(surface: *const VideoSurface) -> i64 {
    surface.as_ref().unwrap().texture_id()
}

#[no_mangle]
pub unsafe extern "C" fn set_http_headers(
    player: *const MediaPlayer,
    headers: *const *const i8,
    header_count: usize,
) {
    assert!(!headers.is_null());

    let headers = std::slice::from_raw_parts(headers, header_count)
        .iter()
        .map(|header| CStr::from_ptr(*header))
        .collect::<Vec<_>>();

    player.as_ref().unwrap().set_http_headers(&headers);
}

#[repr(C)]
pub struct VideoItem {
    pub url: *const i8,
    pub title: *const i8,
    pub subtitle: *const i8,
    pub external_subtitles_count: usize,
    pub external_subtitles: *const ExternalSubtitle,
}

#[repr(C)]
pub struct ExternalSubtitle {
    pub url: *const i8,
    pub title: *const i8,
    pub language: *const i8,
}

#[no_mangle]
pub unsafe extern "C" fn load(
    player: *const MediaPlayer,
    items: *const VideoItem,
    item_count: usize,
    start_index: u32,
    start_position: f64,
) {
    assert!(!items.is_null());

    let to_owned_string = |ptr| CStr::from_ptr(ptr).to_str().unwrap().to_owned();

    let items = std::slice::from_raw_parts(items, item_count)
        .iter()
        .map(|item| media_player::VideoItem {
            url: to_owned_string(item.url),
            title: item.title.as_ref().map(|p| to_owned_string(p)),
            subtitle: item.subtitle.as_ref().map(|s| to_owned_string(s)),
            external_subtitles: if item.external_subtitles_count == 0 {
                vec![]
            } else {
                std::slice::from_raw_parts(item.external_subtitles, item.external_subtitles_count)
                    .iter()
                    .map(|sub| media_player::ExternalSubtitle {
                        url: to_owned_string(sub.url),
                        language: sub.language.as_ref().map(|s| to_owned_string(s)),
                        title: sub.title.as_ref().map(|s| to_owned_string(s)),
                    })
                    .collect()
            },
        })
        .collect::<Vec<_>>();

    player
        .as_ref()
        .unwrap()
        .load(items, start_index, start_position);
}

#[no_mangle]
pub unsafe extern "C" fn set_audio_track(player: *const MediaPlayer, index: i32) {
    player.as_ref().unwrap().set_audio_track(index);
}

#[no_mangle]
pub unsafe extern "C" fn set_subtitle_track(player: *const MediaPlayer, id: i64) {
    player
        .as_ref()
        .unwrap()
        .set_subtitle_track(if id == -1 { None } else { Some(id) });
}

#[no_mangle]
pub unsafe extern "C" fn pause(player: *const MediaPlayer) {
    player.as_ref().unwrap().set_paused(true);
}

#[no_mangle]
pub unsafe extern "C" fn play(player: *const MediaPlayer) {
    player.as_ref().unwrap().set_paused(false);
}

#[no_mangle]
pub unsafe extern "C" fn playlist_next(player: *const MediaPlayer) {
    player.as_ref().unwrap().playlist_next();
}

#[no_mangle]
pub unsafe extern "C" fn playlist_prev(player: *const MediaPlayer) {
    player.as_ref().unwrap().playlist_prev();
}

#[no_mangle]
pub unsafe extern "C" fn seek_to(player: *const MediaPlayer, position: f64) {
    player.as_ref().unwrap().seek_to(position);
}

#[no_mangle]
pub unsafe extern "C" fn set_speed(player: *const MediaPlayer, speed: f64) {
    player.as_ref().unwrap().set_speed(speed);
}

#[no_mangle]
pub unsafe extern "C" fn set_subtitle_font_size(player: *const MediaPlayer, size: u32) {
    player.as_ref().unwrap().set_subtitle_font_size(size);
}
