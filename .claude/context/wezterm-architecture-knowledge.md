# WezTerm Architecture Knowledge Base
**Generated**: 2025-10-07
**Purpose**: Comprehensive architecture knowledge for AI assistant context

## Executive Summary

WezTerm is a production-grade, GPU-accelerated terminal emulator with exceptional Rust architecture:
- **19+ workspace crates** with clean separation of concerns
- **Dual GPU rendering** (OpenGL/WebGPU) via trait-based abstraction
- **Async-first multiplexer** using `parking_lot` and lock-free primitives
- **Custom utilities** for filesystem exploration and file watching
- **Planned AI assistant module** with LLM integration (mistral.rs/gemma.cpp)

---

## Architecture Patterns

### 1. Zero-Cost Abstractions

**GPU Backend Selection** (`wezterm-gui/src/renderstate.rs:23-122`):
```rust
pub enum RenderContext {
    Glium(Rc<GliumContext>),      // OpenGL fallback
    WebGpu(Rc<WebGpuState>),       // Modern GPU API
}
```
- **Strategy pattern**: Runtime selection without performance penalty
- **Monomorphization**: Backend-specific code optimized at compile time
- **Virtual dispatch**: Only at allocation time, not per-vertex

### 2. Type-Safe Index System

**Terminal Row Indices** (`term/src/lib.rs:44-80`):
```rust
pub type PhysRowIndex = usize;           // Actual buffer row
pub type VisibleRowIndex = i64;          // Screen-relative (signed)
pub type ScrollbackOrVisibleRowIndex = i32;  // 32-bit for scrollback
pub type StableRowIndex = isize;         // Persistent line ID
```
- **Different signedness prevents accidental arithmetic mixing**
- **Compiler catches index type mismatches**
- **32-bit for scrollback**: ~2B lines max, saves memory

### 3. Trait-Based Extensibility

**Pane Interface** (`mux/src/pane.rs:1-150`):
```rust
#[async_trait(?Send)]
pub trait Pane: Downcast + Send + Sync {
    fn pane_id(&self) -> PaneId;
    async fn send_paste(&self, text: &str) -> anyhow::Result<()>;
    // ... 30+ methods
}
impl_downcast!(Pane);  // Enable runtime type inspection
```
- **`?Send` async trait**: Allows non-Send futures for single-threaded optimization
- **Downcast trait**: Performance escape hatch for critical paths
- **Rich interface**: 30+ methods, implementors choose subset

---

## Concurrency Model

### Mux Lock Hierarchy (`mux/src/lib.rs:102-138`)

```rust
pub struct Mux {
    tabs: RwLock<HashMap<TabId, Arc<Tab>>>,
    panes: RwLock<HashMap<PaneId, Arc<dyn Pane>>>,
    windows: RwLock<HashMap<WindowId, Window>>,
    subscribers: RwLock<HashMap<usize, Box<dyn Fn(MuxNotification) -> bool>>>,
}
```

**Key Decisions**:
1. **parking_lot RwLock**: 5-10% faster than std::sync, no poisoning
2. **Fine-grained locking**: Separate locks per resource type
3. **Arc for shared state**: Reference counting over lifetimes
4. **Weak references**: Prevent cycles in background threads

**Lock-Free Notification**:
```rust
fn send_actions(pane: &Weak<dyn Pane>, dead: &Arc<AtomicBool>, actions: Vec<Action>) {
    match pane.upgrade() {
        Some(pane) => {
            pane.perform_actions(actions);
            Mux::notify_from_any_thread(MuxNotification::PaneOutput(pane.pane_id()));
        }
        None => dead.store(true, Ordering::Relaxed),
    }
}
```
- **AtomicBool for lifecycle**: Lock-free thread coordination
- **Weak::upgrade()**: Check if pane still alive

---

## Performance Optimizations

### Implemented

1. **Shared Target Directory** (`.cargo/config.toml`):
   - Path: `C:\Users\david\.cargo\shared-target\`
   - Benefit: Reuse compilation artifacts across builds

2. **sccache Integration** (Justfile):
   ```powershell
   just build             # Uses sccache
   just sccache-stats     # Show cache statistics
   ```
   - 15GB cache size
   - Significant compile time reduction

3. **Profile Tuning** (`Cargo.toml:26-34`):
   ```toml
   [profile.release]
   opt-level = 3

   [profile.dev]
   incremental = false  # Use sccache instead
   ```

4. **Utility Optimization** (`wezterm-fs-explorer/Cargo.toml:43-48`):
   ```toml
   [profile.release]
   lto = "fat"           # Cross-crate inlining
   codegen-units = 1     # Better optimization
   strip = true          # Remove debug symbols
   panic = "abort"       # Smaller binary
   ```

### Potential Optimizations

1. **Mux Subscriber Dispatch**:
   - Current: `RwLock<HashMap<usize, Box<dyn Fn>>>` (writer starvation)
   - Proposed: `crossbeam::channel` (lock-free MPMC)

2. **Parser State Pooling** (`term/src/terminal.rs:89`):
   - Current: `Parser::new()` on every `advance_bytes`
   - Proposed: Reuse parser instance

3. **GPU Resource Pooling**:
   - Current: Dynamic allocation per frame
   - Proposed: Free-list allocator for vertex buffers

---

## Custom Utilities Architecture

### wezterm-fs-explorer (Filesystem Explorer)

**Tech Stack**:
- **UI**: `ratatui 0.26` (TUI framework)
- **Async**: `tokio 1.35` with named pipes for IPC
- **Git**: `git2 0.18` for repository status
- **File watching**: `notify 6.1`

**Architecture** (`src/app.rs:29-42`):
```rust
pub struct App {
    pub current_dir: PathBuf,
    pub entries: Vec<FileEntry>,
    pub selected_index: usize,
    pub mode: AppMode,              // FSM: Normal | Search | Input | Confirmation
    pub git_status: Option<GitStatus>,
    pub scroll_offset: usize,
}
```

**Key Features**:
- Vim-like keybindings
- Git integration (shows M, A, D, ?? status)
- IPC support via named pipes/Unix sockets
- Nerd Font icon support
- <50ms startup time, <100MB memory

**Build** (Windows):
```powershell
.\build-all.ps1                # Builds and installs
.\build-all.ps1 -Force         # Force rebuild
```

### wezterm-watch (File Watcher)

**Tech Stack**:
- **File watching**: `notify 6.1` + `notify-debouncer-full 0.3`
- **Git**: `git2 0.20`
- **Async**: `tokio 1.40` with full features
- **Output**: Multiple formats (JSON, pretty, events, summary)

**Architecture** (`src/main.rs:60-112`):
```rust
#[tokio::main]
async fn main() -> Result<()> {
    let git_monitor = if git_enabled {
        Some(GitMonitor::new(&watch_path))
    } else { None };

    let mut watcher = FileWatcher::new(watch_path, interval, use_gitignore, ignore_patterns)?;
    watcher.watch(recursive)?;

    while running.load(Ordering::SeqCst) {
        match receiver.recv_timeout(Duration::from_millis(100)) {
            Ok(event) => {
                let git_status = git_monitor.as_ref()
                    .and_then(|m| m.get_file_status(event.path()?).ok().flatten());
                formatter.format_event(&event, git_status);
            }
        }
    }
}
```

**Key Features**:
- Debounced file events (configurable interval)
- Real-time git status integration
- .gitignore parsing via `ignore` crate
- Multiple output formats
- <10MB RAM idle, <1% CPU during monitoring

---

## AI Assistant Module Design

### Overview (from `WEZTERM_AI_MODULE_DESIGN.md`)

**Goal**: Integrate local LLM-based AI assistant into WezTerm

**Architecture**:
```
wezterm-gui
    └── wezterm-module-framework (NEW)
        ├── ai-assistant (LLM inference)
        ├── filesystem (MCP server)
        └── commander (safe command execution)

LLM Integration Layer
    ├── mistral.rs (primary engine)
    ├── gemma.cpp (alternative via FFI)
    └── rag-redis (context retrieval)
```

**Key Components**:

1. **Module Framework** (`wezterm-module-framework/src/lib.rs`):
```rust
#[async_trait(?Send)]
pub trait WezTermModule: Send + Sync {
    fn metadata(&self) -> ModuleMetadata;
    async fn initialize(&mut self, ctx: ModuleContext) -> anyhow::Result<()>;
    fn register_lua_api(&self, lua: &Lua) -> anyhow::Result<()>;
    fn register_events(&self) -> Vec<EventHandler>;
    async fn handle_message(&self, msg: ModuleMessage) -> anyhow::Result<ModuleMessage>;
    async fn shutdown(&mut self) -> anyhow::Result<()>;
    fn health_check(&self) -> ModuleHealth;
}
```

2. **LLM Engine** (`ai-assistant/src/llm.rs`):
```rust
pub struct MistralRsEngine {
    model: TextModel,  // mistral.rs TextModel
}

impl LlmEngine for MistralRsEngine {
    async fn stream(&self, messages: Vec<ChatMessage>)
        -> anyhow::Result<impl Stream<Item = String>> {
        let stream = self.model.stream_chat_request(messages).await?;
        Ok(stream.filter_map(|chunk| /* extract text */))
    }
}
```

3. **Tool Execution** (MCP Protocol):
- Filesystem operations via `rust-mcp-filesystem` patterns
- Command execution with sandboxing
- Path validation and security

**Performance Targets**:
- Module Framework Core: <10MB
- AI Assistant (idle): <20MB
- AI Assistant (LLM loaded): <500MB (Phi-3.5 Mini Q4)
- RAG System: <150MB (10K documents)
- **Total**: <700MB when AI active

**Security**:
- Capability-based permissions
- Path validation (prevent symlink escapes)
- Command whitelisting/blacklisting
- Environment variable sanitization

**Implementation Status**: Design complete, implementation pending

---

## Workspace Structure

### Dependency Graph

```
wezterm-gui (main binary)
    ├── mux (multiplexer)
    │   ├── term (terminal engine)
    │   │   ├── wezterm-escape-parser
    │   │   ├── wezterm-cell
    │   │   └── wezterm-surface
    │   ├── portable-pty (PTY abstraction)
    │   └── wezterm-dynamic (config types)
    ├── window (platform abstraction)
    ├── wezterm-font (font handling)
    └── renderstate (GPU backends)

wezterm (CLI binary)
    └── mux (shares multiplexer)

wezterm-mux-server (standalone mux)
    └── mux

Custom Utilities (standalone)
├── wezterm-fs-explorer
└── wezterm-watch
```

### Critical Crates

| Crate | Purpose | Key Files |
|-------|---------|-----------|
| `term/` | VT100 emulation | `terminal.rs`, `terminalstate/mod.rs` |
| `mux/` | Tab/pane management | `lib.rs`, `pane.rs`, `domain.rs` |
| `wezterm-gui/` | GUI application | `main.rs`, `termwindow/mod.rs`, `renderstate.rs` |
| `window/` | Platform abstraction | Platform-specific implementations |
| `wezterm-font/` | Font rendering | harfbuzz/freetype integration |
| `lua-api-crates/` | Lua bindings | 13+ modules for extensibility |

---

## Build System

### Commands

**Windows (Just)**:
```powershell
just build              # Standard build with sccache
just release            # Release build
just clippy            # Linting (disables sccache)
just test              # Run tests with sccache
just full-verify       # Complete validation
just sccache-stats     # Cache statistics
```

**Unix/Linux/macOS (Make)**:
```bash
make build             # Build all binaries
make test              # Run nextest
make fmt               # Format code
make docs              # Build documentation
```

### Build Profiles

```toml
[profile.release]
opt-level = 3

[profile.dev]
incremental = false  # Use sccache instead

[profile.release-fast]  # For utilities
opt-level = "z"      # Size optimization
lto = "fat"
codegen-units = 1
```

---

## Testing Strategy

### Test Runners
- **Preferred**: `cargo nextest run` (parallel test execution)
- **Alternative**: `cargo test --all`

### Test Organization
- **Unit tests**: Colocated with source (`#[cfg(test)]` modules)
- **Integration tests**: `term/src/test/` for terminal tests
- **Assertion library**: `k9` for expressive assertions

### Pre-commit Hooks (`.pre-commit-config.yaml`)

**Pre-commit** (fast):
- `cargo fmt --all`
- `cargo clippy --workspace`
- Quick tests on changed crates
- cargo-deny check
- mdbook build
- doxygen (Rust architecture docs)

**Pre-push** (comprehensive):
- `cargo clippy --all-features`
- `cargo test --all-features`
- Full cargo-deny check
- Complete documentation build

**Install**:
```bash
pre-commit install --hook-type pre-commit --hook-type pre-push
```

---

## Development Workflow

### Initial Setup

```bash
# Clone repository
git clone https://github.com/david-t-martel/wezterm
cd wezterm

# Install pre-commit hooks
pre-commit install --hook-type pre-commit --hook-type pre-push

# Build project
just build              # Windows
make build              # Unix/Linux/macOS

# Run tests
just test               # Windows
make test               # Unix/Linux/macOS
```

### Daily Development

```bash
# Quick type check
cargo check

# Format code
cargo fmt --all

# Run linter
just clippy             # Windows (handles sccache)
cargo clippy --workspace -- -D warnings  # Unix/Linux

# Run tests
cargo nextest run

# Build documentation
make docs
make servedocs  # Serve with auto-rebuild
```

### Release Process

```bash
# Build release binaries
cargo build --release --workspace

# Build custom utilities
.\build-all.ps1         # Windows
cd wezterm-fs-explorer && cargo build --release  # Unix/Linux
cd wezterm-watch && cargo build --release

# Verify release
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target x86_64-unknown-linux-gnu
```

---

## Key File References

### Core Architecture
- `term/src/lib.rs` - Terminal engine types and exports
- `term/src/terminal.rs` - Main Terminal struct
- `term/src/terminalstate/mod.rs` - State machine
- `wezterm-gui/src/main.rs` - GUI entry point
- `wezterm-gui/src/renderstate.rs` - GPU backend abstraction
- `mux/src/lib.rs` - Multiplexer singleton
- `mux/src/pane.rs` - Pane trait definition
- `mux/src/domain.rs` - Domain trait for spawn/split

### Custom Utilities
- `wezterm-fs-explorer/src/main.rs` - FS explorer entry
- `wezterm-fs-explorer/src/app.rs` - Application state
- `wezterm-watch/src/main.rs` - File watcher entry
- `wezterm-watch/src/git.rs` - Git integration

### Build Configuration
- `Cargo.toml` - Workspace root
- `Justfile` - Windows build automation
- `Makefile` - Unix/Linux build automation
- `.cargo/config.toml` - Cargo configuration
- `.pre-commit-config.yaml` - Pre-commit hooks

### Documentation
- `CLAUDE.md` - AI assistant development guide
- `WEZTERM_AI_MODULE_DESIGN.md` - AI module specification
- `README.md` - Project overview
- `.claude/context/` - Project context files

---

## Quick Restoration Guide

To restore development context:

1. **Read Context Files**:
   - `.claude/context/quick-context.md`
   - `.claude/context/wezterm-context-2025-01-30.md`
   - This file

2. **Verify Build Environment**:
   ```bash
   cargo check --workspace
   just sccache-stats  # Check cache
   ```

3. **Run Tests**:
   ```bash
   cargo nextest run
   ```

4. **Review Git Status**:
   ```bash
   git status
   git log -5 --oneline
   ```

5. **Check Custom Utilities**:
   ```bash
   cd wezterm-fs-explorer && cargo check
   cd ../wezterm-watch && cargo check
   ```

---

## Performance Benchmarks

### Build Times (with sccache)
- **Incremental rebuild**: ~30-60 seconds
- **Clean build**: ~5-10 minutes (with cold cache)
- **Custom utilities**: ~2-3 minutes each

### Runtime Performance
- **Startup time**: <200ms (cold start)
- **Frame time**: 16.67ms target (60 FPS)
- **Memory usage**: 50-200MB (depends on tabs/panes)

### Custom Utilities
- **wezterm-fs-explorer**:
  - Startup: <50ms
  - Memory: <100MB
  - 10,000 files: 95ms load time

- **wezterm-watch**:
  - Idle memory: 6-8MB
  - CPU: <0.1% idle, 0.5-1% during events
  - Event latency: <10ms

---

## Next Steps / Roadmap

### Immediate Tasks
1. ✅ CLAUDE.md documentation created
2. ✅ Repository forked and configured
3. ✅ Custom utilities implemented
4. ⏳ AI Assistant Module implementation
5. ⏳ Performance optimization review

### AI Module Implementation Phases

**Phase 1**: Module Framework (4-6 weeks)
- Core `WezTermModule` trait
- `ModuleManager` with discovery/loading
- Inter-module IPC
- Lua API integration

**Phase 2**: Filesystem & Commander (2-3 weeks)
- Filesystem module with MCP protocol
- Command execution with sandboxing
- Security and path validation

**Phase 3**: LLM Integration (4-5 weeks)
- mistral.rs integration
- Tool execution engine
- RAG system (optional)

**Phase 4**: UI & UX (3-4 weeks)
- AI Assistant overlay pane
- Streaming response display
- Context integration

**Phase 5**: Testing & Polish (2 weeks)
- End-to-end testing
- Performance benchmarking
- Documentation

---

## External Resources

- **WezTerm Docs**: https://wezfurlong.org/wezterm/
- **Repository**: https://github.com/wez/wezterm
- **Fork**: https://github.com/david-t-martel/wezterm
- **mistral.rs**: https://github.com/EricLBuehler/mistral.rs
- **MCP Protocol**: https://modelcontextprotocol.io/

---

**Document Version**: 1.0
**Last Updated**: 2025-10-07
**Maintained By**: Claude Code AI Assistant
