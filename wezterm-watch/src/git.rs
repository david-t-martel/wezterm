use anyhow::{Context, Result};
use git2::{Repository, Status, StatusOptions};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileStatus {
    Modified,
    Added,
    Deleted,
    Renamed,
    Untracked,
    Conflicted,
    Staged,
    Unknown,
}

impl FileStatus {
    pub fn to_short_str(&self) -> &str {
        match self {
            FileStatus::Modified => "M",
            FileStatus::Added => "A",
            FileStatus::Deleted => "D",
            FileStatus::Renamed => "R",
            FileStatus::Untracked => "?",
            FileStatus::Conflicted => "U",
            FileStatus::Staged => "S",
            FileStatus::Unknown => " ",
        }
    }

    pub fn to_colored_str(&self) -> String {
        use colored::Colorize;
        match self {
            FileStatus::Modified => "M".yellow().to_string(),
            FileStatus::Added => "A".green().to_string(),
            FileStatus::Deleted => "D".red().to_string(),
            FileStatus::Renamed => "R".blue().to_string(),
            FileStatus::Untracked => "?".bright_black().to_string(),
            FileStatus::Conflicted => "U".red().bold().to_string(),
            FileStatus::Staged => "S".green().to_string(),
            FileStatus::Unknown => " ".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GitInfo {
    pub branch: String,
    pub ahead: usize,
    pub behind: usize,
    pub has_conflicts: bool,
    pub file_statuses: HashMap<PathBuf, FileStatus>,
}

pub struct GitMonitor {
    repo_path: Option<PathBuf>,
    repo: Option<Repository>,
    cache: Arc<Mutex<CachedGitInfo>>,
}

struct CachedGitInfo {
    info: Option<GitInfo>,
    last_update: Instant,
    cache_duration: Duration,
}

impl GitMonitor {
    pub fn new(path: &Path) -> Self {
        let (repo_path, repo) = Self::find_repository(path);

        Self {
            repo_path,
            repo,
            cache: Arc::new(Mutex::new(CachedGitInfo {
                info: None,
                last_update: Instant::now() - Duration::from_secs(10),
                cache_duration: Duration::from_millis(500),
            })),
        }
    }

    fn find_repository(path: &Path) -> (Option<PathBuf>, Option<Repository>) {
        Repository::discover(path)
            .ok()
            .map(|repo| {
                let workdir = repo.workdir().map(|p| p.to_path_buf());
                (workdir, Some(repo))
            })
            .unwrap_or((None, None))
    }

    pub fn is_git_repo(&self) -> bool {
        self.repo.is_some()
    }

    pub fn repo_root(&self) -> Option<&Path> {
        self.repo_path.as_deref()
    }

    pub fn get_status(&self) -> Result<GitInfo> {
        let mut cache = self.cache.lock().unwrap();

        // Return cached info if still valid
        if let Some(info) = &cache.info {
            if cache.last_update.elapsed() < cache.cache_duration {
                return Ok(info.clone());
            }
        }

        // Update cache
        let info = self.fetch_status()?;
        cache.info = Some(info.clone());
        cache.last_update = Instant::now();

        Ok(info)
    }

    pub fn invalidate_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.last_update = Instant::now() - Duration::from_secs(10);
    }

    fn fetch_status(&self) -> Result<GitInfo> {
        let repo = self.repo.as_ref().context("No git repository")?;

        // Get branch info
        let head = repo.head().context("Failed to get HEAD")?;
        let branch = if head.is_branch() {
            head.shorthand().unwrap_or("unknown").to_string()
        } else {
            "detached".to_string()
        };

        // Get ahead/behind counts
        let (ahead, behind) = self.get_ahead_behind(repo)?;

        // Get file statuses
        let mut opts = StatusOptions::new();
        opts.include_untracked(true);
        opts.recurse_untracked_dirs(false);

        let statuses = repo
            .statuses(Some(&mut opts))
            .context("Failed to get git status")?;

        let mut file_statuses = HashMap::new();
        let mut has_conflicts = false;

        for entry in statuses.iter() {
            let path = PathBuf::from(entry.path().unwrap_or(""));
            let status = entry.status();

            let file_status = if status.is_conflicted() {
                has_conflicts = true;
                FileStatus::Conflicted
            } else if status.is_index_new()
                || status.is_index_modified()
                || status.is_index_deleted()
            {
                FileStatus::Staged
            } else if status.is_wt_new() {
                FileStatus::Untracked
            } else if status.is_wt_modified() {
                FileStatus::Modified
            } else if status.is_wt_deleted() {
                FileStatus::Deleted
            } else if status.is_wt_renamed() || status.is_index_renamed() {
                FileStatus::Renamed
            } else {
                FileStatus::Unknown
            };

            file_statuses.insert(path, file_status);
        }

        Ok(GitInfo {
            branch,
            ahead,
            behind,
            has_conflicts,
            file_statuses,
        })
    }

    fn get_ahead_behind(&self, repo: &Repository) -> Result<(usize, usize)> {
        let head = repo.head()?;
        if !head.is_branch() {
            return Ok((0, 0));
        }

        let local_oid = head.target().context("Failed to get HEAD target")?;

        let branch = head.shorthand().context("Failed to get branch name")?;
        let upstream_name = format!("refs/remotes/origin/{}", branch);

        let upstream = match repo.find_reference(&upstream_name) {
            Ok(r) => r,
            Err(_) => return Ok((0, 0)),
        };

        let upstream_oid = upstream.target().context("Failed to get upstream target")?;

        let (ahead, behind) = repo.graph_ahead_behind(local_oid, upstream_oid)?;

        Ok((ahead, behind))
    }

    pub fn get_file_status(&self, path: &Path) -> Result<Option<FileStatus>> {
        let info = self.get_status()?;

        // Try exact match first
        if let Some(status) = info.file_statuses.get(path) {
            return Ok(Some(status.clone()));
        }

        // Try relative to repo root
        if let Some(repo_root) = self.repo_root() {
            if let Ok(rel_path) = path.strip_prefix(repo_root) {
                if let Some(status) = info.file_statuses.get(rel_path) {
                    return Ok(Some(status.clone()));
                }
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_status_display() {
        assert_eq!(FileStatus::Modified.to_short_str(), "M");
        assert_eq!(FileStatus::Added.to_short_str(), "A");
        assert_eq!(FileStatus::Deleted.to_short_str(), "D");
    }
}
