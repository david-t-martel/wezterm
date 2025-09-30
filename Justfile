# Justfile: convenient developer commands

set shell := ["powershell", "-NoLogo", "-Command"]

default: build

build:
    cargo build --workspace

release:
    cargo build --workspace --release

fmt:
    cargo fmt --all

clippy:
    $env:RUSTC_WRAPPER="sccache"; cargo clippy --workspace --all-targets -- -D warnings -Aclippy::type_complexity

test:
    $env:RUSTC_WRAPPER="sccache"; cargo test --workspace --no-fail-fast

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
