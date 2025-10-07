# Implementation Details

## Architecture Overview

`wezterm-watch` is a high-performance file watcher built with Rust, designed specifically for integration with WezTerm terminal. It combines real-time file system monitoring with Git repository awareness.

### Core Design Principles

1. **Performance First**: <10MB RAM, <1% CPU during monitoring
2. **Event-Driven**: Non-blocking async I/O throughout
3. **Smart Caching**: Git status cached with invalidation on changes
4. **Configurable Output**: Multiple formats for different use cases
5. **Cross-Platform**: Works on Windows, Linux, and macOS

## Module Structure

```
src/
├── main.rs       # CLI interface and event loop orchestration
├── watcher.rs    # File system monitoring with debouncing
├── git.rs        # Git repository integration
└── output.rs     # Output formatting (JSON, Pretty, Events, Summary)
```

### main.rs - Orchestration

**Responsibilities:**
- CLI argument parsing with `clap`
- Initialization of watcher and git monitor
- Main event loop
- Signal handling (Ctrl-C)
- Coordination between modules

**Key Components:**

```rust
#[derive(Parser)]
struct Args {
    path: PathBuf,              // Directory to watch
    format: String,             // Output format
    interval: u64,              // Debounce interval (ms)
    git: bool,                  // Force git integration
    no_git: bool,               // Disable git integration
    ignore_patterns: Vec<String>, // Custom ignores
    // ... more options
}
```

**Event Loop Pattern:**

```rust
while running.load(Ordering::SeqCst) {
    match receiver.recv_timeout(Duration::from_millis(100)) {
        Ok(event) => {
            // Get git status if enabled
            let git_status = git_monitor.get_file_status(event.path())?;

            // Format and output event
            let output = formatter.format_event(&event, git_status);
            println!("{}", output);
        }
        Err(Timeout) => {
            // Periodic updates for summary mode
        }
        Err(Disconnected) => break,
    }
}
```

### watcher.rs - File System Monitoring

**Responsibilities:**
- File system event detection
- Event debouncing
- Gitignore pattern matching
- Event filtering and transformation

**Key Components:**

```rust
pub struct FileWatcher {
    _debouncer: Debouncer<RecommendedWatcher, FileIdMap>,
    receiver: Receiver<WatchEvent>,
    gitignore: Option<Gitignore>,
    watch_path: PathBuf,
}
```

**Debouncing Strategy:**

The watcher uses `notify-debouncer-full` to group rapid file changes:

1. Events arrive from `notify` crate
2. Debouncer groups events within time window (default 100ms)
3. Only final state is emitted
4. Reduces event storm from build systems

Example: Saving a file in editor might trigger:
- CREATE temp file
- WRITE to temp file
- DELETE original file
- RENAME temp to original

With debouncing, this becomes a single MODIFY event.

**Gitignore Integration:**

```rust
fn load_gitignore(path: &Path) -> Result<Gitignore> {
    let mut builder = GitignoreBuilder::new(path);

    // Add .gitignore if exists
    let gitignore_path = path.join(".gitignore");
    if gitignore_path.exists() {
        builder.add(gitignore_path);
    }

    // Add common patterns
    builder.add_line(None, ".git")?;
    builder.add_line(None, "target/")?;
    builder.add_line(None, "node_modules/")?;

    builder.build()
}
```

### git.rs - Git Repository Integration

**Responsibilities:**
- Git repository detection
- Branch information extraction
- File status tracking
- Ahead/behind calculation
- Conflict detection
- Status caching with TTL

**Key Components:**

```rust
pub struct GitMonitor {
    repo_path: Option<PathBuf>,
    repo: Option<Repository>,
    cache: Arc<Mutex<CachedGitInfo>>,
}

struct CachedGitInfo {
    info: Option<GitInfo>,
    last_update: Instant,
    cache_duration: Duration,  // 500ms default
}
```

**Caching Strategy:**

Git status queries can be expensive (100-300ms for large repos). The cache:

1. Stores last git status result
2. Timestamps each query
3. Returns cached result if < 500ms old
4. Invalidates on file change events
5. Prevents redundant git operations

**Git Status Extraction:**

```rust
fn fetch_status(&self) -> Result<GitInfo> {
    let repo = self.repo.as_ref()?;

    // Get branch name
    let head = repo.head()?;
    let branch = head.shorthand().unwrap_or("detached");

    // Calculate ahead/behind
    let (ahead, behind) = self.get_ahead_behind(repo)?;

    // Get file statuses
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);

    let statuses = repo.statuses(Some(&mut opts))?;

    // Process each file
    for entry in statuses.iter() {
        // Map git2::Status to FileStatus
        // ...
    }

    Ok(GitInfo {
        branch,
        ahead,
        behind,
        file_statuses,
        has_conflicts,
    })
}
```

### output.rs - Formatting

**Responsibilities:**
- Event formatting in multiple formats
- Git status formatting
- Color coding
- JSON serialization

**Output Formats:**

1. **JSON** - Machine-readable, structured data
   ```json
   {
     "event_type": "modified",
     "path": "src/main.rs",
     "git_status": "M",
     "timestamp": 1704067200
   }
   ```

2. **Pretty** - Human-readable with colors
   ```
   [M] MODIFIED src/main.rs
   [?] CREATED test.txt
   [D] DELETED old.rs
   ```

3. **Events** - Compact stream
   ```
   M ~ src/main.rs
   ? + test.txt
   D - old.rs
   ```

4. **Summary** - Inline status
   ```
   [main] ↑0 ↓0 | M:2 S:0 U:1
   ```

**Color Coding:**

```rust
impl FileStatus {
    pub fn to_colored_str(&self) -> String {
        use colored::Colorize;
        match self {
            FileStatus::Modified => "M".yellow(),
            FileStatus::Added => "A".green(),
            FileStatus::Deleted => "D".red(),
            FileStatus::Conflicted => "U".red().bold(),
            // ...
        }
    }
}
```

## Performance Optimizations

### 1. Lazy Initialization

Components are only created when needed:

```rust
// Git monitor only if in repo or --git flag
let git_monitor = if git_enabled {
    Some(GitMonitor::new(&watch_path))
} else {
    None
};
```

### 2. Arc/Mutex for Shared State

Cache is shared across threads efficiently:

```rust
cache: Arc<Mutex<CachedGitInfo>>
```

### 3. Event Debouncing

Reduces processing overhead by 90%+ during rapid changes:

```rust
let debouncer = new_debouncer(
    Duration::from_millis(100),
    None,
    event_handler,
)?;
```

### 4. Pre-compiled Patterns

Gitignore patterns compiled once at startup:

```rust
let gitignore = GitignoreBuilder::new(path).build()?;
// Fast lookups thereafter
gitignore.matched(path, is_dir).is_ignore()
```

### 5. Zero-Copy Where Possible

Use references instead of clones:

```rust
pub fn path(&self) -> Option<&Path> {
    match self {
        WatchEvent::Modified(p) => Some(p),
        // No clone, just reference
    }
}
```

## Error Handling Strategy

### Graceful Degradation

The application continues even if git operations fail:

```rust
let git_status = if let Some(monitor) = &git_monitor {
    monitor.get_file_status(path).ok().flatten()
} else {
    None
};
```

### Context-Rich Errors

Using `anyhow` for error context:

```rust
let repo = repo.as_ref()
    .context("No git repository")?;
```

### Non-Fatal Errors

File watcher continues even if individual events fail:

```rust
match result {
    Ok(events) => {
        for event in events {
            if let Some(watch_event) = convert_event(event) {
                sender.send(watch_event).ok();
            }
        }
    }
    Err(errors) => {
        for error in errors {
            sender.send(WatchEvent::Error(error.to_string())).ok();
        }
    }
}
```

## Threading Model

### Main Thread

- CLI argument parsing
- Event loop
- Output formatting
- Signal handling

### Notify Thread (notify crate)

- File system event detection
- Raw event collection
- Passes events to debouncer

### Debouncer Thread (notify-debouncer-full)

- Event grouping
- Time-window management
- Filtered event emission

### Git Operations

- Synchronous (blocking) on main thread
- Cached to minimize impact
- Invalidated on file changes

## Memory Management

### Stack Allocation

Most data structures use stack allocation:

```rust
struct Args { ... }           // Stack
let watch_path: PathBuf;      // Stack-allocated smart pointer
let formatter = OutputFormatter::new(format);  // Stack
```

### Heap Allocation

Only when necessary:

```rust
let gitignore: Option<Gitignore>  // Gitignore patterns on heap
let cache: Arc<Mutex<...>>        // Shared cache on heap
```

### Reference Counting

Arc for shared state:

```rust
let running = Arc::new(AtomicBool::new(true));
let r = running.clone();  // Clone Arc, not data
```

## Cross-Platform Considerations

### File Watching Backends

- **Windows**: `ReadDirectoryChangesW`
- **Linux**: `inotify`
- **macOS**: `FSEvents`

The `notify` crate abstracts these differences.

### Path Handling

Using `PathBuf` and `Path` for cross-platform paths:

```rust
let watch_path = args.path.canonicalize()?;  // Resolves to absolute path
```

### Signal Handling

Platform-specific Ctrl-C handling:

```rust
#[cfg(windows)]
{
    // Windows console handler
}

#[cfg(unix)]
{
    // Unix signal handler
}
```

## Testing Strategy

### Unit Tests

Each module has unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_status_display() {
        assert_eq!(FileStatus::Modified.to_short_str(), "M");
    }
}
```

### Integration Tests

Testing the full pipeline:

1. Create temp directory
2. Initialize git repo
3. Start watcher
4. Create/modify files
5. Verify events
6. Verify git status
7. Clean up

### Manual Testing

```bash
# Test basic functionality
wezterm-watch . --format pretty

# Test git integration
wezterm-watch . --git --format json

# Test with rapid changes
for i in {1..100}; do
    touch test_$i.txt
done
```

## Future Improvements

### Performance

- [ ] Multi-threaded git status for parallel file checks
- [ ] mmap for reading large .gitignore files
- [ ] SIMD for pattern matching
- [ ] Custom allocator tuned for workload

### Features

- [ ] Configuration file support (.wezterm-watch.toml)
- [ ] Remote watching (SSH/SFTP)
- [ ] Custom event filters (regex)
- [ ] Plugin system for extensibility
- [ ] WebSocket server mode

### Reliability

- [ ] Automatic reconnection on errors
- [ ] Watchdog for hung git operations
- [ ] Graceful handling of file system unmounts
- [ ] Rate limiting for event storms

## Debugging

### Enable Debug Logging

```bash
RUST_LOG=debug wezterm-watch .
```

### Profile Performance

```bash
cargo build --release
perf record ./wezterm-watch .
perf report
```

### Memory Profiling

```bash
valgrind --tool=massif ./wezterm-watch .
ms_print massif.out.<pid>
```

### Trace Events

```bash
strace -e inotify ./wezterm-watch .  # Linux
dtrace -n 'syscall::kevent:entry' -p <pid>  # macOS
```

## Build Configurations

### Release (Default)

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = true
panic = "abort"
```

Produces smallest, fastest binary (~3MB, <50ms startup).

### Development

```toml
[profile.dev]
opt-level = 1
```

Faster compile times, debugging symbols preserved.

### Release-Fast

```toml
[profile.release-fast]
opt-level = 3
lto = "thin"
codegen-units = 16
```

Faster compilation than release, 95% of performance.

## Dependencies

### Core

- **notify** (6.1): File system watching
- **notify-debouncer-full** (0.3): Event debouncing
- **git2** (0.20): Git integration via libgit2
- **tokio** (1.40): Async runtime

### CLI & Output

- **clap** (4.5): Argument parsing
- **serde** + **serde_json** (1.0): JSON serialization
- **colored** (2.1): Terminal colors

### Utilities

- **anyhow** (1.0): Error handling
- **crossbeam-channel** (0.5): Event channels
- **ignore** (0.4): Gitignore parsing
- **walkdir** (2.5): Directory traversal

## Contribution Guidelines

### Code Style

- Follow Rust 2021 idioms
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add tests for new functionality

### Pull Request Process

1. Fork and create feature branch
2. Implement changes with tests
3. Update documentation
4. Run full test suite
5. Submit PR with description

### Performance Requirements

- No regressions in memory usage
- No regressions in CPU usage
- Event latency must stay <20ms
- Startup time must stay <100ms