use std::mem::size_of;
use std::path::Path;
use std::{io, slice};

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowPlacement, SetWindowPlacement, WINDOWPLACEMENT,
};

pub fn try_restore(path: &Path, window: HWND) -> io::Result<()> {
    let bytes = std::fs::read(path)?;

    if bytes.len() != size_of::<WINDOWPLACEMENT>() {
        return Err(io::Error::new(io::ErrorKind::Other, "file is invalid"));
    }

    let placement = unsafe { bytes.align_to::<WINDOWPLACEMENT>().1[0] };
    unsafe { SetWindowPlacement(window, &placement)? };

    Ok(())
}

pub fn try_save(path: &Path, window: HWND) -> io::Result<()> {
    let bytes = unsafe {
        let mut placement = WINDOWPLACEMENT {
            length: size_of::<WINDOWPLACEMENT>() as u32,
            ..Default::default()
        };

        GetWindowPlacement(window, &mut placement)?;

        &slice::from_raw_parts(
            &placement as *const WINDOWPLACEMENT as *const u8,
            placement.length as usize,
        )
    };

    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, bytes).unwrap();

    Ok(())
}
