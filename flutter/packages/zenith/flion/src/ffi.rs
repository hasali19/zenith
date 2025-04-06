#![allow(clippy::missing_safety_doc)]

use std::ffi::CStr;

use crate::media_player::{self, MediaPlayer};

#[macro_export]
macro_rules! cstr {
    ($s:literal) => {
        unsafe { std::ffi::CStr::from_ptr(concat!($s, "\0").as_ptr().cast()) }
    };
}

pub unsafe extern "C" fn set_http_headers(
    player: *const MediaPlayer,
    headers: *const *const i8,
    header_count: usize,
) {
    unsafe {
        assert!(!headers.is_null());

        let headers = std::slice::from_raw_parts(headers, header_count)
            .iter()
            .map(|header| CStr::from_ptr(*header))
            .collect::<Vec<_>>();

        player.as_ref().unwrap().set_http_headers(&headers);
    }
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

pub unsafe extern "C" fn load(
    player: *const MediaPlayer,
    items: *const VideoItem,
    item_count: usize,
    start_index: u32,
    start_position: f64,
) {
    unsafe {
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
                    std::slice::from_raw_parts(
                        item.external_subtitles,
                        item.external_subtitles_count,
                    )
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
}

pub unsafe extern "C" fn set_audio_track(player: *const MediaPlayer, index: i32) {
    unsafe {
        player.as_ref().unwrap().set_audio_track(index);
    }
}

pub unsafe extern "C" fn set_subtitle_track(player: *const MediaPlayer, id: i64) {
    unsafe {
        player
            .as_ref()
            .unwrap()
            .set_subtitle_track(if id == -1 { None } else { Some(id) });
    }
}

pub unsafe extern "C" fn pause(player: *const MediaPlayer) {
    unsafe {
        player.as_ref().unwrap().set_paused(true);
    }
}

pub unsafe extern "C" fn play(player: *const MediaPlayer) {
    unsafe {
        player.as_ref().unwrap().set_paused(false);
    }
}

pub unsafe extern "C" fn playlist_next(player: *const MediaPlayer) {
    unsafe {
        player.as_ref().unwrap().playlist_next();
    }
}

pub unsafe extern "C" fn playlist_prev(player: *const MediaPlayer) {
    unsafe {
        player.as_ref().unwrap().playlist_prev();
    }
}

pub unsafe extern "C" fn seek_to(player: *const MediaPlayer, position: f64) {
    unsafe {
        player.as_ref().unwrap().seek_to(position);
    }
}

pub unsafe extern "C" fn set_speed(player: *const MediaPlayer, speed: f64) {
    unsafe {
        player.as_ref().unwrap().set_speed(speed);
    }
}

pub unsafe extern "C" fn set_subtitle_font_size(player: *const MediaPlayer, size: u32) {
    unsafe {
        player.as_ref().unwrap().set_subtitle_font_size(size);
    }
}
