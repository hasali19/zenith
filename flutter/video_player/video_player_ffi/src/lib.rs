use std::sync::mpsc;

use mpv::{
    mpv_command, mpv_create, mpv_format_MPV_FORMAT_FLAG, mpv_format_MPV_FORMAT_INT64, mpv_handle,
    mpv_initialize, mpv_set_option, mpv_set_property,
};
use windows::w;
use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::CreateSolidBrush;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::{GetStartupInfoW, STARTUPINFOW};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadCursorW, LoadIconW,
    PostQuitMessage, RegisterClassW, TranslateMessage, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT,
    IDC_ARROW, IDI_APPLICATION, MSG, WINDOW_EX_STYLE, WM_DESTROY, WNDCLASSW, WS_OVERLAPPEDWINDOW,
};

macro_rules! s {
    ($s:literal) => {{
        concat!($s, "\0").as_ptr() as *const u8 as *const i8
    }};
}

#[no_mangle]
pub unsafe extern "C" fn create_window() -> HWND {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || unsafe {
        let mut startup_info = STARTUPINFOW::default();
        GetStartupInfoW(&mut startup_info);

        unsafe extern "system" fn window_proc(
            window: HWND,
            message: u32,
            wparam: WPARAM,
            lparam: LPARAM,
        ) -> LRESULT {
            if message == WM_DESTROY {
                PostQuitMessage(0);
                return LRESULT(0);
            }
            DefWindowProcW(window, message, wparam, lparam)
        }

        let instance = GetModuleHandleW(None).unwrap();
        let window_class = WNDCLASSW {
            lpszClassName: w!("DART_VLC_WINDOW"),
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
            w!("DART_VLC_WINDOW"),
            w!("dart_vlc.instance"),
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
        while GetMessageW(&mut msg, window, 0, 0).as_bool() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    });

    rx.recv().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn create_player(hwnd: HWND) -> *mut mpv_handle {
    let ctx = mpv_create();
    mpv_set_option(
        ctx,
        s!("wid"),
        mpv_format_MPV_FORMAT_INT64,
        &mut hwnd.clone() as *mut _ as *mut _,
    );
    mpv_initialize(ctx);
    ctx
}

#[no_mangle]
pub unsafe extern "C" fn load(ctx: *mut mpv_handle, url: *const i8) {
    mpv_command(
        ctx,
        &mut [s!("loadfile"), url, std::ptr::null()] as *mut *const i8,
    );
}

#[no_mangle]
pub unsafe extern "C" fn pause(ctx: *mut mpv_handle) {
    mpv_set_property(
        ctx,
        s!("pause"),
        mpv_format_MPV_FORMAT_FLAG,
        &mut 1 as *mut _ as *mut _,
    );
}

#[no_mangle]
pub unsafe extern "C" fn play(ctx: *mut mpv_handle) {
    mpv_set_property(
        ctx,
        s!("pause"),
        mpv_format_MPV_FORMAT_FLAG,
        &mut 0 as *mut _ as *mut _,
    );
}
