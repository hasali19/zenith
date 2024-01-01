#![windows_subsystem = "windows"]

mod flutter_view_controller;
mod flutter_window_binding;
mod reference;
mod window_placement;

use ctor::ctor;
use flutter_desktop_messenger::codec::EncodableValue;
use reference::Reference;
use windows::core::PCWSTR;
use windows::h;
use windows::Foundation::TypedEventHandler;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, LoadIconW, PostQuitMessage, TranslateMessage, MSG,
};
use windows::UI::Color;
use windows_app_sdk::ui_interop::WindowingInteropFns;
use windows_app_sdk::Microsoft::UI::IconId;
use windows_app_sdk::Microsoft::UI::Windowing::{
    AppWindow, AppWindowPresenterKind, AppWindowTitleBar,
};

use crate::flutter_view_controller::FlutterDesktopViewController;
use crate::flutter_window_binding::FlutterWindowBinding;

/// See resource.h
const IDI_APP_ICON: PCWSTR = PCWSTR(101usize as _);

fn get_window_icon_id() -> windows::core::Result<IconId> {
    unsafe {
        let hinstance = GetModuleHandleW(None)?;
        let icon = LoadIconW(hinstance, IDI_APP_ICON)?;
        WINDOWING_INTEROP.GetIconIdFromIcon(icon)
    }
}

#[ctor]
static WINDOWING_INTEROP: WindowingInteropFns = {
    windows_app_sdk::initialize().unwrap();
    WindowingInteropFns::load().unwrap()
};

pub fn main() -> windows::core::Result<()> {
    windows_app_sdk::initialize()?;

    let window = AppWindow::Create()?;

    window.SetTitle(h!("Zenith"))?;
    window.SetIconWithIconId(get_window_icon_id()?)?;

    let title_bar = window.TitleBar()?;

    if AppWindowTitleBar::IsCustomizationSupported()? {
        const BACKGROUND: Color = Color {
            R: 32,
            G: 26,
            B: 24,
            A: 255,
        };

        const HOVER_BACKGROUND: Color = Color {
            R: 63,
            G: 63,
            B: 63,
            A: 255,
        };

        title_bar.SetBackgroundColor(&Reference::box_value(BACKGROUND))?;
        title_bar.SetButtonBackgroundColor(&Reference::box_value(BACKGROUND))?;
        title_bar.SetButtonHoverBackgroundColor(&Reference::box_value(HOVER_BACKGROUND))?;
    }

    let window_closing_handler = window.Closing(&TypedEventHandler::new(
        |window: &Option<AppWindow>, _args| {
            let window_placement_path = dirs::data_local_dir()
                .unwrap()
                .join("Zenith/windowplacement");

            let window = window.as_ref().unwrap();
            let window_id = window.Id()?;
            let hwnd = WINDOWING_INTEROP.GetWindowFromWindowId(window_id)?;

            if let Err(e) = window_placement::try_save(&window_placement_path, hwnd) {
                eprintln!("failed to save window placement state: {e}");
            }

            unsafe { PostQuitMessage(0) };

            Ok(())
        },
    ))?;

    let window_placement_path = dirs::data_local_dir()
        .unwrap()
        .join("Zenith/windowplacement");

    let window_handle = WINDOWING_INTEROP.GetWindowFromWindowId(window.Id()?)?;

    if let Err(e) = window_placement::try_restore(&window_placement_path, window_handle) {
        eprintln!("failed to restore window placement state: {e}");
    }

    window.Show()?;

    let window_size = window.ClientSize()?;

    let view_controller = Box::new(FlutterDesktopViewController::new(
        window_size.Width,
        window_size.Height,
    ));

    let messenger = &view_controller.engine().create_messenger();

    messenger.set_callback("zenith.hasali.uk/windowing", {
        let window = window.clone();
        move |name, args, reply| {
            if name == "isWindowed" {
                reply.success(&EncodableValue::Bool(true));
            } else if name == "setFullscreen" {
                let value = args.as_bool().expect("setFullscreen arg must be bool");

                if value {
                    window
                        .SetPresenterByKind(AppWindowPresenterKind::FullScreen)
                        .unwrap();
                } else {
                    window
                        .SetPresenterByKind(AppWindowPresenterKind::Default)
                        .unwrap();
                }

                reply.success(&EncodableValue::Null);
            } else {
                reply.not_implemented();
            }
        }
    });

    let _binding = FlutterWindowBinding::new(&view_controller, window_handle);

    unsafe {
        let mut message = MSG::default();
        while GetMessageW(&mut message, HWND::default(), 0, 0).as_bool() {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }

    window.RemoveClosing(window_closing_handler)?;

    Ok(())
}
