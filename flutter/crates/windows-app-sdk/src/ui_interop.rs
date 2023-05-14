use windows::core::Result;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows::Win32::UI::WindowsAndMessaging::HICON;
use windows::{s, w};

use crate::Microsoft;
use crate::Microsoft::UI::{IconId, WindowId};

// This code is ported from Microsoft.UI.Interop.h in the WindowsAppSDK nuget package and is
// probably unstable.

pub struct WindowingInteropFns {
    GetWindowFromWindowId:
        unsafe extern "stdcall" fn(WindowId, *mut HWND) -> windows::core::HRESULT,
    GetIconIdFromIcon: unsafe extern "stdcall" fn(
        windows::Win32::UI::WindowsAndMessaging::HICON,
        *mut Microsoft::UI::IconId,
    ) -> windows::core::HRESULT,
}

impl WindowingInteropFns {
    pub fn load() -> Result<WindowingInteropFns> {
        unsafe {
            let lib = LoadLibraryW(w!("Microsoft.Internal.FrameworkUdk.dll"))?;

            let GetIconIdFromIcon = std::mem::transmute(
                GetProcAddress(lib, s!("Windowing_GetIconIdFromIcon")).unwrap(),
            );
            let GetWindowFromWindowId = std::mem::transmute(
                GetProcAddress(lib, s!("Windowing_GetWindowFromWindowId")).unwrap(),
            );

            Ok(WindowingInteropFns {
                GetWindowFromWindowId,
                GetIconIdFromIcon,
            })
        }
    }

    pub fn GetWindowFromWindowId(&self, window_id: WindowId) -> Result<HWND> {
        let mut window = HWND::default();
        unsafe { (self.GetWindowFromWindowId)(window_id, &mut window).ok()? };
        Ok(window)
    }

    pub fn GetIconIdFromIcon(&self, icon: HICON) -> Result<IconId> {
        let mut icon_id = IconId::default();
        unsafe { (self.GetIconIdFromIcon)(icon, &mut icon_id).ok()? };
        Ok(icon_id)
    }
}
