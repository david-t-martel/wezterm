use git2::{Repository, Status, StatusOptions};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct GitStatus {
    pub statuses: HashMap<PathBuf, GitFileStatus>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GitFileStatus {
    Modified,
    Added,
    Deleted,
    Renamed,
    Untracked,
    Ignored,
}

impl GitStatus {
    pub fn from_repo(path: &Path) -> Option<Self> {
        let repo = Repository::discover(path).ok()?;
        let mut statuses = HashMap::new();

        let mut opts = StatusOptions::new();
        opts.include_untracked(true);
        opts.recurse_untracked_dirs(true);

        if let Ok(git_statuses) = repo.statuses(Some(&mut opts)) {
            for entry in git_statuses.iter() {
                if let Some(path) = entry.path() {
                    let full_path = repo.workdir()?.join(path);
                    let status = Self::parse_status(entry.status());
                    statuses.insert(full_path, status);
                }
            }
        }

        Some(Self { statuses })
    }

    fn parse_status(status: Status) -> GitFileStatus {
        if status.contains(Status::WT_NEW) || status.contains(Status::INDEX_NEW) {
            GitFileStatus::Added
        } else if status.contains(Status::WT_MODIFIED) || status.contains(Status::INDEX_MODIFIED) {
            GitFileStatus::Modified
        } else if status.contains(Status::WT_DELETED) || status.contains(Status::INDEX_DELETED) {
            GitFileStatus::Deleted
        } else if status.contains(Status::WT_RENAMED) || status.contains(Status::INDEX_RENAMED) {
            GitFileStatus::Renamed
        } else if status.contains(Status::IGNORED) {
            GitFileStatus::Ignored
        } else {
            GitFileStatus::Untracked
        }
    }

    pub fn get_status(&self, path: &Path) -> Option<GitFileStatus> {
        self.statuses.get(path).copied()
    }

    pub fn get_indicator(&self, path: &Path) -> Option<&str> {
        self.get_status(path).map(|status| match status {
            GitFileStatus::Modified => "M",
            GitFileStatus::Added => "A",
            GitFileStatus::Deleted => "D",
            GitFileStatus::Renamed => "R",
            GitFileStatus::Untracked => "?",
            GitFileStatus::Ignored => "!",
        })
    }
}