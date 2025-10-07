# Justfile: convenient developer commands

set shell := ["powershell", "-NoLogo", "-Command"]

default: build

build:
    $env:RUSTC_WRAPPER="sccache"; cargo build --workspace

release:
    $env:RUSTC_WRAPPER="sccache"; cargo build --workspace --release

fmt:
    cargo fmt --all

clippy:
    # Run clippy without sccache (wrapper causes -vV probe failure in current environment)
    Remove-Item Env:RUSTC_WRAPPER -ErrorAction SilentlyContinue; cargo clippy --workspace --all-targets -- -D warnings -A clippy::type_complexity

clippy-cache:
    # Attempt clippy with sccache (may fail). Use for experimentation.
    $env:RUSTC_WRAPPER="sccache"; cargo clippy --workspace --all-targets -- -D warnings -A clippy::type_complexity

test:
    $env:RUSTC_WRAPPER="sccache"; cargo test --workspace --no-fail-fast

test-nextest:
    # Run tests with nextest if installed; fallback to cargo test
    if (Get-Command cargo-nextest -ErrorAction SilentlyContinue) { $env:RUSTC_WRAPPER="sccache"; cargo nextest run --workspace } else { Write-Host 'cargo-nextest not installed; using cargo test'; $env:RUSTC_WRAPPER="sccache"; cargo test --workspace --no-fail-fast }

bench:
    cargo bench

lint: fmt clippy

check-docs:
    mdbook build docs

arch-docs:
    doxygen Doxyfile.rust

sccache-stats:
    sccache --show-stats

sccache-zero:
    sccache --zero-stats

full-verify: fmt clippy test check-docs sccache-stats

full-local-ci: fmt clippy test-nextest check-docs arch-docs sccache-stats
    # Run an exhaustive local validation approximating old CI heavy checks.
    # Includes architecture docs (doxygen) and nextest (if present).
