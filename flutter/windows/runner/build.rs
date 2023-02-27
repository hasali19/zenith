use embed_manifest::embed_manifest_file;

fn main() {
    windres::Build::new().compile("Runner.rc").unwrap();
    embed_manifest_file("runner.exe.manifest").unwrap();
}
