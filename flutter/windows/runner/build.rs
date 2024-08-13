use std::path::PathBuf;
use std::{env, io};

use embed_manifest::embed_manifest_file;

fn main() {
    let sdk = get_winsdk_include().unwrap();

    let defines = [
        "FLUTTER_VERSION",
        "FLUTTER_VERSION_MAJOR",
        "FLUTTER_VERSION_MINOR",
        "FLUTTER_VERSION_PATCH",
        "FLUTTER_VERSION_BUILD",
    ];

    let mut build = windres::Build::new();

    build.include(sdk.join("um")).include(sdk.join("shared"));

    for define in defines {
        build.define(define, env::var(define).as_deref().ok());
    }

    build.compile("Runner.rc").unwrap();

    embed_manifest_file("runner.exe.manifest").unwrap();
}

fn get_winsdk_include() -> io::Result<PathBuf> {
    let mut paths = std::fs::read_dir(r"C:\Program Files (x86)\Windows Kits\10\Include")?
        .flatten()
        .map(|e| e.path())
        .collect::<Vec<_>>();
    paths.sort();
    Ok(paths.into_iter().next_back().unwrap())
}
