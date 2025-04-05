use std::ffi::{CStr, CString, c_void};
use std::ops::Deref;
use std::os::raw::c_char;
use std::ptr;

use mpv_sys::{
    mpv_command_async, mpv_create, mpv_error, mpv_event_id_MPV_EVENT_AUDIO_RECONFIG,
    mpv_event_id_MPV_EVENT_CLIENT_MESSAGE, mpv_event_id_MPV_EVENT_COMMAND_REPLY,
    mpv_event_id_MPV_EVENT_END_FILE, mpv_event_id_MPV_EVENT_FILE_LOADED,
    mpv_event_id_MPV_EVENT_GET_PROPERTY_REPLY, mpv_event_id_MPV_EVENT_HOOK,
    mpv_event_id_MPV_EVENT_IDLE, mpv_event_id_MPV_EVENT_LOG_MESSAGE,
    mpv_event_id_MPV_EVENT_PLAYBACK_RESTART, mpv_event_id_MPV_EVENT_PROPERTY_CHANGE,
    mpv_event_id_MPV_EVENT_QUEUE_OVERFLOW, mpv_event_id_MPV_EVENT_SEEK,
    mpv_event_id_MPV_EVENT_SET_PROPERTY_REPLY, mpv_event_id_MPV_EVENT_SHUTDOWN,
    mpv_event_id_MPV_EVENT_START_FILE, mpv_event_id_MPV_EVENT_TICK,
    mpv_event_id_MPV_EVENT_VIDEO_RECONFIG, mpv_event_log_message, mpv_event_name,
    mpv_event_property, mpv_format, mpv_format_MPV_FORMAT_DOUBLE, mpv_format_MPV_FORMAT_FLAG,
    mpv_format_MPV_FORMAT_INT64, mpv_format_MPV_FORMAT_NODE, mpv_format_MPV_FORMAT_NODE_ARRAY,
    mpv_format_MPV_FORMAT_STRING, mpv_free, mpv_get_property, mpv_handle, mpv_initialize, mpv_node,
    mpv_node__bindgen_ty_1, mpv_node_list, mpv_observe_property, mpv_request_log_messages,
    mpv_set_property, mpv_set_property_async, mpv_terminate_destroy, mpv_wait_event,
};

macro_rules! s {
    ($s:literal) => {{ concat!($s, "\0").as_ptr() as *const u8 as *const i8 }};
}

pub struct MpvPlayer {
    mpv: *mut mpv_handle,
}

unsafe impl Send for MpvPlayer {}
unsafe impl Sync for MpvPlayer {}

impl MpvPlayer {
    pub fn new() -> MpvPlayer {
        let mpv = unsafe {
            let mpv = mpv_create();

            mpv_request_log_messages(mpv, s!("info"));
            mpv_initialize(mpv);

            mpv
        };

        MpvPlayer { mpv }
    }

    pub fn handle(&self) -> *mut mpv_handle {
        self.mpv
    }

    pub fn get_property<T: MpvGetType>(&self, name: &str) -> T {
        let name = CString::new(name).unwrap();

        T::with_ptr(|ptr| unsafe {
            mpv_get_property(self.mpv, name.as_ptr(), T::FORMAT as mpv_format, ptr);
        })
    }

    pub fn set_property<T: MpvSetType>(&self, name: &str, value: T) -> mpv_error {
        let name = CString::new(name).unwrap();
        value.with_ptr(|ptr| unsafe {
            mpv_set_property(self.mpv, name.as_ptr(), T::FORMAT as mpv_format, ptr)
        })
    }

    pub fn set_property_async<T: MpvSetType>(&self, name: &str, value: T, reply_userdata: u64) {
        let name = CString::new(name).unwrap();
        value.with_ptr(|ptr| unsafe {
            mpv_set_property_async(
                self.mpv,
                reply_userdata,
                name.as_ptr(),
                T::FORMAT as mpv_format,
                ptr,
            );
        });
    }

    pub fn observe_property(&self, name: &str, format: MpvFormat, user_data: u64) {
        let name = CString::new(name).unwrap();
        unsafe {
            mpv_observe_property(self.mpv, user_data, name.as_ptr(), format as mpv_format);
        }
    }

    pub fn load_file(&self, url: &str) {
        let url = CString::new(url).unwrap();
        let mut args = [s!("loadfile"), url.as_ptr(), ptr::null()];
        unsafe {
            mpv_command_async(self.mpv, 0, &mut args as *mut *const c_char);
        }
    }

    pub fn add_sub_async(&self, url: &str, title: Option<&str>, lang: Option<&str>) {
        let url = CString::new(url).unwrap();
        let title = title.map(|v| CString::new(v).unwrap());
        let lang = lang.map(|v| CString::new(v).unwrap());
        let mut args = [
            s!("sub-add"),
            url.as_ptr(),
            s!("auto"),
            title.as_ref().map(|v| v.as_ptr()).unwrap_or(ptr::null()),
            lang.as_ref().map(|v| v.as_ptr()).unwrap_or(ptr::null()),
            ptr::null(),
        ];
        unsafe {
            mpv_command_async(self.mpv, 0, &mut args as *mut *const c_char);
        }
    }

    pub fn playlist_next(&self) {
        let mut args = [s!("playlist-next"), ptr::null()];
        unsafe {
            mpv_command_async(self.mpv, 0, &mut args as *mut *const c_char);
        }
    }

    pub fn playlist_prev(&self) {
        let mut args = [s!("playlist-prev"), ptr::null()];
        unsafe {
            mpv_command_async(self.mpv, 0, &mut args as *mut *const c_char);
        }
    }

    pub fn seek(&self, position: f64) {
        let position = CString::new(position.to_string()).unwrap();
        let mut args = [s!("seek"), position.as_ptr(), s!("absolute"), ptr::null()];
        unsafe {
            mpv_command_async(self.mpv, 0, &mut args as *mut *const c_char);
        }
    }

    pub fn quit(&self) {
        let mut args = [s!("quit"), ptr::null()];
        unsafe {
            mpv_command_async(self.mpv, 0, &mut args as *mut *const c_char);
        }
    }

    #[allow(non_upper_case_globals)]
    pub fn wait_event(&self, timeout: f64) -> Event {
        let event = unsafe { mpv_wait_event(self.mpv, timeout) };
        let event_id = unsafe { (*event).event_id };
        match event_id {
            mpv_event_id_MPV_EVENT_SHUTDOWN => Event::Shutdown,
            mpv_event_id_MPV_EVENT_LOG_MESSAGE => {
                let data = unsafe { (*event).data as *mut mpv_event_log_message };
                let text = unsafe { CStr::from_ptr((*data).text) };
                Event::LogMessage { text: MpvStr(text) }
            }
            mpv_event_id_MPV_EVENT_GET_PROPERTY_REPLY => Event::GetPropertyReply,
            mpv_event_id_MPV_EVENT_SET_PROPERTY_REPLY => Event::SetPropertyReply,
            mpv_event_id_MPV_EVENT_COMMAND_REPLY => Event::CommandReply,
            mpv_event_id_MPV_EVENT_START_FILE => Event::StartFile,
            mpv_event_id_MPV_EVENT_END_FILE => Event::EndFile,
            mpv_event_id_MPV_EVENT_FILE_LOADED => Event::FileLoaded,
            mpv_event_id_MPV_EVENT_TICK => Event::Tick,
            mpv_event_id_MPV_EVENT_IDLE => Event::Idle,
            mpv_event_id_MPV_EVENT_CLIENT_MESSAGE => Event::ClientMessage,
            mpv_event_id_MPV_EVENT_VIDEO_RECONFIG => Event::VideoReconfig,
            mpv_event_id_MPV_EVENT_AUDIO_RECONFIG => Event::AudioReconfig,
            mpv_event_id_MPV_EVENT_SEEK => Event::Seek,
            mpv_event_id_MPV_EVENT_PLAYBACK_RESTART => Event::PlaybackRestart,
            mpv_event_id_MPV_EVENT_PROPERTY_CHANGE => {
                let data = unsafe { (*event).data as *mut mpv_event_property };
                let name = unsafe { CStr::from_ptr((*data).name) };
                let data = unsafe { (*data).data };
                Event::PropertyChange { name, data }
            }
            mpv_event_id_MPV_EVENT_QUEUE_OVERFLOW => Event::QueueOverflow,
            mpv_event_id_MPV_EVENT_HOOK => Event::Hook,
            _ => {
                let name = unsafe { CStr::from_ptr(mpv_event_name(event_id)) };
                todo!("{}", name.to_str().unwrap())
            }
        }
    }
}

impl Default for MpvPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for MpvPlayer {
    fn drop(&mut self) {
        unsafe {
            mpv_terminate_destroy(self.mpv);
        }
    }
}

#[repr(i32)]
pub enum MpvFormat {
    Int64 = mpv_format_MPV_FORMAT_INT64,
    Double = mpv_format_MPV_FORMAT_DOUBLE,
    Flag = mpv_format_MPV_FORMAT_FLAG,
    String = mpv_format_MPV_FORMAT_STRING,
    Node = mpv_format_MPV_FORMAT_NODE,
}

#[derive(Debug)]
pub struct MpvStr<'a>(&'a CStr);

impl Deref for MpvStr<'_> {
    type Target = CStr;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl Drop for MpvStr<'_> {
    fn drop(&mut self) {
        unsafe {
            mpv_free(self.0.as_ptr() as *mut c_void);
        }
    }
}

pub trait MpvSetType {
    const FORMAT: MpvFormat;

    fn with_ptr<T, F: FnOnce(*mut c_void) -> T>(self, f: F) -> T;
}

impl MpvSetType for i32 {
    const FORMAT: MpvFormat = MpvFormat::Int64;

    fn with_ptr<T, F: FnOnce(*mut c_void) -> T>(self, f: F) -> T {
        let mut value = self as i64;
        f(&mut value as *mut i64 as *mut c_void)
    }
}

impl MpvSetType for u32 {
    const FORMAT: MpvFormat = MpvFormat::Int64;

    fn with_ptr<T, F: FnOnce(*mut c_void) -> T>(self, f: F) -> T {
        let mut value = self as i64;
        f(&mut value as *mut i64 as *mut c_void)
    }
}

impl MpvSetType for i64 {
    const FORMAT: MpvFormat = MpvFormat::Int64;

    fn with_ptr<T, F: FnOnce(*mut c_void) -> T>(mut self, f: F) -> T {
        f(&mut self as *mut i64 as *mut c_void)
    }
}

impl MpvSetType for isize {
    const FORMAT: MpvFormat = MpvFormat::Int64;

    fn with_ptr<T, F: FnOnce(*mut c_void) -> T>(self, f: F) -> T {
        let mut value = self as i64;
        f(&mut value as *mut i64 as *mut c_void)
    }
}

impl MpvSetType for f64 {
    const FORMAT: MpvFormat = MpvFormat::Double;

    fn with_ptr<T, F: FnOnce(*mut c_void) -> T>(mut self, f: F) -> T {
        f(&mut self as *mut f64 as *mut c_void)
    }
}

impl MpvSetType for bool {
    const FORMAT: MpvFormat = MpvFormat::Flag;

    fn with_ptr<T, F: FnOnce(*mut c_void) -> T>(self, f: F) -> T {
        let mut value = self as i32;
        f(&mut value as *mut i32 as *mut c_void)
    }
}

impl MpvSetType for &CStr {
    const FORMAT: MpvFormat = MpvFormat::String;

    fn with_ptr<T, F: FnOnce(*mut c_void) -> T>(self, f: F) -> T {
        f(&mut self.as_ptr() as *mut *const c_char as *mut c_void)
    }
}

impl MpvSetType for MpvStr<'_> {
    const FORMAT: MpvFormat = MpvFormat::String;

    fn with_ptr<T, F: FnOnce(*mut c_void) -> T>(self, f: F) -> T {
        self.0.with_ptr(f)
    }
}

impl MpvSetType for &[&CStr] {
    const FORMAT: MpvFormat = MpvFormat::Node;

    fn with_ptr<T, F: FnOnce(*mut c_void) -> T>(self, f: F) -> T {
        let mut nodes = vec![mpv_node::default(); self.len()];

        for (&str, node) in self.iter().zip(nodes.iter_mut()) {
            node.format = mpv_format_MPV_FORMAT_STRING;
            node.u = mpv_node__bindgen_ty_1 {
                string: str.as_ptr() as _,
            }
        }

        let mut node_list = mpv_node_list {
            num: self.len() as i32,
            values: nodes.as_mut_ptr(),
            ..Default::default()
        };

        let mut node = mpv_node {
            format: mpv_format_MPV_FORMAT_NODE_ARRAY,
            u: mpv_node__bindgen_ty_1 {
                list: &mut node_list,
            },
        };

        f(&mut node as *mut mpv_node as *mut c_void)
    }
}

pub trait MpvGetType {
    const FORMAT: MpvFormat;

    fn with_ptr<F: FnOnce(*mut c_void)>(f: F) -> Self;
}

impl MpvGetType for f64 {
    const FORMAT: MpvFormat = MpvFormat::Double;

    fn with_ptr<F: FnOnce(*mut c_void)>(f: F) -> Self {
        let mut value = 0f64;
        f(&mut value as *mut f64 as *mut c_void);
        value
    }
}

impl MpvGetType for MpvStr<'_> {
    const FORMAT: MpvFormat = MpvFormat::String;

    fn with_ptr<F: FnOnce(*mut c_void)>(f: F) -> Self {
        let mut value = ptr::null();
        f(&mut value as *mut *const c_char as *mut c_void);
        MpvStr(unsafe { CStr::from_ptr(value) })
    }
}

#[derive(Debug)]
pub enum Event<'a> {
    Shutdown,
    LogMessage { text: MpvStr<'a> },
    GetPropertyReply,
    SetPropertyReply,
    CommandReply,
    StartFile,
    EndFile,
    FileLoaded,
    Idle,
    Tick,
    ClientMessage,
    VideoReconfig,
    AudioReconfig,
    Seek,
    PlaybackRestart,
    PropertyChange { name: &'a CStr, data: *mut c_void },
    QueueOverflow,
    Hook,
}
