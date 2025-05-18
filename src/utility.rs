use std::path::PathBuf;

pub fn is_typ(path_buf: &PathBuf) -> bool {
    return path_buf.extension().map(|ext| ext == "typ").unwrap_or(false);
}

pub fn is_yml(path_buf: &PathBuf) -> bool {
    return path_buf.extension().map(|ext| ext == "yml" || ext == "yaml" ).unwrap_or(false);
}