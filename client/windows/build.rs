use winres::WindowsResource;

fn main() {
    let mut res = WindowsResource::new();

    println!("cargo:rerun-if-env-changed=WINRES_TOOLKIT_PATH");

    if let Ok(toolkit_path) = std::env::var("WINRES_TOOLKIT_PATH") {
        res.set_toolkit_path(&toolkit_path);
    }

    res.set_manifest(include_str!("app.manifest"))
        .compile()
        .unwrap();
}
