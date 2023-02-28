use std::env;
use std::path::PathBuf;

use which::which;

fn main() {
    let dart_sdk_include = which("flutter")
        .expect("flutter executable must be on the PATH")
        .parent()
        .unwrap()
        .join("cache/dart-sdk/include");

    cc::Build::new()
        .file(dart_sdk_include.join("dart_api_dl.c"))
        .compile("dart");

    bindgen::builder()
        .header(dart_sdk_include.join("dart_api_dl.h").to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap()
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .unwrap();
}
