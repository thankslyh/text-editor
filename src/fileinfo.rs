use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: Option<PathBuf>,
}

impl FileInfo {
    pub fn from(path: &str) -> Self {
        Self {
            path: Some(PathBuf::from(path)),
        }
    }

    pub fn get_path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub const fn has_path(&self) -> bool {
        self.path.is_some()
    }
}

impl Display for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self
            .get_path()
            .and_then(|p| p.file_name())
            .and_then(|p| p.to_str())
            .unwrap();
        write!(f, "{name}")
    }
}
