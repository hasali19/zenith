#![allow(clippy::missing_safety_doc)]

mod media_player;
mod mpv_player;
mod plugin;
mod system_media_controls;
mod window;

use std::ffi::{c_void, CStr};

use media_player::{MediaPlayer, MediaPlayerEvent};
use windows::Win32::Foundation::HWND;

use dart_sdk_sys::{
    Dart_CObject_Type_Dart_CObject_kArray, Dart_CObject_Type_Dart_CObject_kBool,
    Dart_CObject_Type_Dart_CObject_kDouble, Dart_CObject_Type_Dart_CObject_kInt64,
    Dart_InitializeApiDL, Dart_PostCObject_DL, _Dart_CObject, _Dart_CObject__bindgen_ty_1,
    _Dart_CObject__bindgen_ty_1__bindgen_ty_3,
};

#[macro_export]
macro_rules! cstr {
    ($s:literal) => {
        unsafe { std::ffi::CStr::from_ptr(concat!($s, "\0").as_ptr().cast()) }
    };
}

#[no_mangle]
pub unsafe extern "C" fn create_player(
    native_port: DartNativePort,
    dart_params: *mut c_void,
) -> *const MediaPlayer {
    if Dart_InitializeApiDL(dart_params) < 0 {
        panic!("failed to initialize dart api");
    }

    let player = Box::new(MediaPlayer::new());
    let player_ref = player.as_ref() as *const MediaPlayer;

    std::thread::spawn({
        move || {
            player.run_event_loop(|position, event| {
                native_port.send_msg(&PlayerEventWithPosition { position, event });
            });
        }
    });

    player_ref
}

#[no_mangle]
pub unsafe extern "C" fn get_window_handle(player: *const MediaPlayer) -> HWND {
    player.as_ref().unwrap().hwnd()
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

#[no_mangle]
pub unsafe extern "C" fn destroy_player(player: *const MediaPlayer) {
    println!("destroying player");
    player.as_ref().unwrap().quit();
}

#[repr(C)]
pub struct DartNativePort(i64);

struct PlayerEventWithPosition {
    position: f64,
    event: MediaPlayerEvent,
}

impl DartNativePort {
    fn send_msg(&self, msg: &PlayerEventWithPosition) {
        let (kind, mut value) = match msg.event {
            MediaPlayerEvent::DurationChanged(duration) => (1, cobject_f64(duration)),
            MediaPlayerEvent::PauseChanged(play_when_ready) => (2, cobject_bool(play_when_ready)),
            MediaPlayerEvent::IdleChanged(idle) => (3, cobject_bool(idle)),
            MediaPlayerEvent::VideoEnded => (4, cobject_i64(0)),
        };

        let mut values = [
            &mut cobject_f64(msg.position) as *mut _Dart_CObject,
            &mut cobject_i64(kind) as *mut _Dart_CObject,
            &mut value as *mut _Dart_CObject,
        ];

        let mut msg = _Dart_CObject {
            type_: Dart_CObject_Type_Dart_CObject_kArray,
            value: _Dart_CObject__bindgen_ty_1 {
                as_array: _Dart_CObject__bindgen_ty_1__bindgen_ty_3 {
                    length: values.len() as isize,
                    values: &mut values as *mut *mut _Dart_CObject,
                },
            },
        };

        unsafe {
            Dart_PostCObject_DL.unwrap()(self.0, &mut msg);
        }
    }
}

fn cobject_i64(value: i64) -> _Dart_CObject {
    _Dart_CObject {
        type_: Dart_CObject_Type_Dart_CObject_kInt64,
        value: _Dart_CObject__bindgen_ty_1 { as_int64: value },
    }
}

fn cobject_f64(value: f64) -> _Dart_CObject {
    _Dart_CObject {
        type_: Dart_CObject_Type_Dart_CObject_kDouble,
        value: _Dart_CObject__bindgen_ty_1 { as_double: value },
    }
}

fn cobject_bool(value: bool) -> _Dart_CObject {
    _Dart_CObject {
        type_: Dart_CObject_Type_Dart_CObject_kBool,
        value: _Dart_CObject__bindgen_ty_1 { as_bool: value },
    }
}
