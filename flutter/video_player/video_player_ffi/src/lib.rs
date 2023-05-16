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
pub unsafe extern "C" fn load(
    player: *const MediaPlayer,
    url: *const i8,
    title: *const i8,
    subtitle: *const i8,
    start_position: f64,
) {
    let url = CStr::from_ptr(url).to_str().unwrap();

    let title = title
        .as_ref()
        .map(|p| CStr::from_ptr(p))
        .and_then(|s| s.to_str().ok());

    let subtitle = subtitle
        .as_ref()
        .map(|p| CStr::from_ptr(p))
        .and_then(|s| s.to_str().ok());

    player
        .as_ref()
        .unwrap()
        .load(url, title, subtitle, start_position);
}

#[no_mangle]
pub unsafe extern "C" fn set_audio_track(player: *const MediaPlayer, index: i32) {
    player.as_ref().unwrap().set_audio_track(index);
}

#[no_mangle]
pub unsafe extern "C" fn set_subtitle_file(player: *const MediaPlayer, url: *const i8) {
    player
        .as_ref()
        .unwrap()
        .set_subtitle_file(if url.is_null() {
            None
        } else {
            Some(CStr::from_ptr(url).to_str().unwrap())
        });
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
pub unsafe extern "C" fn seek_to(player: *const MediaPlayer, position: f64) {
    player.as_ref().unwrap().seek_to(position);
}
