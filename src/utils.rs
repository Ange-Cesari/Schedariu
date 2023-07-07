use std::path::{Path, PathBuf};

pub struct Utils;

impl Utils {
    pub fn new() -> Self {
        Self {}
    }

    pub fn path_to_string(&self, path: &Path) -> String {
        path.to_str()
            .expect("Failed to convert Path to String")
            .to_string()
    }

    pub fn replace_extension(&self, path: &Path, new_ext: &str) -> PathBuf {
        path.with_extension(new_ext)
    }

    pub fn get_filename(&self, path: &Path) -> String {
        path.file_stem()
            .expect("Failed to get file name")
            .to_str()
            .expect("Failed to convert OsStr to String")
            .to_string()
    }
}