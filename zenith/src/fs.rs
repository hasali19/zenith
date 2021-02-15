use std::path::{Path, PathBuf};

pub enum DirEntryType {
    Directory,
    File,
}

pub struct DirEntry {
    pub entry_type: DirEntryType,
    pub path: PathBuf,
}

pub trait FileSystem {
    fn list_dir(&self, path: &Path) -> eyre::Result<Vec<DirEntry>>;
}

pub struct RealFs;

impl FileSystem for RealFs {
    fn list_dir(&self, path: &Path) -> eyre::Result<Vec<DirEntry>> {
        Ok(std::fs::read_dir(path)?
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let file_type = e.file_type().ok()?;
                let entry_type = if file_type.is_dir() {
                    DirEntryType::Directory
                } else if file_type.is_file() {
                    DirEntryType::File
                } else {
                    return None;
                };

                Some(DirEntry {
                    entry_type,
                    path: e.path(),
                })
            })
            .collect())
    }
}
