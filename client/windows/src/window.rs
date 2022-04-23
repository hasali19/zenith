use std::ptr::null;

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, PWSTR, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::MapWindowPoints;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetClientRect, GetMessageW,
    GetWindowLongPtrW, PostQuitMessage, RegisterClassW, SetWindowLongPtrW, ShowWindow,
    TranslateMessage, CW_USEDEFAULT, GWLP_USERDATA, GWLP_WNDPROC, MK_CONTROL, MK_LBUTTON,
    MK_MBUTTON, MK_RBUTTON, MK_SHIFT, MK_XBUTTON1, MK_XBUTTON2, MSG, SW_SHOWNORMAL, WM_DESTROY,
    WM_LBUTTONDBLCLK, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDBLCLK, WM_MBUTTONDOWN, WM_MBUTTONUP,
    WM_MOUSEHWHEEL, WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_POINTERACTIVATE, WM_POINTERDOWN,
    WM_POINTERENTER, WM_POINTERLEAVE, WM_POINTERUP, WM_POINTERUPDATE, WM_RBUTTONDBLCLK,
    WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SIZE, WM_XBUTTONDBLCLK, WM_XBUTTONDOWN, WM_XBUTTONUP,
    WNDCLASSW, WS_EX_NOREDIRECTIONBITMAP, WS_OVERLAPPEDWINDOW,
};

pub struct Window {
    hwnd: HWND,
}

impl Window {
    pub fn new(title: &str) -> Window {
        let hinstance = unsafe { GetModuleHandleW(None) };

        let mut class_name = "MainWindow".into_pwstr();
        let wc = WNDCLASSW {
            lpfnWndProc: Some(default_window_proc),
            hInstance: hinstance,
            lpszClassName: PWSTR(class_name.as_mut_ptr()),
            ..Default::default()
        };

        unsafe {
            RegisterClassW(&wc);

            let hwnd = CreateWindowExW(
                WS_EX_NOREDIRECTIONBITMAP,
                PWSTR(class_name.as_mut_ptr()),
                title,
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                hinstance,
                null(),
            );

            if hwnd == HWND(0) {
                panic!("failed to create window");
            }

            ShowWindow(hwnd, SW_SHOWNORMAL);

            Window { hwnd }
        }
    }

    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }

    pub fn inner_size(&self) -> (u32, u32) {
        let mut rect = RECT::default();
        unsafe {
            GetClientRect(self.hwnd, &mut rect);
        }
        (
            rect.right.try_into().unwrap(),
            rect.bottom.try_into().unwrap(),
        )
    }
}

unsafe extern "system" fn default_window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => PostQuitMessage(0),
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }
    LRESULT(0)
}

unsafe extern "system" fn window_proc<H: Handler>(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let window = Window { hwnd };
    let handler = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut H;

    if handler.is_null() {
        return default_window_proc(hwnd, msg, wparam, lparam);
    }

    let handled = match msg {
        WM_SIZE => (*handler).on_resize(&window),

        WM_MOUSEMOVE => (*handler).on_mouse_move(
            &window,
            MouseMoveEvent {
                position: lparam.as_cursor_pos(),
                virtual_keys: VirtualKeys::from_wparam(wparam),
            },
        ),

        WM_LBUTTONDOWN | WM_LBUTTONUP | WM_LBUTTONDBLCLK | WM_RBUTTONDOWN | WM_RBUTTONUP
        | WM_RBUTTONDBLCLK | WM_MBUTTONDOWN | WM_MBUTTONUP | WM_MBUTTONDBLCLK | WM_XBUTTONDOWN
        | WM_XBUTTONUP | WM_XBUTTONDBLCLK => {
            (*handler).on_mouse_button(&window, MouseButtonEvent::new(msg, wparam, lparam).unwrap())
        }

        WM_MOUSEWHEEL | WM_MOUSEHWHEEL => (*handler).on_mouse_wheel(
            &window,
            MouseWheelEvent::new(&window, msg, wparam, lparam).unwrap(),
        ),

        WM_POINTERACTIVATE | WM_POINTERDOWN | WM_POINTERENTER | WM_POINTERLEAVE | WM_POINTERUP
        | WM_POINTERUPDATE => {
            let kind = match msg {
                WM_POINTERACTIVATE => PointerEventKind::Activate,
                WM_POINTERDOWN => PointerEventKind::Down,
                WM_POINTERENTER => PointerEventKind::Enter,
                WM_POINTERLEAVE => PointerEventKind::Leave,
                WM_POINTERUP => PointerEventKind::Up,
                WM_POINTERUPDATE => PointerEventKind::Update,
                _ => unreachable!(),
            };

            (*handler).on_pointer(
                &window,
                PointerEvent {
                    kind,
                    id: wparam.low_word(),
                    msg,
                    wparam,
                },
            )
        }
        _ => false,
    };

    if !handled {
        return default_window_proc(hwnd, msg, wparam, lparam);
    }

    LRESULT(0)
}

pub fn run<H: Handler>(window: &Window, mut handler: H) {
    let hwnd = window.hwnd();
    unsafe {
        SetWindowLongPtrW(hwnd, GWLP_WNDPROC, window_proc::<H> as usize as isize);
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, &mut handler as *mut _ as _);

        let mut msg = MSG::default();

        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

trait IntoPWSTR {
    fn into_pwstr(self) -> Vec<u16>;
}

impl IntoPWSTR for &str {
    fn into_pwstr(self) -> Vec<u16> {
        self.encode_utf16().chain([0u16]).collect()
    }
}

#[allow(unused)]
pub trait Handler {
    fn on_resize(&mut self, window: &Window) -> bool {
        false
    }

    fn on_mouse_move(&mut self, window: &Window, event: MouseMoveEvent) -> bool {
        false
    }

    fn on_mouse_button(&mut self, window: &Window, event: MouseButtonEvent) -> bool {
        false
    }

    fn on_mouse_wheel(&mut self, window: &Window, event: MouseWheelEvent) -> bool {
        false
    }

    fn on_pointer(&mut self, window: &Window, event: PointerEvent) -> bool {
        false
    }
}

pub enum ButtonAction {
    Down,
    Up,
    DoubleClick,
}

pub enum MouseButton {
    Left,
    Right,
    Middle,
    X(u32),
}

bitflags::bitflags! {
    pub struct VirtualKeys: u32 {
        const CONTROL = MK_CONTROL;
        const LBUTTON = MK_LBUTTON;
        const MBUTTON = MK_MBUTTON;
        const RBUTTON = MK_RBUTTON;
        const SHIFT = MK_SHIFT;
        const XBUTTON1 = MK_XBUTTON1;
        const XBUTTON2 = MK_XBUTTON2;
    }
}

impl VirtualKeys {
    pub fn from_wparam(wparam: WPARAM) -> VirtualKeys {
        VirtualKeys::from_bits(wparam.low_word()).unwrap()
    }
}

pub struct MouseMoveEvent {
    pub position: (i32, i32),
    pub virtual_keys: VirtualKeys,
}

pub struct MouseButtonEvent {
    pub button: MouseButton,
    pub action: ButtonAction,
    pub position: (i32, i32),
    pub virtual_keys: VirtualKeys,
}

impl MouseButtonEvent {
    fn new(msg: u32, wparam: WPARAM, lparam: LPARAM) -> Option<MouseButtonEvent> {
        use ButtonAction::*;
        use MouseButton::*;

        let (button, action) = match msg {
            WM_LBUTTONDOWN => (Left, Down),
            WM_LBUTTONUP => (Left, Up),
            WM_LBUTTONDBLCLK => (Left, DoubleClick),
            WM_RBUTTONDOWN => (Right, Down),
            WM_RBUTTONUP => (Right, Up),
            WM_RBUTTONDBLCLK => (Right, DoubleClick),
            WM_MBUTTONDOWN => (Middle, Down),
            WM_MBUTTONUP => (Middle, Up),
            WM_MBUTTONDBLCLK => (Middle, DoubleClick),
            WM_XBUTTONDOWN => (X(wparam.high_word()), Down),
            WM_XBUTTONUP => (X(wparam.high_word()), Up),
            WM_XBUTTONDBLCLK => (X(wparam.high_word()), DoubleClick),
            _ => return None,
        };

        Some(MouseButtonEvent {
            button,
            action,
            position: lparam.as_cursor_pos(),
            virtual_keys: VirtualKeys::from_wparam(wparam),
        })
    }
}

#[derive(Debug)]
pub enum MouseWheel {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub struct MouseWheelEvent {
    pub wheel: MouseWheel,
    pub delta: u32,
    pub position: (i32, i32),
    pub virtual_keys: VirtualKeys,
}

impl MouseWheelEvent {
    fn new(window: &Window, msg: u32, wparam: WPARAM, lparam: LPARAM) -> Option<MouseWheelEvent> {
        let wheel = match msg {
            WM_MOUSEWHEEL => MouseWheel::Vertical,
            WM_MOUSEHWHEEL => MouseWheel::Horizontal,
            _ => return None,
        };

        let delta = wparam.high_word();

        let (x, y) = lparam.as_cursor_pos();
        let mut point = POINT { x, y };
        unsafe {
            MapWindowPoints(None, window.hwnd(), &mut point, 1);
        }

        Some(MouseWheelEvent {
            wheel,
            delta,
            position: (point.x, point.y),
            virtual_keys: VirtualKeys::from_wparam(wparam),
        })
    }
}

pub enum PointerEventKind {
    Activate,
    Down,
    Enter,
    Leave,
    Up,
    Update,
}

pub struct PointerEvent {
    pub kind: PointerEventKind,
    pub id: u32,
    pub msg: u32,
    pub wparam: WPARAM,
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
