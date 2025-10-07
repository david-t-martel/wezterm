# Build Instructions for WezTerm Filesystem Explorer

## Prerequisites

### Windows
- **Rust**: Install from https://rustup.rs/
- **Git**: For git status features (optional)
- **Visual Studio Build Tools**: Required for linking

```powershell
# Install Rust
winget install Rustlang.Rustup

# Or download from rustup.rs and run:
# rustup-init.exe
```

### Linux/macOS
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install build dependencies (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install build-essential pkg-config libssl-dev

# macOS (requires Homebrew)
brew install rust
```

## Build Methods

### Method 1: Using PowerShell Build Script (Windows)

```powershell
# Navigate to project directory
cd C:\Users\david\wezterm\wezterm-fs-explorer

# Development build (fast compilation, larger binary)
.\build.ps1

# Release build (optimized, smaller binary)
.\build.ps1 -Release

# Ultra-optimized build (smallest binary)
.\build.ps1 -Fast

# Build and install to ~/.local/bin
.\build.ps1 -Release -Install
```

### Method 2: Direct Cargo Commands

#### Windows
```powershell
cd C:\Users\david\wezterm\wezterm-fs-explorer

# Check for compilation errors
cargo check

# Development build
cargo build

# Release build
cargo build --release

# Ultra-optimized release build
cargo build --profile release-fast

# Run directly without building binary
cargo run -- --help
cargo run -- C:\Users\david
```

#### Linux/macOS
```bash
cd ~/wezterm/wezterm-fs-explorer

# Check for compilation errors
cargo check

# Development build
cargo build

# Release build
cargo build --release

# Ultra-optimized release build
cargo build --profile release-fast

# Run directly
cargo run -- --help
cargo run -- ~/projects
```

## Installation

### Windows Installation

#### Option 1: Manual Copy
```powershell
# After building
copy target\release\wezterm-fs-explorer.exe C:\Users\david\.local\bin\

# Verify installation
wezterm-fs-explorer --version
```

#### Option 2: Add to PATH
```powershell
# Add build directory to PATH temporarily
$env:PATH += ";C:\Users\david\wezterm\wezterm-fs-explorer\target\release"

# Add to PATH permanently (PowerShell profile)
$profilePath = $PROFILE
Add-Content $profilePath "`n# WezTerm Explorer"
Add-Content $profilePath "`$env:PATH += ';C:\Users\david\.local\bin'"
```

### Linux Installation

```bash
# Build release version
cargo build --release

# Install to system directory (requires sudo)
sudo cp target/release/wezterm-fs-explorer /usr/local/bin/

# Or install to user directory
mkdir -p ~/.local/bin
cp target/release/wezterm-fs-explorer ~/.local/bin/

# Add to PATH (if not already)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Verify installation
wezterm-fs-explorer --version
```

### macOS Installation

```bash
# Build release version
cargo build --release

# Install to system directory
sudo cp target/release/wezterm-fs-explorer /usr/local/bin/

# Or use Homebrew-style location
mkdir -p /usr/local/opt/wezterm-fs-explorer/bin
cp target/release/wezterm-fs-explorer /usr/local/opt/wezterm-fs-explorer/bin/
ln -s /usr/local/opt/wezterm-fs-explorer/bin/wezterm-fs-explorer /usr/local/bin/

# Verify installation
wezterm-fs-explorer --version
```

## Build Optimization

### Size Optimization

For smallest possible binary:

```bash
# Build with release-fast profile
cargo build --profile release-fast

# Strip symbols (Linux/macOS)
strip target/release-fast/wezterm-fs-explorer

# Windows: Symbols are already stripped in release mode
```

Expected binary sizes:
- **Debug**: ~15-20 MB
- **Release**: ~3-5 MB
- **Release-fast (stripped)**: ~2-3 MB

### Performance Optimization

For fastest execution:

```toml
# Already configured in Cargo.toml:
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
```

## Troubleshooting

### Issue: "cargo: command not found"

**Windows:**
```powershell
# Restart PowerShell to reload PATH
# Or manually add Cargo to PATH
$env:PATH += ";$env:USERPROFILE\.cargo\bin"
```

**Linux/macOS:**
```bash
# Reload shell or source cargo env
source $HOME/.cargo/env
```

### Issue: Linker errors on Windows

**Solution:** Install Visual Studio Build Tools

```powershell
# Download and install from:
# https://visualstudio.microsoft.com/downloads/

# Or install via winget:
winget install Microsoft.VisualStudio.2022.BuildTools
```

### Issue: OpenSSL errors (git2 dependency)

**Windows:** Use the bundled OpenSSL
```powershell
# Set environment variable before building
$env:OPENSSL_STATIC = "1"
cargo build --release
```

**Linux:**
```bash
# Install OpenSSL development libraries
sudo apt-get install libssl-dev pkg-config
```

**macOS:**
```bash
# Install OpenSSL via Homebrew
brew install openssl
export PKG_CONFIG_PATH="/usr/local/opt/openssl/lib/pkgconfig"
```

### Issue: Git status not working

Git status requires libgit2. If you don't need git integration, you can disable it:

```toml
# In Cargo.toml, comment out git2 dependency:
# git2 = { version = "0.18", default-features = false }
```

Then modify `src/git_status.rs` to return `None`:
```rust
pub fn from_repo(path: &Path) -> Option<Self> {
    None // Disable git integration
}
```

### Issue: Slow compilation

Speed up compilation:

```bash
# Use mold linker (Linux)
cargo install mold
export RUSTFLAGS="-C link-arg=-fuse-ld=mold"

# Use lld linker (all platforms)
export RUSTFLAGS="-C link-arg=-fuse-ld=lld"

# Enable incremental compilation
export CARGO_INCREMENTAL=1

# Use faster linker (macOS)
export RUSTFLAGS="-C link-arg=-fuse-ld=lld"
```

## Verification

After successful build:

```bash
# Check binary exists
ls -lh target/release/wezterm-fs-explorer  # Linux/macOS
dir target\release\wezterm-fs-explorer.exe  # Windows

# Test execution
./target/release/wezterm-fs-explorer --version
./target/release/wezterm-fs-explorer --help

# Test in current directory
./target/release/wezterm-fs-explorer .

# Test JSON output
./target/release/wezterm-fs-explorer --json ~
```

## Development Workflow

### Quick development cycle:

```bash
# Watch for changes and rebuild (install cargo-watch first)
cargo install cargo-watch
cargo watch -x 'run -- .'

# Format code
cargo fmt

# Check for common mistakes
cargo clippy

# Run tests (when tests are added)
cargo test

# Check performance
cargo build --release
time ./target/release/wezterm-fs-explorer /large/directory
```

### Adding Features

1. Modify source files in `src/`
2. Test with `cargo run -- [args]`
3. Run `cargo check` to verify compilation
4. Run `cargo clippy` for lints
5. Run `cargo fmt` to format code
6. Build release version: `cargo build --release`

## Cross-Compilation

### Windows to Linux (WSL)
```powershell
# Install cross-compilation tools
rustup target add x86_64-unknown-linux-gnu

# Build for Linux
cargo build --release --target x86_64-unknown-linux-gnu
```

### Linux to Windows
```bash
# Install cross-compilation tools
rustup target add x86_64-pc-windows-gnu
sudo apt-get install mingw-w64

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu
```

## Performance Benchmarks

Expected performance after build:

| Operation | Debug Build | Release Build |
|-----------|-------------|---------------|
| Startup | 80-150ms | 25-50ms |
| Load 1K files | 30-50ms | 8-15ms |
| Load 10K files | 200-400ms | 80-120ms |
| Navigate | 5-10ms | <1ms |
| Search | 15-30ms | 3-8ms |

Memory usage:
- **Idle**: 5-10 MB
- **1K files**: 12-18 MB
- **10K files**: 35-55 MB

## Next Steps

1. Build the project
2. Test basic functionality
3. Integrate with WezTerm (see WEZTERM_INTEGRATION.md)
4. Configure keybindings
5. Customize for your workflow

For integration examples, see [WEZTERM_INTEGRATION.md](WEZTERM_INTEGRATION.md)

For usage documentation, see [README.md](README.md)