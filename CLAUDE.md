# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

WezTerm is a GPU-accelerated cross-platform terminal emulator and multiplexer written in Rust. It uses wgpu for rendering, supports terminal multiplexing (panes, tabs, windows), includes an SSH client with native tabs, and features Lua configuration with hot-reloading.

## Build and Development Commands

### Building the Project

```bash
# Build all main binaries (wezterm, wezterm-gui, wezterm-mux-server, strip-ansi-escapes)
make build

# Build specific binary with release optimizations
cargo build --release -p wezterm-gui

# Quick type checking during development
cargo check
# Check specific no_std crates
cargo check -p wezterm-escape-parser
```

### Running Tests

```bash
# Run all tests using nextest (preferred test runner)
make test
# Or directly:
cargo nextest run

# Run tests for specific package
cargo nextest run -p wezterm-escape-parser

# Run single test with verbose output
cargo nextest run -p <package> <test_name>

# Run tests with standard cargo test
cargo test --all
```

### Code Formatting and Linting

```bash
# Format all code (requires nightly Rust)
make fmt
# Or directly:
cargo +nightly fmt

# Check formatting without applying changes
cargo +nightly fmt -- --check
```

### Documentation

```bash
# Build documentation locally
make docs
# Or directly:
ci/build-docs.sh

# Serve documentation with auto-rebuild on changes
make servedocs
# Or directly:
ci/build-docs.sh serve
```

### Development Iteration

```bash
# Quick type-check during development (fastest feedback loop)
cargo check

# Run in debug mode for testing changes
cargo run

# Run with backtrace for debugging panics
RUST_BACKTRACE=1 cargo run

# Debug with gdb
cargo build
gdb ./target/debug/wezterm
```

## High-Level Architecture

### Workspace Structure

This is a Cargo workspace with 19+ member crates organized by functionality:

**Core Terminal Engine** (platform-agnostic):
- `term/` - Core terminal emulator implementation
- `wezterm-cell/` - Terminal cell representation
- `wezterm-surface/` - Terminal surface/screen buffer management
- `wezterm-escape-parser/` - ANSI escape sequence parser (no_std compatible)
- `vtparse/` - VT sequence parser

**GUI and Rendering**:
- `wezterm-gui/` - Main GUI application using wgpu for GPU acceleration
- `window/` - Window management abstraction layer
- `wezterm-font/` - Font handling and rendering

**Multiplexer**:
- `mux/` - Core multiplexer implementation
- `wezterm-mux-server/` - Standalone multiplexer server
- `wezterm-mux-server-impl/` - Server implementation details

**Configuration and Scripting**:
- `config/` - Configuration management with hot-reloading support
- `luahelper/` - Lua scripting integration
- `lua-api-crates/` - Collection of 13+ Lua API modules for extensibility

**Cross-Platform Support**:
- `pty/` - Portable pseudo-terminal implementation
- `wezterm-ssh/` - Native SSH client implementation
- `filedescriptor/` - Cross-platform file descriptor utilities

**Supporting Libraries**:
- `termwiz/` - Reusable terminal utilities library
- `promise/` - Async/promise utilities
- `wezterm-dynamic/` - Dynamic type system for configuration

### Key Design Patterns

1. **Separation of Concerns**: Terminal logic (`term/`) is completely separate from GUI (`wezterm-gui/`) and multiplexer (`mux/`)

2. **Platform Abstraction**: Window management and PTY operations are abstracted to support Linux, macOS, Windows, and BSDs

3. **GPU Acceleration**: Uses wgpu for efficient rendering across different graphics APIs

4. **Hot-Reloading Configuration**: Lua configuration can be changed without restarting the terminal

5. **Vendored Dependencies**: Critical C libraries (cairo, fontconfig, freetype, harfbuzz) are vendored in `deps/` for consistent builds

### Testing Approach

- Unit tests are colocated with source files using `#[cfg(test)]` modules
- Integration tests use helper classes in `term/src/test/`
- The `k9` assertion library is used for expressive test assertions
- Tests should include comments explaining their intent

### CI/CD Pipeline

The project has extensive CI coverage with 40+ GitHub Actions workflows testing on:
- Linux: Debian, Ubuntu, CentOS, Fedora
- macOS
- Windows
- Various architecture combinations

Key workflows check formatting, run tests, and build packages for distribution.