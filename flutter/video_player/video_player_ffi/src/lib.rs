#![allow(clippy::missing_safety_doc)]

mod window;

use std::ffi::{c_void, CStr};
use std::sync::atomic::{self, AtomicBool, AtomicU64};
use std::sync::Arc;

use mpv::{
    mpv_command, mpv_command_async, mpv_create, mpv_event_id_MPV_EVENT_END_FILE,
    mpv_event_id_MPV_EVENT_LOG_MESSAGE, mpv_event_id_MPV_EVENT_PLAYBACK_RESTART,
    mpv_event_id_MPV_EVENT_PROPERTY_CHANGE, mpv_event_id_MPV_EVENT_SHUTDOWN,
    mpv_event_id_MPV_EVENT_START_FILE, mpv_event_log_message, mpv_event_property,
    mpv_format_MPV_FORMAT_DOUBLE, mpv_format_MPV_FORMAT_FLAG, mpv_format_MPV_FORMAT_INT64,
    mpv_format_MPV_FORMAT_STRING, mpv_get_property, mpv_handle, mpv_initialize,
    mpv_observe_property, mpv_request_log_messages, mpv_set_option, mpv_set_property,
    mpv_set_property_string, mpv_terminate_destroy, mpv_wait_event, mpv_wakeup,
};
use windows::Win32::Foundation::HWND;

use dart_sdk::{
    Dart_CObject_Type_Dart_CObject_kArray, Dart_CObject_Type_Dart_CObject_kBool,
    Dart_CObject_Type_Dart_CObject_kDouble, Dart_CObject_Type_Dart_CObject_kInt64,
    Dart_InitializeApiDL, Dart_PostCObject_DL, _Dart_CObject, _Dart_CObject__bindgen_ty_1,
    _Dart_CObject__bindgen_ty_1__bindgen_ty_3,
};

macro_rules! s {
    ($s:literal) => {{
        concat!($s, "\0").as_ptr() as *const u8 as *const i8
    }};
}

pub struct Player {
    hwnd: HWND,
    ctx: *mut mpv_handle,
    start_position: AtomicU64,
    quit: AtomicBool,
}

unsafe impl Send for Player {}
unsafe impl Sync for Player {}

#[no_mangle]
pub unsafe extern "C" fn create_player(
    native_port: i64,
    dart_params: *mut c_void,
) -> *const Player {
    if Dart_InitializeApiDL(dart_params) < 0 {
        panic!("failed to initialize dart api");
    }

    let hwnd = window::create();
    let ctx = mpv_create();

    mpv_set_option(
        ctx,
        s!("wid"),
        mpv_format_MPV_FORMAT_INT64,
        &mut hwnd.clone() as *mut _ as *mut _,
    );

    mpv_request_log_messages(ctx, s!("info"));
    mpv_initialize(ctx);

    mpv_set_property_string(ctx, s!("alang"), s!("eng,en"));
    mpv_set_property_string(ctx, s!("sid"), s!("no"));

    mpv_observe_property(ctx, 0, s!("duration"), mpv_format_MPV_FORMAT_DOUBLE);
    mpv_observe_property(ctx, 0, s!("pause"), mpv_format_MPV_FORMAT_FLAG);
    mpv_observe_property(ctx, 0, s!("core-idle"), mpv_format_MPV_FORMAT_FLAG);

    let player = Arc::new(Player {
        ctx,
        hwnd,
        start_position: AtomicU64::new(0),
        quit: AtomicBool::new(false),
    });

    std::thread::spawn({
        let player = player.clone();
        move || unsafe { run_player_event_loop(player, native_port) }
    });

    Arc::into_raw(player)
}

unsafe fn run_player_event_loop(player: Arc<Player>, native_port: i64) {
    let ctx = player.ctx as *mut mpv_handle;
    let mut is_start = false;
    loop {
        let event = mpv_wait_event(ctx, -1.0);
        if player.quit.load(atomic::Ordering::SeqCst) {
            break;
        }

        let mut position = 0f64;
        unsafe {
            mpv_get_property(
                ctx,
                s!("time-pos"),
                mpv_format_MPV_FORMAT_DOUBLE,
                &mut position as *mut f64 as *mut c_void,
            );
        }

        let event_id = unsafe { (*event).event_id };
        #[allow(non_upper_case_globals)]
        match event_id {
            mpv_event_id_MPV_EVENT_SHUTDOWN => break,
            mpv_event_id_MPV_EVENT_START_FILE => {
                is_start = true;
            }
            mpv_event_id_MPV_EVENT_END_FILE => {
                send_msg(
                    native_port,
                    &PlayerMsg {
                        position,
                        kind: PlayerMsgKind::VideoEnded,
                    },
                );
            }
            mpv_event_id_MPV_EVENT_PLAYBACK_RESTART => {
                if is_start {
                    is_start = false;
                    seek_to(
                        player.as_ref() as *const Player,
                        f64::from_bits(player.start_position.swap(0, atomic::Ordering::SeqCst)),
                    );
                }
            }
            mpv_event_id_MPV_EVENT_PROPERTY_CHANGE => {
                let data = unsafe { (*event).data as *mut mpv_event_property };
                let name = unsafe { CStr::from_ptr((*data).name) };
                if name.to_bytes() == b"duration" {
                    let value = unsafe { ((*data).data as *mut f64).as_ref() };
                    if let Some(duration) = value {
                        send_msg(
                            native_port,
                            &PlayerMsg {
                                position,
                                kind: PlayerMsgKind::DurationChanged(*duration),
                            },
                        );
                    }
                } else if name.to_bytes() == b"pause" {
                    let value = unsafe { ((*data).data as *mut bool).as_ref() };
                    if let Some(paused) = value {
                        send_msg(
                            native_port,
                            &PlayerMsg {
                                position,
                                kind: PlayerMsgKind::PauseChanged(*paused),
                            },
                        );
                    }
                } else if name.to_bytes() == b"core-idle" {
                    let value = unsafe { ((*data).data as *mut bool).as_ref() };
                    if let Some(idle) = value {
                        send_msg(
                            native_port,
                            &PlayerMsg {
                                position,
                                kind: PlayerMsgKind::IdleChanged(*idle),
                            },
                        );
                    }
                }
            }
            mpv_event_id_MPV_EVENT_LOG_MESSAGE => {
                let data = unsafe { (*event).data as *mut mpv_event_log_message };
                print!("{}", CStr::from_ptr((*data).text).to_str().unwrap());
            }
            _ => {}
        }
    }
    mpv_terminate_destroy(player.ctx);
    window::close(player.hwnd);
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

struct PlayerMsg {
    position: f64,
    kind: PlayerMsgKind,
}

#[allow(clippy::enum_variant_names)]
enum PlayerMsgKind {
    DurationChanged(f64),
    PauseChanged(bool),
    IdleChanged(bool),
    VideoEnded,
}

fn send_msg(port: i64, msg: &PlayerMsg) {
    let (kind, mut value) = match msg.kind {
        PlayerMsgKind::DurationChanged(duration) => (1, cobject_f64(duration)),
        PlayerMsgKind::PauseChanged(play_when_ready) => (2, cobject_bool(play_when_ready)),
        PlayerMsgKind::IdleChanged(idle) => (3, cobject_bool(idle)),
        PlayerMsgKind::VideoEnded => (4, cobject_i64(0)),
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
        Dart_PostCObject_DL.unwrap()(port, &mut msg);
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_window_handle(player: *const Player) -> HWND {
    (*player).hwnd
}

#[no_mangle]
pub unsafe extern "C" fn load(player: *const Player, url: *const i8, start_position: f64) {
    (*player)
        .start_position
        .store(start_position.to_bits(), atomic::Ordering::SeqCst);

    mpv_command(
        (*player).ctx,
        &mut [s!("loadfile"), url, std::ptr::null()] as *mut *const i8,
    );
}

#[no_mangle]
pub unsafe extern "C" fn set_subtitle_file(player: *const Player, url: *const i8) {
    if url.is_null() {
        mpv_set_property(
            (*player).ctx,
            s!("sid"),
            mpv_format_MPV_FORMAT_STRING,
            &mut s!("no") as *mut _ as _,
        );
    } else {
        mpv_command_async(
            (*player).ctx,
            0,
            &mut [s!("sub-add"), url, s!("cached"), std::ptr::null()] as *mut *const i8,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn pause(player: *const Player) {
    mpv_set_property(
        (*player).ctx,
        s!("pause"),
        mpv_format_MPV_FORMAT_FLAG,
        &mut 1 as *mut _ as *mut _,
    );
}

#[no_mangle]
pub unsafe extern "C" fn play(player: *const Player) {
    mpv_set_property(
        (*player).ctx,
        s!("pause"),
        mpv_format_MPV_FORMAT_FLAG,
        &mut 0 as *mut i32 as *mut c_void,
    );
}

#[no_mangle]
pub unsafe extern "C" fn seek_to(player: *const Player, position: f64) {
    let position = format!("{}\0", position as i64);
    mpv_command(
        (*player).ctx,
        &mut [
            s!("seek"),
            position.as_ptr() as *const i8,
            s!("absolute"),
            std::ptr::null(),
        ] as *mut *const i8,
    );
}

#[no_mangle]
pub unsafe extern "C" fn destroy_player(player: *const Player) {
    let player = Arc::from_raw(player);
    player.quit.store(true, atomic::Ordering::SeqCst);
    mpv_wakeup(player.ctx);
}
