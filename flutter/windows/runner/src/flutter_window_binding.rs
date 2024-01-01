use std::ptr;

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::SetFocus;
use windows::Win32::UI::Shell::{DefSubclassProc, RemoveWindowSubclass, SetWindowSubclass};
use windows::Win32::UI::WindowsAndMessaging::{
    GetClientRect, MoveWindow, SetParent, MINMAXINFO, WM_ACTIVATE, WM_FONTCHANGE, WM_GETMINMAXINFO,
    WM_SIZE,
};

use crate::flutter_view_controller::FlutterDesktopViewController;

pub struct FlutterWindowBinding<'a> {
    window: HWND,
    _view_controller: &'a FlutterDesktopViewController,
}

impl<'a> FlutterWindowBinding<'a> {
    #[must_use = "this should usually stay alive until the application exits"]
    pub fn new(
        view_controller: &FlutterDesktopViewController,
        window: HWND,
    ) -> FlutterWindowBinding {
        let view_window = view_controller.view().hwnd();

        unsafe {
            SetParent(view_window, window);

            let mut rect = RECT::default();
            GetClientRect(window, &mut rect);

            MoveWindow(
                view_window,
                rect.left,
                rect.top,
                rect.right - rect.left,
                rect.bottom - rect.top,
                true,
            );

            SetFocus(view_window);

            SetWindowSubclass(
                window,
                Some(subclass_proc),
                424242,
                view_controller as *const FlutterDesktopViewController as usize,
            );
        }

        FlutterWindowBinding {
            window,
            _view_controller: view_controller,
        }
    }
}

impl<'a> Drop for FlutterWindowBinding<'a> {
    fn drop(&mut self) {
        unsafe { RemoveWindowSubclass(self.window, Some(subclass_proc), 424242) };
    }
}

unsafe extern "system" fn subclass_proc(
    hwnd: HWND,
    umsg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    _uidsubclass: usize,
    dwrefdata: usize,
) -> LRESULT {
    let view_controller = (dwrefdata as *const FlutterDesktopViewController)
        .as_ref()
        .unwrap();

    if let Some(result) = view_controller.handle_top_level_window_proc(hwnd, umsg, wparam, lparam) {
        return result;
    }

    match umsg {
        WM_SIZE => {
            let mut rect = RECT::default();
            GetClientRect(hwnd, &mut rect);
            MoveWindow(
                view_controller.view().hwnd(),
                rect.left,
                rect.top,
                rect.right - rect.left,
                rect.bottom - rect.top,
                true,
            );
            return LRESULT(0);
        }
        WM_ACTIVATE => {
            SetFocus(view_controller.view().hwnd());
            return LRESULT(0);
        }
        WM_FONTCHANGE => view_controller.engine().reload_system_fonts(),
        WM_GETMINMAXINFO => {
            let out_ptr = lparam.0 as *mut MINMAXINFO;
            let min_track_size_ptr = ptr::addr_of_mut!((*out_ptr).ptMinTrackSize);
            ptr::write(min_track_size_ptr, POINT { x: 300, y: 300 });
            return LRESULT(0);
        }
        _ => {}
    }

    DefSubclassProc(hwnd, umsg, wparam, lparam)
}
