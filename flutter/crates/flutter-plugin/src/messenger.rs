use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ffi::{c_void, CStr, CString};
use std::io::Cursor;

use flutter_codec::EncodableValue;
use flutter_windows_sys::{
    FlutterDesktopMessage, FlutterDesktopMessageResponseHandle, FlutterDesktopMessengerRef,
    FlutterDesktopMessengerSendResponse, FlutterDesktopMessengerSetCallback,
};

pub type FlutterDesktopMessengerCallback =
    Box<dyn Fn(&str, EncodableValue, FlutterDesktopMessengerReply)>;

pub struct FlutterDesktopMessenger {
    ptr: FlutterDesktopMessengerRef,
    callbacks: RefCell<BTreeMap<String, FlutterDesktopMessengerCallback>>,
}

impl FlutterDesktopMessenger {
    pub(crate) fn new(messenger: FlutterDesktopMessengerRef) -> FlutterDesktopMessenger {
        FlutterDesktopMessenger {
            ptr: messenger,
            callbacks: RefCell::new(BTreeMap::new()),
        }
    }

    pub fn set_callback(
        &self,
        channel: &str,
        callback: impl Fn(&str, EncodableValue, FlutterDesktopMessengerReply) + 'static,
    ) {
        self.set_callback_impl(channel, Box::new(callback))
    }

    fn set_callback_impl(&self, channel: &str, callback: FlutterDesktopMessengerCallback) {
        let mut callbacks = self.callbacks.borrow_mut();
        callbacks.insert(channel.to_owned(), Box::new(callback));

        let callback = callbacks.get(channel).unwrap();
        let channel = CString::new(channel).unwrap();

        unsafe {
            FlutterDesktopMessengerSetCallback(
                self.ptr,
                channel.as_ptr(),
                Some(messenger_callback),
                callback as *const _ as _,
            )
        };
    }
}

unsafe extern "C" fn messenger_callback(
    messenger: FlutterDesktopMessengerRef,
    message: *const FlutterDesktopMessage,
    user_data: *mut c_void,
) {
    let callback =
        user_data as *const Box<dyn Fn(&str, EncodableValue, FlutterDesktopMessengerReply)>;
    let callback = callback.as_ref().unwrap();

    let buf = std::slice::from_raw_parts((*message).message, (*message).message_size);
    let mut cursor = Cursor::new(buf);

    let method_name = flutter_codec::read_value(&mut cursor).unwrap();
    let method_args = flutter_codec::read_value(&mut cursor).unwrap();

    let EncodableValue::String(method_name) = method_name else {
        let channel = CStr::from_ptr((*message).channel);
        let channel = channel.to_str().unwrap_or("invalid_channel");
        eprintln!("[method_call({channel})] invalid method name: {method_name:?}");
        return;
    };

    let response_handle = (*message).response_handle;
    let reply = FlutterDesktopMessengerReply {
        messenger,
        response_handle,
    };

    callback(method_name, method_args, reply);
}

pub struct FlutterDesktopMessengerReply {
    messenger: FlutterDesktopMessengerRef,
    response_handle: *const FlutterDesktopMessageResponseHandle,
}

impl FlutterDesktopMessengerReply {
    pub fn success(&self, value: &EncodableValue) {
        let mut bytes = vec![0];
        flutter_codec::write_value(&mut bytes, value).unwrap();
        unsafe {
            FlutterDesktopMessengerSendResponse(
                self.messenger,
                self.response_handle,
                bytes.as_ptr(),
                bytes.len(),
            );
        }
    }

    pub fn not_implemented(&self) {
        unsafe {
            FlutterDesktopMessengerSendResponse(
                self.messenger,
                self.response_handle,
                std::ptr::null(),
                0,
            );
        }
    }
}
