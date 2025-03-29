use std::path::Path;

use cli::wgsl::generate_import_path_map;



fn main() {
    let import_paths = generate_import_path_map(Path::new("/Users/hectorcrean/rust/bevy")).unwrap();
    println!("{:?}", import_paths);
}