use webview2_com::Microsoft::Web::WebView2::Win32::{
    CreateCoreWebView2EnvironmentWithOptions, ICoreWebView2, ICoreWebView2CompositionController,
    ICoreWebView2Controller2, ICoreWebView2Environment, ICoreWebView2Environment3,
    ICoreWebView2EnvironmentOptions, COREWEBVIEW2_COLOR,
    COREWEBVIEW2_MOUSE_EVENT_KIND_HORIZONTAL_WHEEL,
    COREWEBVIEW2_MOUSE_EVENT_KIND_LEFT_BUTTON_DOUBLE_CLICK,
    COREWEBVIEW2_MOUSE_EVENT_KIND_LEFT_BUTTON_DOWN, COREWEBVIEW2_MOUSE_EVENT_KIND_LEFT_BUTTON_UP,
    COREWEBVIEW2_MOUSE_EVENT_KIND_MIDDLE_BUTTON_DOUBLE_CLICK,
    COREWEBVIEW2_MOUSE_EVENT_KIND_MIDDLE_BUTTON_DOWN,
    COREWEBVIEW2_MOUSE_EVENT_KIND_MIDDLE_BUTTON_UP, COREWEBVIEW2_MOUSE_EVENT_KIND_MOVE,
    COREWEBVIEW2_MOUSE_EVENT_KIND_RIGHT_BUTTON_DOUBLE_CLICK,
    COREWEBVIEW2_MOUSE_EVENT_KIND_RIGHT_BUTTON_DOWN, COREWEBVIEW2_MOUSE_EVENT_KIND_RIGHT_BUTTON_UP,
    COREWEBVIEW2_MOUSE_EVENT_KIND_WHEEL, COREWEBVIEW2_MOUSE_EVENT_KIND_X_BUTTON_DOUBLE_CLICK,
    COREWEBVIEW2_MOUSE_EVENT_KIND_X_BUTTON_DOWN, COREWEBVIEW2_MOUSE_EVENT_KIND_X_BUTTON_UP,
    COREWEBVIEW2_POINTER_EVENT_KIND,
};
use webview2_com::{
    CoreWebView2EnvironmentOptions, CreateCoreWebView2CompositionControllerCompletedHandler,
    CreateCoreWebView2EnvironmentCompletedHandler, CursorChangedEventHandler,
};
use widestring::U16CStr;
use windows::core::Interface;
use windows::Win32::Foundation::{LPARAM, POINT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::MapWindowPoints;
use windows::Win32::System::WinRT::EventRegistrationToken;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetCapture, ReleaseCapture, SetCapture};
use windows::Win32::UI::Input::Pointer::{
    GetPointerInfo, GetPointerPenInfo, GetPointerTouchInfo, POINTER_INFO, POINTER_PEN_INFO,
    POINTER_TOUCH_INFO,
};
use windows::Win32::UI::WindowsAndMessaging::{SetClassLongPtrA, GCLP_HCURSOR, PT_PEN, PT_TOUCH};
use windows::UI::Composition::ContainerVisual;

use crate::window::{
    self, ButtonAction, MouseButtonEvent, MouseMoveEvent, MouseWheelEvent, PointerEvent, Window,
};

#[derive(Clone)]
pub struct WebView {
    environment: ICoreWebView2Environment3,
    controller: ICoreWebView2Controller2,
    composition_controller: ICoreWebView2CompositionController,
}

impl WebView {
    pub fn new(window: &Window) -> WebView {
        let hwnd = window.hwnd();
        let environment: ICoreWebView2Environment = {
            let (tx, rx) = std::sync::mpsc::channel();

            // CoCreateInstance(ICoreWebView2EnvironmentOptions::IID, punkouter, dwclscontext)
            let options =
                ICoreWebView2EnvironmentOptions::from(CoreWebView2EnvironmentOptions::default());

            unsafe {
                options.SetAdditionalBrowserArguments("--flag-switches-begin --enable-features=msOverlayScrollbarWinStyleMasterFlag,msVisualRejuvMaterialsTitleBar --flag-switches-end").unwrap();
            }

            CreateCoreWebView2EnvironmentCompletedHandler::wait_for_async_operation(
                Box::new(|environmentcreatedhandler| unsafe {
                    CreateCoreWebView2EnvironmentWithOptions(
                        None,
                        None,
                        options,
                        environmentcreatedhandler,
                    )
                    .map_err(webview2_com::Error::WindowsError)
                }),
                Box::new(move |error_code, environment| {
                    error_code?;
                    tx.send(environment.ok_or_else(|| {
                        windows::core::Error::fast_error(windows::Win32::Foundation::E_POINTER)
                    }))
                    .expect("send over mpsc channel");
                    Ok(())
                }),
            )
            .unwrap();

            rx.recv().unwrap().unwrap()
        };

        let environment = environment.cast::<ICoreWebView2Environment3>().unwrap();

        let composition_controller: ICoreWebView2CompositionController = {
            let (tx, rx) = std::sync::mpsc::channel();

            CreateCoreWebView2CompositionControllerCompletedHandler::wait_for_async_operation(
                Box::new({
                    let environment = environment.clone();
                    move |handler| unsafe {
                        environment
                            .CreateCoreWebView2CompositionController(hwnd, handler)
                            .map_err(webview2_com::Error::WindowsError)
                    }
                }),
                Box::new(move |error_code, controller| {
                    error_code?;
                    tx.send(controller.ok_or_else(|| {
                        windows::core::Error::fast_error(windows::Win32::Foundation::E_POINTER)
                    }))
                    .expect("send over mpsc channel");
                    Ok(())
                }),
            )
            .unwrap();

            rx.recv().unwrap().unwrap()
        };

        unsafe {
            let mut token = Default::default();
            composition_controller
                .CursorChanged(
                    CursorChangedEventHandler::create(Box::new(
                        move |controller: Option<ICoreWebView2CompositionController>, _| {
                            let mut cursor = Default::default();
                            controller.unwrap().Cursor(&mut cursor).unwrap();
                            SetClassLongPtrA(hwnd, GCLP_HCURSOR, cursor.0 as _);
                            Ok(())
                        },
                    )),
                    &mut token,
                )
                .unwrap();
        }

        let controller = composition_controller
            .cast::<ICoreWebView2Controller2>()
            .unwrap();

        unsafe {
            // Make the webview transparent
            controller
                .SetDefaultBackgroundColor(COREWEBVIEW2_COLOR {
                    R: 0,
                    G: 0,
                    B: 0,
                    A: 0,
                })
                .unwrap();
        }

        let webview = WebView {
            environment,
            controller,
            composition_controller,
        };

        let (width, height) = window.inner_size();

        webview.set_size(width, height);
        webview.set_visible(true);

        // let webview = controller.CoreWebView2().unwrap();
        // webview.NavigateToString("<!doctype html><html><head><title>test</title><style>html { background-color: rgba(0,255,0,0.5); }</style></head><body><h1 style=\"color: white\">Hello, world!</h1></body></html>").unwrap();

        webview
    }

    pub fn set_size(&self, width: u32, height: u32) {
        unsafe {
            self.controller
                .SetBounds(windows::Win32::Foundation::RECT {
                    left: 0,
                    top: 0,
                    right: width as i32,
                    bottom: height as i32,
                })
                .unwrap();
        }
    }

    pub fn set_visible(&self, visible: bool) {
        unsafe {
            self.controller.SetIsVisible(visible).unwrap();
        }
    }

    pub fn set_visual_target(&self, visual: &ContainerVisual) {
        unsafe {
            self.composition_controller
                .SetRootVisualTarget(visual)
                .unwrap();
        }
    }

    pub fn set_message_handler(
        &self,
        mut message_handler: impl FnMut(&WebView, String) + 'static,
    ) -> impl Drop {
        struct Registration(ICoreWebView2, EventRegistrationToken);

        impl Drop for Registration {
            fn drop(&mut self) {
                unsafe {
                    self.0.RemoveWebMessageReceived(self.1).unwrap();
                }
            }
        }

        let mut token = Default::default();
        unsafe {
            let webview = self.controller.CoreWebView2().unwrap();
            let this = self.clone();
            self.controller
                .CoreWebView2()
                .unwrap()
                .WebMessageReceived(
                    webview2_com::WebMessageReceivedEventHandler::create(Box::new(
                        move |_, args| {
                            let args = args.unwrap();
                            let mut message = Default::default();
                            args.TryGetWebMessageAsString(&mut message)?;
                            let message = U16CStr::from_ptr_str(message.0).to_string().unwrap();
                            message_handler(&this, message);
                            Ok(())
                        },
                    )),
                    &mut token,
                )
                .unwrap();

            Registration(webview, token)
        }
    }

    pub fn navigate_to_url(&self, url: &str) {
        unsafe {
            self.controller
                .CoreWebView2()
                .unwrap()
                .Navigate(url)
                .unwrap();
        }
    }

    #[allow(unused)]
    pub fn navigate_to_string(&self, html: &str) {
        unsafe {
            self.controller
                .CoreWebView2()
                .unwrap()
                .NavigateToString(html)
                .unwrap();
        }
    }

    pub fn send_mouse_move_event(&self, _window: &Window, event: MouseMoveEvent) {
        let position = POINT {
            x: event.position.0,
            y: event.position.1,
        };

        unsafe {
            self.composition_controller
                .SendMouseInput(
                    COREWEBVIEW2_MOUSE_EVENT_KIND_MOVE,
                    event.virtual_keys.bits(),
                    0,
                    position,
                )
                .unwrap();
        }
    }

    pub fn send_mouse_button_event(&self, window: &Window, event: MouseButtonEvent) {
        let hwnd = window.hwnd();

        match event.action {
            ButtonAction::Down => unsafe {
                if GetCapture() != hwnd {
                    SetCapture(hwnd);
                }
            },
            ButtonAction::Up => unsafe {
                if GetCapture() == hwnd {
                    ReleaseCapture();
                }
            },
            ButtonAction::DoubleClick => {}
        }

        let mut mousedata = 0;
        let kind = match (event.button, event.action) {
            (window::MouseButton::Left, ButtonAction::Down) => {
                COREWEBVIEW2_MOUSE_EVENT_KIND_LEFT_BUTTON_DOWN
            }
            (window::MouseButton::Left, ButtonAction::Up) => {
                COREWEBVIEW2_MOUSE_EVENT_KIND_LEFT_BUTTON_UP
            }
            (window::MouseButton::Left, ButtonAction::DoubleClick) => {
                COREWEBVIEW2_MOUSE_EVENT_KIND_LEFT_BUTTON_DOUBLE_CLICK
            }
            (window::MouseButton::Right, ButtonAction::Down) => {
                COREWEBVIEW2_MOUSE_EVENT_KIND_RIGHT_BUTTON_DOWN
            }
            (window::MouseButton::Right, ButtonAction::Up) => {
                COREWEBVIEW2_MOUSE_EVENT_KIND_RIGHT_BUTTON_UP
            }
            (window::MouseButton::Right, ButtonAction::DoubleClick) => {
                COREWEBVIEW2_MOUSE_EVENT_KIND_RIGHT_BUTTON_DOUBLE_CLICK
            }
            (window::MouseButton::Middle, ButtonAction::Down) => {
                COREWEBVIEW2_MOUSE_EVENT_KIND_MIDDLE_BUTTON_DOWN
            }
            (window::MouseButton::Middle, ButtonAction::Up) => {
                COREWEBVIEW2_MOUSE_EVENT_KIND_MIDDLE_BUTTON_UP
            }
            (window::MouseButton::Middle, ButtonAction::DoubleClick) => {
                COREWEBVIEW2_MOUSE_EVENT_KIND_MIDDLE_BUTTON_DOUBLE_CLICK
            }
            (window::MouseButton::X(v), ButtonAction::Down) => {
                mousedata = v;
                COREWEBVIEW2_MOUSE_EVENT_KIND_X_BUTTON_DOWN
            }
            (window::MouseButton::X(v), ButtonAction::Up) => {
                mousedata = v;
                COREWEBVIEW2_MOUSE_EVENT_KIND_X_BUTTON_UP
            }
            (window::MouseButton::X(v), ButtonAction::DoubleClick) => {
                mousedata = v;
                COREWEBVIEW2_MOUSE_EVENT_KIND_X_BUTTON_DOUBLE_CLICK
            }
        };

        let position = POINT {
            x: event.position.0,
            y: event.position.1,
        };

        unsafe {
            self.composition_controller
                .SendMouseInput(kind, event.virtual_keys.bits(), mousedata, position)
                .unwrap();
        }
    }

    pub fn send_mouse_wheel_event(&self, _window: &Window, event: MouseWheelEvent) {
        let kind = match event.wheel {
            window::MouseWheel::Horizontal => COREWEBVIEW2_MOUSE_EVENT_KIND_HORIZONTAL_WHEEL,
            window::MouseWheel::Vertical => COREWEBVIEW2_MOUSE_EVENT_KIND_WHEEL,
        };

        let position = POINT {
            x: event.position.0,
            y: event.position.1,
        };

        unsafe {
            self.composition_controller
                .SendMouseInput(kind, event.virtual_keys.bits(), event.delta, position)
                .unwrap();
        }
    }

    pub fn send_pointer_event(&self, window: &Window, event: PointerEvent) -> bool {
        let mut ptrinfo = POINTER_INFO::default();
        unsafe {
            GetPointerInfo(event.wparam.low_word(), &mut ptrinfo)
                .ok()
                .unwrap();
        }

        if ptrinfo.pointerType != PT_TOUCH && ptrinfo.pointerType != PT_PEN {
            return false;
        }

        let info = unsafe { self.environment.CreateCoreWebView2PointerInfo() }.unwrap();

        unsafe {
            let mut pixel_loc = ptrinfo.ptPixelLocation;
            let mut pixel_loc_raw = ptrinfo.ptPixelLocationRaw;

            MapWindowPoints(None, window.hwnd(), &mut pixel_loc, 1);
            MapWindowPoints(None, window.hwnd(), &mut pixel_loc_raw, 1);

            info.SetPointerKind(ptrinfo.pointerType as u32).unwrap();
            info.SetPointerId(ptrinfo.pointerId).unwrap();
            info.SetFrameId(ptrinfo.frameId).unwrap();
            info.SetPointerFlags(ptrinfo.pointerFlags).unwrap();
            info.SetPixelLocation(pixel_loc).unwrap();
            info.SetPixelLocationRaw(pixel_loc_raw).unwrap();
            info.SetTime(ptrinfo.dwTime).unwrap();
            info.SetHistoryCount(ptrinfo.historyCount).unwrap();
            info.SetButtonChangeKind(ptrinfo.ButtonChangeType).unwrap();
        }

        if ptrinfo.pointerType == PT_TOUCH {
            let mut touchinfo = POINTER_TOUCH_INFO::default();
            unsafe {
                GetPointerTouchInfo(event.wparam.low_word(), &mut touchinfo)
                    .ok()
                    .unwrap();

                let mut contact = touchinfo.rcContact;
                let mut contact_raw = touchinfo.rcContactRaw;

                MapWindowPoints(None, window.hwnd(), &mut contact as *mut _ as _, 2);
                MapWindowPoints(None, window.hwnd(), &mut contact_raw as *mut _ as _, 2);

                info.SetTouchFlags(touchinfo.touchFlags).unwrap();
                info.SetTouchMask(touchinfo.touchMask).unwrap();
                info.SetTouchContact(contact).unwrap();
                info.SetTouchContactRaw(contact_raw).unwrap();
                info.SetTouchOrientation(touchinfo.orientation).unwrap();
                info.SetTouchPressure(touchinfo.pressure).unwrap();
            }
        } else if ptrinfo.pointerType == PT_PEN {
            let mut peninfo = POINTER_PEN_INFO::default();
            unsafe {
                GetPointerPenInfo(event.wparam.low_word(), &mut peninfo)
                    .ok()
                    .unwrap();

                info.SetPenFlags(peninfo.penFlags).unwrap();
                info.SetPenMask(peninfo.penMask).unwrap();
                info.SetPenPressure(peninfo.pressure).unwrap();
                info.SetPenRotation(peninfo.rotation).unwrap();
                info.SetPenTiltX(peninfo.tiltX).unwrap();
                info.SetPenTiltY(peninfo.tiltY).unwrap();
            }
        }

        unsafe {
            self.composition_controller
                .SendPointerInput(event.msg as COREWEBVIEW2_POINTER_EVENT_KIND, info)
                .unwrap();
        }

        true
    }
}

trait WPARAMExt {
    fn high_word(&self) -> u32;
    fn low_word(&self) -> u32;
}

impl WPARAMExt for WPARAM {
    fn high_word(&self) -> u32 {
        ((self.0 & 0xffff0000) >> 16) as u32
    }

    fn low_word(&self) -> u32 {
        (self.0 & 0xffff) as u32
    }
}

trait LPARAMExt {
    fn as_cursor_pos(&self) -> (i32, i32);
}

impl LPARAMExt for LPARAM {
    fn as_cursor_pos(&self) -> (i32, i32) {
        let x = self.0 & 0xffff;
        let y = (self.0 & 0xffff0000) >> 16;
        (x as _, y as _)
    }
}

trait RECTExt {
    fn as_points_mut(&mut self) -> &mut [POINT];
}

impl RECTExt for RECT {
    fn as_points_mut(&mut self) -> &mut [POINT] {
        unsafe { std::slice::from_raw_parts_mut(self as *mut RECT as *mut POINT, 2) }
    }
}
