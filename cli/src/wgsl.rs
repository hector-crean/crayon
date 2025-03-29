use std::collections::HashMap;
use std::path::{Path, PathBuf};
use naga_oil::compose::ComposerError;
use regex::Regex;

pub fn generate_import_path_map(root_dir: &Path) -> Result<HashMap<String, PathBuf>, ComposerError> {
    let mut import_paths = HashMap::new();

    let import_regex = Regex::new(r#"#define_import_path\s+(\S+)"#).unwrap();


    
    for entry in walkdir::WalkDir::new(root_dir).into_iter().filter_map(Result::ok) {
        match entry.path().extension().and_then(std::ffi::OsStr::to_str) {
            Some("wgsl") => {
                let path = entry.path();
                let source = std::fs::read_to_string(path).expect("Failed to read WGSL file");
                
                // Find all #define_import_path statements using regex
                for capture in import_regex.captures_iter(&source) {
                    if let Some(import_path) = capture.get(1) {
                        import_paths.insert(import_path.as_str().to_string(), path.to_path_buf());
                    }
                }
            },
            _ => continue,
        }
    }

    Ok(import_paths)
}

