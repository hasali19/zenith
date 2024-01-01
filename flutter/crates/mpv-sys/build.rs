use std::env;
use std::path::PathBuf;

fn main() {
    let mpv_dir = PathBuf::from(env::var("MPV_DIR").expect("MPV_DIR must be set"));

    println!("cargo:rerun-if-env-changed=MPV_DIR");
    println!("cargo:rustc-link-search={}", mpv_dir.display());

    let bindings = bindgen::builder()
        .header(mpv_dir.join("include/mpv/client.h").to_str().unwrap())
        .header(mpv_dir.join("include/mpv/render.h").to_str().unwrap())
        .header(mpv_dir.join("include/mpv/render_gl.h").to_str().unwrap())
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .unwrap()
        .to_string();

    let bindings = bindings.replace(
        "extern \"C\" {",
        "#[link(name = \"libmpv-2\", kind = \"raw-dylib\")] extern \"C\" {",
    );

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    std::fs::write(out_path, bindings).unwrap();
}
