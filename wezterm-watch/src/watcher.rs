use anyhow::{Context, Result};
use crossbeam_channel::{Receiver, Sender};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, FileIdMap};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum WatchEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
    #[allow(dead_code)] // Reserved for future rename detection
    Renamed { from: PathBuf, to: PathBuf },
    Error(String),
}

impl WatchEvent {
    pub fn path(&self) -> Option<&Path> {
        match self {
            WatchEvent::Created(p) | WatchEvent::Modified(p) | WatchEvent::Deleted(p) => Some(p),
            WatchEvent::Renamed { to, .. } => Some(to),
            WatchEvent::Error(_) => None,
        }
    }

    #[cfg(test)]
    pub fn event_type(&self) -> &str {
        match self {
            WatchEvent::Created(_) => "created",
            WatchEvent::Modified(_) => "modified",
            WatchEvent::Deleted(_) => "deleted",
            WatchEvent::Renamed { .. } => "renamed",
            WatchEvent::Error(_) => "error",
        }
    }
}

pub struct FileWatcher {
    _debouncer: Debouncer<RecommendedWatcher, FileIdMap>,
    receiver: Receiver<WatchEvent>,
    #[allow(dead_code)] // Used for filtering, stored for potential future use
    gitignore: Option<Gitignore>,
    watch_path: PathBuf,
}

impl FileWatcher {
    pub fn new(
        path: PathBuf,
        debounce_ms: u64,
        use_gitignore: bool,
        custom_ignores: Vec<String>,
    ) -> Result<Self> {
        let (tx, rx) = crossbeam_channel::unbounded();

        // Load gitignore rules
        let gitignore = if use_gitignore {
            Self::load_gitignore(&path, custom_ignores)?
        } else if !custom_ignores.is_empty() {
            Self::build_custom_ignore(&path, custom_ignores)?
        } else {
            None
        };

        let tx_clone = tx.clone();
        let gitignore_clone = gitignore.clone();
        let watch_path_clone = path.clone();

        let debouncer = new_debouncer(
            Duration::from_millis(debounce_ms),
            None,
            move |result: DebounceEventResult| {
                Self::handle_events(result, &tx_clone, &gitignore_clone, &watch_path_clone);
            },
        )
        .context("Failed to create debouncer")?;

        Ok(Self {
            _debouncer: debouncer,
            receiver: rx,
            gitignore,
            watch_path: path,
        })
    }

    pub fn watch(&mut self, recursive: bool) -> Result<()> {
        let mode = if recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        self._debouncer
            .watcher()
            .watch(&self.watch_path, mode)
            .context("Failed to start watching")?;

        Ok(())
    }

    pub fn receiver(&self) -> &Receiver<WatchEvent> {
        &self.receiver
    }

    fn handle_events(
        result: DebounceEventResult,
        sender: &Sender<WatchEvent>,
        gitignore: &Option<Gitignore>,
        base_path: &Path,
    ) {
        match result {
            Ok(events) => {
                for event in events {
                    if let Some(watch_event) =
                        Self::convert_event(event.event, gitignore, base_path)
                    {
                        let _ = sender.send(watch_event);
                    }
                }
            }
            Err(errors) => {
                for error in errors {
                    let _ = sender.send(WatchEvent::Error(error.to_string()));
                }
            }
        }
    }

    fn convert_event(
        event: Event,
        gitignore: &Option<Gitignore>,
        base_path: &Path,
    ) -> Option<WatchEvent> {
        // Filter ignored files
        if let Some(gi) = gitignore {
            for path in &event.paths {
                if let Ok(rel_path) = path.strip_prefix(base_path) {
                    if gi.matched(rel_path, path.is_dir()).is_ignore() {
                        return None;
                    }
                }
            }
        }

        match event.kind {
            EventKind::Create(_) => event.paths.first().map(|path| WatchEvent::Created(path.clone())),
            EventKind::Modify(_) => event.paths.first().map(|path| WatchEvent::Modified(path.clone())),
            EventKind::Remove(_) => event.paths.first().map(|path| WatchEvent::Deleted(path.clone())),
            EventKind::Any => event.paths.first().map(|path| WatchEvent::Modified(path.clone())),
            _ => None,
        }
    }

    fn load_gitignore(path: &Path, custom_ignores: Vec<String>) -> Result<Option<Gitignore>> {
        let mut builder = GitignoreBuilder::new(path);

        // Add .gitignore if it exists
        let gitignore_path = path.join(".gitignore");
        if gitignore_path.exists() {
            builder.add(gitignore_path);
        }

        // Add common ignore patterns
        builder.add_line(None, ".git")?;
        builder.add_line(None, "target/")?;
        builder.add_line(None, "node_modules/")?;
        builder.add_line(None, "*.swp")?;
        builder.add_line(None, "*.tmp")?;
        builder.add_line(None, ".DS_Store")?;

        // Add custom patterns
        for pattern in custom_ignores {
            builder.add_line(None, &pattern)?;
        }

        Ok(Some(builder.build()?))
    }

    fn build_custom_ignore(path: &Path, patterns: Vec<String>) -> Result<Option<Gitignore>> {
        let mut builder = GitignoreBuilder::new(path);

        for pattern in patterns {
            builder.add_line(None, &pattern)?;
        }

        Ok(Some(builder.build()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watch_event_type() {
        let event = WatchEvent::Created(PathBuf::from("test.txt"));
        assert_eq!(event.event_type(), "created");

        let event = WatchEvent::Modified(PathBuf::from("test.txt"));
        assert_eq!(event.event_type(), "modified");
    }
}
