#![allow(non_snake_case)]

pub mod Microsoft;
pub mod ui_interop;
pub mod version_info;

use version_info::*;
use windows::core::{HRESULT, PCWSTR};

extern "C" {
    fn MddBootstrapInitialize2(
        major_minor_version: u32,
        version_tag: PCWSTR,
        min_version: u64,
        options: i32,
    ) -> HRESULT;
}

pub fn initialize() -> windows::core::Result<()> {
    unsafe {
        MddBootstrapInitialize2(
            WINDOWSAPPSDK_RELEASE_MAJORMINOR,
            WINDOWSAPPSDK_RELEASE_VERSION_TAG_W,
            WINDOWSAPPSDK_RUNTIME_VERSION_UINT64,
            0,
        )
        .ok()
    }
}
