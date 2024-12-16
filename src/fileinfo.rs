use std::{fmt::Display, path::PathBuf};

#[derive(Debug)]
pub struct FileInfo {
    pub path: Option<PathBuf>,
}

impl FileInfo {
    pub fn from(path: &str) -> Self {
        Self {
            path: Some(PathBuf::from(path)),
        }
    }
}

impl Display for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self
            .path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|p| p.to_str())
            .unwrap();
        write!(f, "{name}")
    }
}
