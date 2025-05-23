use std::env;
use std::path::Path;

use which::which;

fn main() {
    let flutter_exe;
    let flutter_bin = env::var("FLUTTER_BIN_DIR");
    let flutter_bin = match &flutter_bin {
        Ok(p) => Path::new(p),
        Err(_) => {
            flutter_exe = which("flutter").expect("flutter not found in PATH");
            flutter_exe.parent().unwrap()
        }
    };

    let header = flutter_bin.join("cache/artifacts/engine/windows-x64/flutter_windows.h");

    let bindings = bindgen::builder()
        .header(header.to_str().unwrap())
        .allowlist_function(r"Flutter\w+")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .unwrap();

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
