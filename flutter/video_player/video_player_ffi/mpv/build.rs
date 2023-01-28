use std::env;
use std::path::PathBuf;

fn main() {
    let mpv_dir = PathBuf::from(env::var("MPV_DIR").expect("MPV_DIR must be set"));

    println!("cargo:rerun-if-env-changed=MPV_DIR");
    println!("cargo:rustc-link-search={}", mpv_dir.display());
    println!("cargo:rustc-link-lib=dylib=mpv");

    bindgen::builder()
        .header(mpv_dir.join("include/mpv/client.h").to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap()
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .unwrap();
}
