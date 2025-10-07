use anyhow::Result;
use chrono::{DateTime, Local};
use std::fs::{self, Metadata};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    File,
    Directory,
    Symlink,
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub file_type: FileType,
    pub size: u64,
    pub modified: SystemTime,
    pub permissions: String,
    pub is_hidden: bool,
}

impl FileEntry {
    pub fn from_path(path: &Path) -> Result<Self> {
        let metadata = fs::metadata(path)?;
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let file_type = if metadata.is_dir() {
            FileType::Directory
        } else if metadata.file_type().is_symlink() {
            FileType::Symlink
        } else {
            FileType::File
        };

        let is_hidden = name.starts_with('.');

        Ok(Self {
            path: path.to_path_buf(),
            name,
            file_type,
            size: metadata.len(),
            modified: metadata.modified()?,
            permissions: Self::format_permissions(&metadata),
            is_hidden,
        })
    }

    pub fn read_directory(dir: &Path, show_hidden: bool) -> Result<Vec<Self>> {
        let mut entries = Vec::new();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if let Ok(file_entry) = Self::from_path(&path) {
                if show_hidden || !file_entry.is_hidden {
                    entries.push(file_entry);
                }
            }
        }

        // Sort: directories first, then by name
        entries.sort_by(|a, b| {
            match (&a.file_type, &b.file_type) {
                (FileType::Directory, FileType::Directory) => a.name.cmp(&b.name),
                (FileType::Directory, _) => std::cmp::Ordering::Less,
                (_, FileType::Directory) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            }
        });

        Ok(entries)
    }

    #[cfg(unix)]
    fn format_permissions(metadata: &Metadata) -> String {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode();
        let user = triplet(mode, 0o100, 0o200, 0o400);
        let group = triplet(mode, 0o010, 0o020, 0o040);
        let other = triplet(mode, 0o001, 0o002, 0o004);
        format!("{}{}{}", user, group, other)
    }

    #[cfg(windows)]
    fn format_permissions(metadata: &Metadata) -> String {
        if metadata.permissions().readonly() {
            "r--".to_string()
        } else {
            "rw-".to_string()
        }
    }

    pub fn format_size(&self) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = self.size as f64;
        let mut unit_idx = 0;

        while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
            size /= 1024.0;
            unit_idx += 1;
        }

        if unit_idx == 0 {
            format!("{} {}", size as u64, UNITS[unit_idx])
        } else {
            format!("{:.1} {}", size, UNITS[unit_idx])
        }
    }

    pub fn format_modified(&self) -> String {
        let datetime: DateTime<Local> = self.modified.into();
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn extension(&self) -> Option<String> {
        self.path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
    }
}

#[cfg(unix)]
fn triplet(mode: u32, read: u32, write: u32, execute: u32) -> String {
    format!(
        "{}{}{}",
        if mode & read != 0 { "r" } else { "-" },
        if mode & write != 0 { "w" } else { "-" },
        if mode & execute != 0 { "x" } else { "-" }
    )
}