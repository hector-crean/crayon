use std::fs;
use wesl::PkgBuilder;

fn main() {
    fs::create_dir_all("src/shaders/bevy").expect("failed to create shader directory");
    
    PkgBuilder::new("bevy")
        .scan_directory("src/shaders/bevy")
        .expect("failed to scan WESL files")
        .validate()
        .map_err(|e| eprintln!("{e}"))
        .expect("validation error")
        .build_artefact()
        .expect("failed to build artifact");
}


