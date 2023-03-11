use flutter_windows_sys::{
    FlutterDesktopPluginRegistrarGetMessenger, FlutterDesktopPluginRegistrarGetView,
    FlutterDesktopPluginRegistrarRef,
};

use crate::messenger::FlutterDesktopMessenger;
use crate::view::FlutterDesktopView;

pub struct FlutterDesktopPluginRegistrar {
    _ptr: FlutterDesktopPluginRegistrarRef,
    messenger: FlutterDesktopMessenger,
    view: FlutterDesktopView,
}

impl FlutterDesktopPluginRegistrar {
    #[doc(hidden)]
    pub unsafe fn new(
        registrar: FlutterDesktopPluginRegistrarRef,
    ) -> FlutterDesktopPluginRegistrar {
        let messenger = FlutterDesktopPluginRegistrarGetMessenger(registrar);
        let view = FlutterDesktopPluginRegistrarGetView(registrar);
        FlutterDesktopPluginRegistrar {
            _ptr: registrar,
            messenger: FlutterDesktopMessenger::new(messenger),
            view: FlutterDesktopView::new(view),
        }
    }

    pub fn messenger(&self) -> &FlutterDesktopMessenger {
        &self.messenger
    }

    pub fn view(&self) -> &FlutterDesktopView {
        &self.view
    }
}
