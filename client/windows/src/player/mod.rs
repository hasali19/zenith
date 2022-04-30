mod surface;

use std::ffi::{c_void, CString};
use std::ptr;

use vlc::{libvlc_instance_t, libvlc_media_player_t};

pub use surface::VideoPlayerSurface;

pub struct VideoPlayer {
    instance: *mut libvlc_instance_t,
    media_player: *mut libvlc_media_player_t,
}

impl VideoPlayer {
    pub fn new() -> VideoPlayer {
        let instance;
        let media_player;

        unsafe {
            instance = vlc::libvlc_new(0, ptr::null());
            media_player = vlc::libvlc_media_player_new(instance);
        }

        VideoPlayer {
            instance,
            media_player,
        }
    }

    pub fn set_media_url(&mut self, url: &str) {
        let url = CString::new(url).unwrap();
        let media = unsafe { vlc::libvlc_media_new_location(self.instance, url.as_ptr()) };

        if media.is_null() {
            panic!("failed to create media");
        }

        unsafe {
            vlc::libvlc_media_player_set_media(self.media_player, media);
            vlc::libvlc_media_release(media);
        }
    }

    pub fn add_duration_changed_callback<F: FnMut(i64)>(&mut self, cb: &mut F) {
        unsafe {
            let event_manager = vlc::libvlc_media_player_event_manager(self.media_player);

            unsafe extern "C" fn callback<F: FnMut(i64)>(
                event: *const vlc::libvlc_event_t,
                userdata: *mut c_void,
            ) {
                let cb = userdata as *mut F;
                (*cb)((*event).u.media_player_length_changed.new_length);
            }

            vlc::libvlc_event_attach(
                event_manager,
                vlc::libvlc_event_e_libvlc_MediaPlayerLengthChanged,
                Some(callback::<F>),
                cb as *mut _ as _,
            );
        }
    }

    pub fn add_playing_callback<F: FnMut()>(&mut self, cb: &mut F) {
        unsafe {
            let event_manager = vlc::libvlc_media_player_event_manager(self.media_player);

            unsafe extern "C" fn callback<F: FnMut()>(
                _: *const vlc::libvlc_event_t,
                userdata: *mut c_void,
            ) {
                let cb = userdata as *mut F;
                (*cb)();
            }

            vlc::libvlc_event_attach(
                event_manager,
                vlc::libvlc_event_e_libvlc_MediaPlayerPlaying,
                Some(callback::<F>),
                cb as *mut _ as _,
            );
        }
    }

    pub fn add_paused_callback<F: FnMut()>(&mut self, cb: &mut F) {
        unsafe {
            let event_manager = vlc::libvlc_media_player_event_manager(self.media_player);

            unsafe extern "C" fn callback<F: FnMut()>(
                _: *const vlc::libvlc_event_t,
                userdata: *mut c_void,
            ) {
                let cb = userdata as *mut F;
                (*cb)();
            }

            vlc::libvlc_event_attach(
                event_manager,
                vlc::libvlc_event_e_libvlc_MediaPlayerPaused,
                Some(callback::<F>),
                cb as *mut _ as _,
            );
        }
    }

    pub fn add_position_changed_callback<F: FnMut(f32)>(&mut self, cb: &mut F) {
        unsafe {
            let event_manager = vlc::libvlc_media_player_event_manager(self.media_player);

            unsafe extern "C" fn callback<F: FnMut(f32)>(
                event: *const vlc::libvlc_event_t,
                userdata: *mut c_void,
            ) {
                let cb = userdata as *mut F;
                let length = vlc::libvlc_media_player_get_length((*event).p_obj as _);
                (*cb)((*event).u.media_player_position_changed.new_position * length as f32);
            }

            vlc::libvlc_event_attach(
                event_manager,
                vlc::libvlc_event_e_libvlc_MediaPlayerPositionChanged,
                Some(callback::<F>),
                cb as *mut _ as _,
            );
        }
    }

    pub fn play(&mut self) {
        unsafe {
            vlc::libvlc_media_player_play(self.media_player);
        }
    }

    pub fn pause(&mut self) {
        unsafe {
            vlc::libvlc_media_player_pause(self.media_player);
        }
    }

    pub fn seek_to(&mut self, position: f32) {
        unsafe {
            let length = vlc::libvlc_media_player_get_length(self.media_player) as f32;
            let pc = if length > 0.0 { position / length } else { 0.0 };
            vlc::libvlc_media_player_set_position(self.media_player, pc, true);
        }
    }
}

impl Drop for VideoPlayer {
    fn drop(&mut self) {
        unsafe {
            vlc::libvlc_media_player_release(self.media_player);
            vlc::libvlc_release(self.instance);
        }
    }
}
