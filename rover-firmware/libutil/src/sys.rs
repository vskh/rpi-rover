use std::path::{Path, PathBuf};

pub fn normalize_path(input_path: &str, default_prefix: &Path) -> String {
    let path = PathBuf::from(input_path);

    if path.is_absolute() {
        input_path.to_owned()
    } else {
        let mut normalized_path = PathBuf::from(default_prefix);
        normalized_path.push(path);

        normalized_path.to_string_lossy().to_string()
    }
}
