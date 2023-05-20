use std::sync::mpsc;

use windows::w;
use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::CreateSolidBrush;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::{GetStartupInfoW, STARTUPINFOW};
use windows::Win32::UI::WindowsAndMessaging::*;

pub fn create() -> HWND {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || unsafe {
        let mut startup_info = STARTUPINFOW::default();
        GetStartupInfoW(&mut startup_info);

        let instance = GetModuleHandleW(None).unwrap();
        let window_class = WNDCLASSW {
            lpszClassName: w!("MPV_WINDOW"),
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: instance,
            hIcon: LoadIconW(None, IDI_APPLICATION).unwrap(),
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            hbrBackground: CreateSolidBrush(COLORREF(0)),
            ..Default::default()
        };

        RegisterClassW(&window_class);

        let window = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("MPV_WINDOW"),
            w!("MPV_WINDOW"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            instance,
            None,
        );

        tx.send(window).unwrap();

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, HWND(0), 0, 0).as_bool() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    });

    rx.recv().unwrap()
}

unsafe extern "system" fn window_proc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if message == WM_DESTROY {
        PostQuitMessage(0);
        return LRESULT(0);
    } else if message == WM_CLOSE {
        DestroyWindow(window);
        return LRESULT(0);
    }
    DefWindowProcW(window, message, wparam, lparam)
}

pub unsafe fn close(window: HWND) {
    PostMessageW(window, WM_CLOSE, WPARAM(0), LPARAM(0));
}
