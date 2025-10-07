# WezTerm Project Context
**Timestamp**: 2025-01-30
**Repository**: github.com/david-t-martel/wezterm

## 1. Project Overview

### Project Goals and Objectives
- **Core Purpose**: WezTerm is a GPU-accelerated cross-platform terminal emulator and multiplexer written in Rust
- **Key Features**: GPU acceleration, cross-platform support, multiplexing capabilities, Lua configuration with hot-reloading
- **Target Platforms**: Linux, macOS, Windows, FreeBSD, OpenBSD

### Key Architectural Decisions
- **Separation of Concerns**: Terminal logic completely separated from GUI implementation
- **GPU Acceleration**: Leverages wgpu for high-performance rendering
- **Configuration System**: Lua scripting with hot-reload capability for dynamic configuration
- **Pure Rust Implementation**: Entire codebase in Rust with carefully vendored C dependencies

### Technology Stack
- **Language**: Pure Rust
- **Build System**: Cargo workspace with 19+ crates
- **Rendering**: wgpu (GPU acceleration framework)
- **Configuration**: Lua with 13+ API modules
- **Testing**: cargo nextest (preferred over cargo test)
- **Documentation**: mkdocs-based documentation system

### Team Conventions
- **Testing Framework**: cargo nextest for all test execution
- **Formatting**: nightly Rust toolchain for formatting
- **Assertions**: k9 library for test assertions
- **CI/CD**: 40+ GitHub Actions workflows for comprehensive platform coverage

## 2. Current State

### Recently Implemented
- **CLAUDE.md Creation**: Comprehensive development guide for Claude Code instances
- **Repository Fork**: Forked to github.com/david-t-martel/wezterm
- **Git Configuration**: Updated with proper user credentials (david.martel@auricleinc.com)

### Work Completed
- Repository successfully forked and configured
- CLAUDE.md documentation file created with full development guidelines
- Git configuration updated with correct author information
- Initial codebase analysis and structure documented

### Known Issues
- **Compiler Warnings**: Recent `mismatched_lifetime_syntaxes` warnings (already fixed in recent commits)
- **Platform Deprecation**: Debian 10.3 (buster) removed from CI due to availability issues

### Performance Configuration
- **Release Builds**: Configured with opt-level 3 for maximum performance
- **GPU Optimization**: Full hardware acceleration via wgpu
- **Memory Management**: Careful attention to Rust ownership patterns

## 3. Design Decisions

### Architecture Structure
```
wezterm/
├── term/                 # Core terminal engine (vt100 emulation)
├── wezterm-gui/         # GUI application code
├── mux/                 # Multiplexing logic
├── window/              # Cross-platform windowing abstraction
├── pty/                 # Cross-platform PTY abstraction
├── wezterm-ssh/         # Native SSH client implementation
├── lua-api-crates/      # 13+ Lua API modules
├── deps/                # Vendored C dependencies
└── ci/                  # CI/CD scripts and workflows
```

### Platform Abstraction Strategy
- **window/**: Provides unified API across Windows, macOS, Linux, Wayland
- **pty/**: Abstracts PTY operations for all platforms
- **Conditional Compilation**: Platform-specific code using cfg attributes

### Vendored Dependencies Rationale
Critical C libraries vendored for consistency:
- cairo (graphics rendering)
- fontconfig (font management)
- freetype (font rendering)
- harfbuzz (text shaping)

### Configuration System Design
- **Hot-Reload**: Lua configuration can be modified without restart
- **13+ API Modules**: Comprehensive Lua API for customization
- **Event-Driven**: Configuration responds to runtime events

## 4. Code Patterns

### Testing Patterns
```rust
// Unit tests colocated with source
#[cfg(test)]
mod tests {
    use super::*;
    use k9::assert_equal;

    #[test]
    fn test_functionality() {
        // Test implementation
    }
}
```

### Integration Testing
- Location: `term/src/test/` for terminal integration tests
- Framework: cargo nextest with k9 assertions
- Coverage: Comprehensive test suite for VT sequences

### Build System Commands
```bash
# Primary build commands
make build          # Build all main binaries
make test          # Run tests via cargo nextest
make fmt           # Format with nightly Rust
cargo check        # Quick type checking

# Documentation
make docs          # Build documentation
make servedocs     # Serve docs locally

# Platform-specific
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target x86_64-unknown-linux-gnu
```

### CI/CD Patterns
- **40+ Workflows**: Comprehensive platform coverage
- **Matrix Testing**: Multiple OS versions and architectures
- **Artifact Generation**: Automated release builds for all platforms
- **Documentation**: Automated docs deployment

### Documentation Standards
- **Code Comments**: Inline documentation for complex logic
- **API Docs**: Comprehensive rustdoc comments
- **User Docs**: mkdocs-based user documentation
- **Configuration Docs**: Lua API documentation

## 5. Agent Coordination History

### Agent Activities Performed
1. **code-reviewer Agent Analysis**
   - Analyzed full codebase structure
   - Identified Rust workspace architecture
   - Documented 19+ crates and their purposes
   - Mapped dependency relationships

2. **Documentation Creation**
   - Successfully created CLAUDE.md file
   - Documented development guidelines
   - Captured build commands and testing procedures
   - Established code quality standards

3. **Git Configuration**
   - Updated git config with proper author details
   - Set user.name to "david-t-martel"
   - Set user.email to "david.martel@auricleinc.com"
   - Changes pushed to fork

### Key Decisions Made
- Use CLAUDE.md format for AI assistant guidance
- Maintain existing code patterns and conventions
- Respect workspace structure and separation of concerns
- Follow established testing practices with cargo nextest

## 6. Future Roadmap

### Active Maintenance Areas
- **Dependency Updates**: Regular updates to Rust dependencies
- **Platform Support**: Continuous integration for new OS versions
- **Performance**: Ongoing GPU rendering optimizations
- **Bug Fixes**: Active issue resolution

### Recent Focus Areas
- **Compiler Warning Fixes**: Addressing Rust compiler warnings
- **Documentation Improvements**: Typo fixes and clarity enhancements
- **CI/CD Maintenance**: Removing deprecated platform builds
- **Test Infrastructure**: Maintaining test coverage

### Potential Enhancement Areas
- **Feature Development**: New terminal features and capabilities
- **Performance**: Further GPU optimization opportunities
- **Platform Support**: Additional platform compatibility
- **Configuration**: Extended Lua API capabilities

## Key Files Reference

### Created/Modified Files
```
CLAUDE.md                    # Comprehensive Claude Code development guide
.git/config                  # Updated with proper author information
```

### Critical Project Files
```
Cargo.toml                   # Workspace root configuration
term/src/lib.rs             # Core terminal engine entry
wezterm-gui/src/main.rs     # GUI application entry point
config/src/lib.rs           # Configuration system core
lua-api-crates/*/src/lib.rs # Lua API implementations
```

### Build Configuration
```
Makefile                     # Build automation wrapper
.github/workflows/*.yml     # 40+ CI/CD workflows
ci/build-docs.sh            # Documentation build script
ci/deploy.sh                # Deployment automation
```

## Quick Command Reference

### Development Commands
```bash
# Build
cargo build --release
make build

# Test
cargo nextest run
make test

# Format
cargo +nightly fmt
make fmt

# Check
cargo check --workspace
cargo clippy --workspace

# Documentation
./ci/build-docs.sh
make servedocs
```

### Git Operations
```bash
# Push changes
git add .
git commit -m "message"
git push origin main

# Sync with upstream
git fetch upstream
git merge upstream/main
```

## Architecture Highlights

### Core Strengths
1. **19+ Workspace Crates**: Well-organized modular architecture
2. **Terminal/GUI Separation**: Clean separation of concerns
3. **GPU Acceleration**: High-performance rendering with wgpu
4. **Native SSH**: Built-in SSH client implementation
5. **Lua Scripting**: Powerful configuration with hot-reload
6. **Cross-Platform**: True multi-platform support

### Technical Excellence
- **Memory Safety**: Rust's ownership system ensures safety
- **Performance**: GPU acceleration and optimization
- **Modularity**: Clear crate boundaries and interfaces
- **Testing**: Comprehensive test coverage with nextest
- **Documentation**: Well-documented codebase and APIs
- **CI/CD**: Robust automated testing and deployment

## Restoration Instructions

To restore this context in a new session:
1. Read this file to understand project state
2. Check git status for any uncommitted changes
3. Review recent commits for latest updates
4. Verify build environment with `cargo check`
5. Run tests to ensure working state: `make test`

## Session Metadata
- **Context Saved**: 2025-01-30
- **Repository**: github.com/david-t-martel/wezterm
- **Branch**: main (forked from wez/wezterm)
- **Last Known Good State**: CLAUDE.md created, git configured, fork established