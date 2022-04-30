use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let libvlc_dir = env::var("LIBVLC_DIR").expect("LIBVLC_DIR environment variable must be set");
    let libvlc_dir = Path::new(&libvlc_dir);

    let libvlc_lib_dir = libvlc_dir.join("sdk/lib");

    // Link the vlc import library
    println!("cargo:rustc-link-search={}", libvlc_lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=libvlc");

    let libvlc_include = libvlc_dir.join("sdk/include");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Generate libvlc bindings from header files
    bindgen::Builder::default()
        .clang_arg(format!("-I{}", libvlc_include.display()))
        .header("libvlc.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
