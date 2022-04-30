use std::rc::Rc;

use webview2_com::Microsoft::Web::WebView2::Win32::*;
use webview2_com::{
    ContainsFullScreenElementChangedEventHandler, CoreWebView2EnvironmentOptions,
    CreateCoreWebView2CompositionControllerCompletedHandler,
    CreateCoreWebView2EnvironmentCompletedHandler, CursorChangedEventHandler,
};
use widestring::{U16CStr, U16CString};
use windows::core::{Error, Interface, Result};
use windows::Win32::Foundation::{BOOL, E_POINTER, HWND, LPARAM, LRESULT, POINT, PWSTR, WPARAM};
use windows::Win32::Graphics::Gdi::MapWindowPoints;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetCapture, ReleaseCapture, SetCapture};
use windows::Win32::UI::Input::Pointer::*;
use windows::Win32::UI::Shell::{
    DefSubclassProc, GetWindowSubclass, RemoveWindowSubclass, SetWindowSubclass,
};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::UI::Composition::ContainerVisual;
use winit::window::{CursorIcon, Fullscreen, Window};

use crate::ext::WindowExt;

pub struct WebView {
    inner: Rc<WebViewState>,
}

struct WebViewState {
    environment: ICoreWebView2Environment3,
    controller: ICoreWebView2Controller2,
    composition_controller: ICoreWebView2CompositionController,
}

pub trait Handler {
    fn on_message_received(&mut self, webview: &WebView, message: String);
}

impl WebView {
    pub fn new(window: Rc<Window>) -> Result<WebView> {
        let hwnd = window.hwnd();
        let size = window.inner_size();

        let environment: ICoreWebView2Environment3 = create_environment()?.cast()?;
        let composition_controller = create_composition_controller(hwnd, &environment)?;
        let controller: ICoreWebView2Controller2 = composition_controller.cast()?;

        let webview = WebView {
            inner: Rc::new(WebViewState {
                environment,
                controller,
                composition_controller,
            }),
        };

        // Update window cursor icon on webview cursor change
        webview.add_cursor_changed_handler({
            let window = window.clone();
            move |_, cursor| {
                window.set_cursor_icon(to_winit_cursor(cursor));
                Ok(())
            }
        })?;

        // Toggle window fullscreen on webview fullscreen change
        webview.add_fullscreen_changed_handler(move |_, fullscreen| {
            if fullscreen {
                window.set_fullscreen(Some(Fullscreen::Borderless(None)));
            } else {
                window.set_fullscreen(None);
            }
            Ok(())
        })?;

        // Make the webview transparent
        webview.set_background_color((0, 0, 0, 0))?;

        // Register window subclass to forward input to webview.
        // We clone the Rc here and convert it to a raw pointer to pass to the window proc.
        // This needs to be converted back to an Rc and dropped, to decrement the refcount when
        // the webview is destroyed.
        let userdata = Rc::into_raw(webview.inner.clone());
        unsafe {
            SetWindowSubclass(hwnd, Some(pfnsubclass), 123, userdata as _).ok()?;
        }

        webview.set_size(size.width, size.height);
        webview.set_visible(true);

        Ok(webview)
    }

    fn add_cursor_changed_handler<F>(&self, mut handler: F) -> Result<()>
    where
        F: FnMut(ICoreWebView2CompositionController, u32) -> Result<()> + 'static,
    {
        let mut token = Default::default();
        let eventhandler = CursorChangedEventHandler::create(Box::new(move |controller, _| {
            let controller = controller.unwrap();
            let mut cursor = 0;
            unsafe { controller.SystemCursorId(&mut cursor) }?;
            handler(controller, cursor)
        }));

        // Register cursor icon change handler
        unsafe {
            self.inner
                .composition_controller
                .CursorChanged(eventhandler, &mut token)?;
        }

        Ok(())
    }

    fn add_fullscreen_changed_handler<F>(&self, mut handler: F) -> Result<()>
    where
        F: FnMut(ICoreWebView2, bool) -> Result<()> + 'static,
    {
        let mut token = Default::default();
        let eventhandler = ContainsFullScreenElementChangedEventHandler::create(Box::new(
            move |webview: Option<ICoreWebView2>, _| {
                let webview = webview.unwrap();
                let mut fullscreen = BOOL(0);
                unsafe { webview.ContainsFullScreenElement(&mut fullscreen)? };
                handler(webview, fullscreen.as_bool())
            },
        ));

        // Register cursor icon change handler
        unsafe {
            self.inner
                .controller
                .CoreWebView2()?
                .ContainsFullScreenElementChanged(eventhandler, &mut token)?;
        }

        Ok(())
    }

    pub fn set_size(&self, width: u32, height: u32) {
        unsafe {
            self.inner
                .controller
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
            self.inner.controller.SetIsVisible(visible).unwrap();
        }
    }

    pub fn set_background_color(&self, color: (u8, u8, u8, u8)) -> Result<()> {
        let controller: ICoreWebView2Controller2 = self.inner.composition_controller.cast()?;
        unsafe {
            controller.SetDefaultBackgroundColor(COREWEBVIEW2_COLOR {
                R: color.0,
                G: color.1,
                B: color.2,
                A: color.3,
            })
        }
    }

    pub fn set_visual_target(&self, visual: &ContainerVisual) {
        unsafe {
            self.inner
                .composition_controller
                .SetRootVisualTarget(visual)
                .unwrap();
        }
    }

    pub fn set_message_handler(&self, mut message_handler: impl Handler + 'static) {
        let state = self.inner.clone();
        let callback =
            webview2_com::WebMessageReceivedEventHandler::create(Box::new(move |_, args| unsafe {
                let args = args.unwrap();
                let mut message = Default::default();
                args.TryGetWebMessageAsString(&mut message)?;

                let message = U16CStr::from_ptr_str(message.0).to_string().unwrap();
                let webview = WebView {
                    inner: state.clone(),
                };

                message_handler.on_message_received(&webview, message);

                Ok(())
            }));

        let mut token = Default::default();
        unsafe {
            self.inner
                .controller
                .CoreWebView2()
                .unwrap()
                .WebMessageReceived(callback, &mut token)
                .unwrap();
        }
    }

    pub fn send_json_message(&self, message: &str) -> Result<()> {
        unsafe {
            self.inner
                .controller
                .CoreWebView2()?
                .PostWebMessageAsJson(message)
        }
    }

    pub fn navigate_to_url(&self, url: &str) {
        unsafe {
            self.inner
                .controller
                .CoreWebView2()
                .unwrap()
                .Navigate(url)
                .unwrap();

            self.inner
                .controller
                .CoreWebView2()
                .unwrap()
                .OpenDevToolsWindow()
                .unwrap();
        }
    }

    #[allow(unused)]
    pub fn navigate_to_string(&self, html: &str) {
        unsafe {
            self.inner
                .controller
                .CoreWebView2()
                .unwrap()
                .NavigateToString(html)
                .unwrap();
        }
    }

    pub fn close(&self) {
        unsafe {
            // Get the handle of the webview's parent window
            let mut hwnd = HWND::default();
            self.inner.controller.ParentWindow(&mut hwnd).unwrap();

            // Get the userdata pointer that was registered with the window subclass
            let mut userdata = 0;
            GetWindowSubclass(hwnd, Some(pfnsubclass), 123, &mut userdata)
                .ok()
                .unwrap();

            // Unregister the window subclass and drop the userdata (to decrement the refcount)
            RemoveWindowSubclass(hwnd, Some(pfnsubclass), 123);
            drop(Rc::from_raw(userdata as *const WebViewState));

            // Finally destroy the webview controller
            self.inner.controller.Close().unwrap();
        }
    }
}

fn create_environment() -> Result<ICoreWebView2Environment> {
    let (tx, rx) = std::sync::mpsc::channel();

    let options = ICoreWebView2EnvironmentOptions::from(CoreWebView2EnvironmentOptions::default());

    unsafe {
        options.SetAdditionalBrowserArguments("--flag-switches-begin --enable-features=msOverlayScrollbarWinStyleMasterFlag,msVisualRejuvMaterialsTitleBar --flag-switches-end").unwrap();
    }

    let mut user_data_dir = std::env::var("ZENITH_USER_DATA")
        .ok()
        .map(|it| U16CString::from_str(it).unwrap());

    let handler = CreateCoreWebView2EnvironmentCompletedHandler::create(Box::new(
        move |error_code, environment| {
            error_code?;
            tx.send(environment.ok_or_else(|| Error::fast_error(E_POINTER)))
                .unwrap();
            Ok(())
        },
    ));

    unsafe {
        CreateCoreWebView2EnvironmentWithOptions(
            None,
            user_data_dir.as_mut().map(|it| PWSTR(it.as_mut_ptr())),
            options,
            handler,
        )?;
    }

    rx.recv().unwrap()
}

fn create_composition_controller(
    hwnd: HWND,
    environment: &ICoreWebView2Environment3,
) -> Result<ICoreWebView2CompositionController> {
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
            tx.send(controller.ok_or_else(|| Error::fast_error(E_POINTER)))
                .unwrap();
            Ok(())
        }),
    )
    .unwrap();

    rx.recv().unwrap()
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
    fn to_cursor_pos(&self) -> POINT;
}

impl LPARAMExt for LPARAM {
    fn to_cursor_pos(&self) -> POINT {
        let x = self.0 & 0xffff;
        let y = (self.0 & 0xffff0000) >> 16;
        POINT {
            x: x as i32,
            y: y as i32,
        }
    }
}

unsafe extern "system" fn pfnsubclass(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    id: usize,
    userdata: usize,
) -> LRESULT {
    debug_assert_eq!(id, 123);

    let state = (userdata as *const WebViewState).as_ref().unwrap();

    match msg {
        WM_MOUSEMOVE => {
            state
                .composition_controller
                .SendMouseInput(
                    COREWEBVIEW2_MOUSE_EVENT_KIND_MOVE,
                    wparam.low_word(),
                    0,
                    lparam.to_cursor_pos(),
                )
                .unwrap();
        }

        WM_LBUTTONDOWN | WM_LBUTTONUP | WM_LBUTTONDBLCLK | WM_RBUTTONDOWN | WM_RBUTTONUP
        | WM_RBUTTONDBLCLK | WM_MBUTTONDOWN | WM_MBUTTONUP | WM_MBUTTONDBLCLK | WM_XBUTTONDOWN
        | WM_XBUTTONUP | WM_XBUTTONDBLCLK => {
            match msg {
                WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_XBUTTONDOWN => {
                    if GetCapture() != hwnd {
                        SetCapture(hwnd);
                    }
                }
                WM_LBUTTONUP | WM_RBUTTONUP | WM_MBUTTONUP | WM_XBUTTONUP => {
                    if GetCapture() == hwnd {
                        ReleaseCapture();
                    }
                }
                _ => {}
            }

            let mousedata = if matches!(msg, WM_XBUTTONDOWN | WM_XBUTTONUP | WM_XBUTTONDBLCLK) {
                wparam.high_word()
            } else {
                0
            };

            state
                .composition_controller
                .SendMouseInput(
                    msg as COREWEBVIEW2_MOUSE_EVENT_KIND,
                    wparam.low_word(),
                    mousedata,
                    lparam.to_cursor_pos(),
                )
                .unwrap();
        }

        WM_MOUSEWHEEL | WM_MOUSEHWHEEL => {
            let mut position = lparam.to_cursor_pos();
            MapWindowPoints(None, hwnd, &mut position, 1);

            state
                .composition_controller
                .SendMouseInput(
                    msg as COREWEBVIEW2_MOUSE_EVENT_KIND,
                    wparam.low_word(),
                    wparam.high_word(),
                    position,
                )
                .unwrap();
        }

        WM_POINTERACTIVATE | WM_POINTERDOWN | WM_POINTERENTER | WM_POINTERLEAVE | WM_POINTERUP
        | WM_POINTERUPDATE => {
            let mut ptrinfo = POINTER_INFO::default();

            GetPointerInfo(wparam.low_word(), &mut ptrinfo)
                .ok()
                .unwrap();

            if ptrinfo.pointerType != PT_TOUCH && ptrinfo.pointerType != PT_PEN {
                return DefWindowProcW(hwnd, msg, wparam, lparam);
            }

            let info = state.environment.CreateCoreWebView2PointerInfo().unwrap();

            let mut pixel_loc = ptrinfo.ptPixelLocation;
            let mut pixel_loc_raw = ptrinfo.ptPixelLocationRaw;

            MapWindowPoints(None, hwnd, &mut pixel_loc, 1);
            MapWindowPoints(None, hwnd, &mut pixel_loc_raw, 1);

            info.SetPointerKind(ptrinfo.pointerType as u32).unwrap();
            info.SetPointerId(ptrinfo.pointerId).unwrap();
            info.SetFrameId(ptrinfo.frameId).unwrap();
            info.SetPointerFlags(ptrinfo.pointerFlags).unwrap();
            info.SetPixelLocation(pixel_loc).unwrap();
            info.SetPixelLocationRaw(pixel_loc_raw).unwrap();
            info.SetTime(ptrinfo.dwTime).unwrap();
            info.SetHistoryCount(ptrinfo.historyCount).unwrap();
            info.SetButtonChangeKind(ptrinfo.ButtonChangeType).unwrap();

            if ptrinfo.pointerType == PT_TOUCH {
                let mut touchinfo = POINTER_TOUCH_INFO::default();

                GetPointerTouchInfo(wparam.low_word(), &mut touchinfo)
                    .ok()
                    .unwrap();

                let mut contact = touchinfo.rcContact;
                let mut contact_raw = touchinfo.rcContactRaw;

                MapWindowPoints(None, hwnd, &mut contact as *mut _ as _, 2);
                MapWindowPoints(None, hwnd, &mut contact_raw as *mut _ as _, 2);

                info.SetTouchFlags(touchinfo.touchFlags).unwrap();
                info.SetTouchMask(touchinfo.touchMask).unwrap();
                info.SetTouchContact(contact).unwrap();
                info.SetTouchContactRaw(contact_raw).unwrap();
                info.SetTouchOrientation(touchinfo.orientation).unwrap();
                info.SetTouchPressure(touchinfo.pressure).unwrap();
            } else if ptrinfo.pointerType == PT_PEN {
                let mut peninfo = POINTER_PEN_INFO::default();

                GetPointerPenInfo(wparam.low_word(), &mut peninfo)
                    .ok()
                    .unwrap();

                info.SetPenFlags(peninfo.penFlags).unwrap();
                info.SetPenMask(peninfo.penMask).unwrap();
                info.SetPenPressure(peninfo.pressure).unwrap();
                info.SetPenRotation(peninfo.rotation).unwrap();
                info.SetPenTiltX(peninfo.tiltX).unwrap();
                info.SetPenTiltY(peninfo.tiltY).unwrap();
            }

            state
                .composition_controller
                .SendPointerInput(msg as COREWEBVIEW2_POINTER_EVENT_KIND, info)
                .unwrap();
        }

        _ => return DefSubclassProc(hwnd, msg, wparam, lparam),
    }

    LRESULT(0)
}

fn to_winit_cursor(cursor: u32) -> CursorIcon {
    let cursor = PWSTR(cursor as _);
    if cursor == IDC_ARROW {
        CursorIcon::Arrow
    } else if cursor == IDC_HAND {
        CursorIcon::Hand
    } else if cursor == IDC_IBEAM {
        CursorIcon::Text
    } else if cursor == IDC_NO {
        CursorIcon::NotAllowed
    } else if cursor == IDC_SIZEALL {
        CursorIcon::Grab
    } else if cursor == IDC_SIZEWE {
        CursorIcon::EwResize
    } else if cursor == IDC_SIZENS {
        CursorIcon::NsResize
    } else if cursor == IDC_SIZENESW {
        CursorIcon::NeswResize
    } else if cursor == IDC_SIZENWSE {
        CursorIcon::NwseResize
    } else if cursor == IDC_WAIT {
        CursorIcon::Wait
    } else if cursor == IDC_APPSTARTING {
        CursorIcon::Progress
    } else if cursor == IDC_HELP {
        CursorIcon::Help
    } else {
        CursorIcon::Default
    }
}
