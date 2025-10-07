use crate::git::{FileStatus, GitInfo};
use crate::watcher::WatchEvent;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Json,
    Pretty,
    Events,
    Summary,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json" => Some(Self::Json),
            "pretty" => Some(Self::Pretty),
            "events" => Some(Self::Events),
            "summary" => Some(Self::Summary),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonOutput {
    pub event_type: String,
    pub path: Option<PathBuf>,
    pub from_path: Option<PathBuf>,
    pub to_path: Option<PathBuf>,
    pub git_status: Option<String>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonSummary {
    pub git_branch: Option<String>,
    pub git_ahead: Option<usize>,
    pub git_behind: Option<usize>,
    pub has_conflicts: bool,
    pub modified_files: usize,
    pub untracked_files: usize,
    pub staged_files: usize,
    pub total_files: usize,
}

pub struct OutputFormatter {
    format: OutputFormat,
}

impl OutputFormatter {
    pub fn new(format: OutputFormat) -> Self {
        Self { format }
    }

    pub fn format_event(&self, event: &WatchEvent, git_status: Option<&FileStatus>) -> String {
        match self.format {
            OutputFormat::Json => self.format_json(event, git_status),
            OutputFormat::Pretty => self.format_pretty(event, git_status),
            OutputFormat::Events => self.format_events(event, git_status),
            OutputFormat::Summary => String::new(), // Summary doesn't output per-event
        }
    }

    pub fn format_git_info(&self, info: &GitInfo) -> String {
        match self.format {
            OutputFormat::Json => self.format_git_json(info),
            OutputFormat::Pretty => self.format_git_pretty(info),
            OutputFormat::Summary => self.format_git_summary(info),
            OutputFormat::Events => String::new(), // Events mode doesn't show git info
        }
    }

    fn format_json(&self, event: &WatchEvent, git_status: Option<&FileStatus>) -> String {
        let output = match event {
            WatchEvent::Created(path) => JsonOutput {
                event_type: "created".to_string(),
                path: Some(path.clone()),
                from_path: None,
                to_path: None,
                git_status: git_status.map(|s| s.to_short_str().to_string()),
                timestamp: Self::current_timestamp(),
            },
            WatchEvent::Modified(path) => JsonOutput {
                event_type: "modified".to_string(),
                path: Some(path.clone()),
                from_path: None,
                to_path: None,
                git_status: git_status.map(|s| s.to_short_str().to_string()),
                timestamp: Self::current_timestamp(),
            },
            WatchEvent::Deleted(path) => JsonOutput {
                event_type: "deleted".to_string(),
                path: Some(path.clone()),
                from_path: None,
                to_path: None,
                git_status: git_status.map(|s| s.to_short_str().to_string()),
                timestamp: Self::current_timestamp(),
            },
            WatchEvent::Renamed { from, to } => JsonOutput {
                event_type: "renamed".to_string(),
                path: None,
                from_path: Some(from.clone()),
                to_path: Some(to.clone()),
                git_status: git_status.map(|s| s.to_short_str().to_string()),
                timestamp: Self::current_timestamp(),
            },
            WatchEvent::Error(_msg) => JsonOutput {
                event_type: "error".to_string(),
                path: None,
                from_path: None,
                to_path: None,
                git_status: None,
                timestamp: Self::current_timestamp(),
            },
        };

        serde_json::to_string(&output).unwrap_or_default()
    }

    fn format_pretty(&self, event: &WatchEvent, git_status: Option<&FileStatus>) -> String {
        let git_indicator = if let Some(status) = git_status {
            format!("[{}] ", status.to_colored_str())
        } else {
            String::new()
        };

        match event {
            WatchEvent::Created(path) => {
                format!(
                    "{}{} {}",
                    git_indicator,
                    "CREATED".green().bold(),
                    path.display()
                )
            }
            WatchEvent::Modified(path) => {
                format!(
                    "{}{} {}",
                    git_indicator,
                    "MODIFIED".yellow().bold(),
                    path.display()
                )
            }
            WatchEvent::Deleted(path) => {
                format!(
                    "{}{} {}",
                    git_indicator,
                    "DELETED".red().bold(),
                    path.display()
                )
            }
            WatchEvent::Renamed { from, to } => {
                format!(
                    "{}{} {} -> {}",
                    git_indicator,
                    "RENAMED".blue().bold(),
                    from.display(),
                    to.display()
                )
            }
            WatchEvent::Error(msg) => {
                format!("{} {}", "ERROR".red().bold(), msg)
            }
        }
    }

    fn format_events(&self, event: &WatchEvent, git_status: Option<&FileStatus>) -> String {
        let git_indicator = if let Some(status) = git_status {
            status.to_short_str()
        } else {
            " "
        };

        match event {
            WatchEvent::Created(path) => {
                format!("{} + {}", git_indicator, path.display())
            }
            WatchEvent::Modified(path) => {
                format!("{} ~ {}", git_indicator, path.display())
            }
            WatchEvent::Deleted(path) => {
                format!("{} - {}", git_indicator, path.display())
            }
            WatchEvent::Renamed { from, to } => {
                format!("{} R {} -> {}", git_indicator, from.display(), to.display())
            }
            WatchEvent::Error(msg) => {
                format!("! {}", msg)
            }
        }
    }

    fn format_git_json(&self, info: &GitInfo) -> String {
        let summary = JsonSummary {
            git_branch: Some(info.branch.clone()),
            git_ahead: Some(info.ahead),
            git_behind: Some(info.behind),
            has_conflicts: info.has_conflicts,
            modified_files: info
                .file_statuses
                .values()
                .filter(|s| **s == FileStatus::Modified)
                .count(),
            untracked_files: info
                .file_statuses
                .values()
                .filter(|s| **s == FileStatus::Untracked)
                .count(),
            staged_files: info
                .file_statuses
                .values()
                .filter(|s| **s == FileStatus::Staged)
                .count(),
            total_files: info.file_statuses.len(),
        };

        serde_json::to_string(&summary).unwrap_or_default()
    }

    fn format_git_pretty(&self, info: &GitInfo) -> String {
        let mut output = String::new();

        // Branch info
        output.push_str(&format!(
            "{} {}\n",
            "Branch:".cyan().bold(),
            info.branch.bright_white()
        ));

        // Ahead/Behind
        if info.ahead > 0 || info.behind > 0 {
            output.push_str(&format!(
                "{} {} ahead, {} behind\n",
                "Status:".cyan().bold(),
                format!("{}", info.ahead).green(),
                format!("{}", info.behind).red()
            ));
        }

        // Conflicts
        if info.has_conflicts {
            output.push_str(&format!("{}\n", "CONFLICTS DETECTED".red().bold()));
        }

        // File counts
        let modified = info
            .file_statuses
            .values()
            .filter(|s| **s == FileStatus::Modified)
            .count();
        let untracked = info
            .file_statuses
            .values()
            .filter(|s| **s == FileStatus::Untracked)
            .count();
        let staged = info
            .file_statuses
            .values()
            .filter(|s| **s == FileStatus::Staged)
            .count();

        output.push_str(&format!(
            "{} {} modified, {} staged, {} untracked\n",
            "Files:".cyan().bold(),
            modified,
            staged,
            untracked
        ));

        output
    }

    fn format_git_summary(&self, info: &GitInfo) -> String {
        let modified = info
            .file_statuses
            .values()
            .filter(|s| **s == FileStatus::Modified)
            .count();
        let untracked = info
            .file_statuses
            .values()
            .filter(|s| **s == FileStatus::Untracked)
            .count();
        let staged = info
            .file_statuses
            .values()
            .filter(|s| **s == FileStatus::Staged)
            .count();

        format!(
            "[{}] ↑{} ↓{} | M:{} S:{} U:{}{}",
            info.branch,
            info.ahead,
            info.behind,
            modified,
            staged,
            untracked,
            if info.has_conflicts {
                " [CONFLICT]"
            } else {
                ""
            }
        )
    }

    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_from_str() {
        assert_eq!(OutputFormat::from_str("json"), Some(OutputFormat::Json));
        assert_eq!(OutputFormat::from_str("pretty"), Some(OutputFormat::Pretty));
        assert_eq!(OutputFormat::from_str("events"), Some(OutputFormat::Events));
        assert_eq!(OutputFormat::from_str("invalid"), None);
    }
}
