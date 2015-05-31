use std::fs;
use std::path::Path;

pub fn list_files(path: &Path) -> Vec<String> {
    let mut result = Vec::new();

    let path_str = path.to_str().unwrap();

    let metadata = fs::metadata(path_str).unwrap();
    if metadata.is_file() {
        result.push(path_str.to_string());
    } else if metadata.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();

            let path_buf = entry.path();
            let path = path_buf.as_path();
            let path_str = path.to_str().unwrap();

            let metadata = fs::metadata(path_str).unwrap();
            if metadata.is_file() {
                result.push(path_str.to_string());
            }
        }
    }

    return result;
}