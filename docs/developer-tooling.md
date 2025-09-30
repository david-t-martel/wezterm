# Developer Tooling & Automation

This document describes the local development automation for the WezTerm repository: pre-commit hooks, formatting, linting, testing, documentation generation and Rust-specific configuration guidance.

## Overview

We aim to keep `main` in a consistently buildable, lint-clean state. The tooling helps contributors catch common issues early and produce uniform code & docs.

| Area | Tool | Command(s) |
|------|------|------------|
| Formatting | `cargo fmt` | `cargo fmt --all` |
| Linting | `clippy` | `cargo clippy --workspace --all-targets -- -D warnings -Aclippy::type_complexity` |
| Dependency / License Audit | `cargo-deny` | `cargo deny check advisories licenses bans sources` |
| Unit/Integration Tests | `cargo test` | `cargo test --workspace` |
| Docs (Rust API) | `rustdoc` | `cargo doc --workspace --all-features --no-deps` |
| Docs (User Guide) | `mdBook` | `mdbook build docs` |
| Supplemental Architecture Docs | `doxygen` | `doxygen Doxyfile.rust` |

### Build Caching (sccache)

We use `sccache` as a compiler cache on developer machines and in tooling hooks to accelerate rebuilds across feature branches.

Key points:

- Incremental compilation is disabled (`incremental = false` in `[profile.dev]`) so object outputs are deterministic and cacheable.
- `.cargo/config.toml` sets `rustc-wrapper = "sccache"` and defines a cache directory & size (`SCCACHE_CACHE_SIZE=15G`).
- Pre-commit and pre-push hooks export `RUSTC_WRAPPER=sccache` to ensure cached compilation during lint/tests.
- If a one-off build misbehaves under sccache, temporarily unset with `set RUSTC_WRAPPER=` (Windows PowerShell: `$env:RUSTC_WRAPPER=''`).

Inspect cache stats:

```bash
sccache --show-stats
```

Common gotcha: some installation flows (like single `cargo install` under a wrapper) may fail if the wrapper disallows incremental internals; disabling the wrapper for that command resolves it.

## Git Pre-Commit Hooks

We use the [`pre-commit`](https://pre-commit.com/) framework for local hook orchestration. It runs fast, incremental checks before you create a commit.

### Installation

```bash
pip install pre-commit  # or: pipx install pre-commit
pre-commit install --hook-type pre-commit --hook-type pre-push
```

This installs both the default `pre-commit` hook and (optionally) a `pre-push` hook if extended later.

### What Runs

On commit, the following run (skipping heavier tasks if dependencies absent):

1. YAML, whitespace & merge conflict detection (`pre-commit-hooks`)
2. Python formatting in `ci/`, `scripts/`, `docs/` (ruff-format) if Python present
3. `cargo fmt` (always) – enforces style
4. `cargo clippy` (fast) – denies warnings, relaxes `type_complexity`
5. Targeted tests when Rust files staged (quick safeguard)
6. Optional: `cargo deny` (if installed) for advisories & licenses
7. Optional: `mdbook build docs` to catch link/render errors early
8. Optional: `doxygen` (if installed) for supplemental architecture XML

You can run all hooks manually:

```bash
pre-commit run --all-files
```

## Doxygen for Rust

Rust already has excellent `rustdoc`, but we provide an experimental `Doxyfile.rust` for teams needing cross-language architecture diagrams or for tools consuming Doxygen XML/tag files.

### Generating

```bash
doxygen Doxyfile.rust
```

Outputs to `docs/doxygen`. If you also generate `rustdoc` JSON (nightly), add paths to that JSON in `Doxyfile.rust` to improve linkability.

### Filter Script

The lightweight filter at `ci/doxygen-rust-filter.sh` strips attributes and translates some Rust constructs for Doxygen's parser. It is intentionally shallow; for richer output consider tools like `cargo doc` + custom processors.

## Configuration Files & Customization (.wezterm.lua)

WezTerm configuration is Lua-based and typically resides at:

- Unix-like: `~/.wezterm.lua`
- Windows: `%USERPROFILE%/.wezterm.lua`

### Minimal Example

```lua
local wezterm = require 'wezterm'
return {
  font = wezterm.font_with_fallback({ 'JetBrainsMono Nerd Font', 'FiraCode Nerd Font' }),
  font_size = 12.0,
  color_scheme = 'Builtin Tango Dark',
  enable_tab_bar = true,
}
```

### Advanced Patterns

Split growing configs into modules:

```lua
-- ~/.wezterm.lua
local wezterm = require 'wezterm'
local act = wezterm.action
local keys = require 'keys'       -- keys.lua in same directory
local appearance = require 'appearance'

return {
  keys = keys.setup(act),
  colors = appearance.colors(),
  window_decorations = 'RESIZE',
}
```

Then create `~/.config/wezterm/keys.lua` (Linux/macOS) or `%USERPROFILE%/wezterm/keys.lua` (Windows) for modular reuse.

### Dynamic Reloading

WezTerm automatically reloads when `~/.wezterm.lua` changes. Use `wezterm ls-fonts --list-system` to explore fonts referenced in your configuration.

### Platform-Specific Sections

Use conditionals:

```lua
local wezterm = require 'wezterm'
local config = {}
if wezterm.target_triple:find('windows') then
  config.default_prog = { 'pwsh.exe', '-NoLogo' }
else
  config.default_prog = { '/usr/bin/zsh' }
end
return config
```

### Multiplexing Integration

Configure startup workspace:

```lua
return {
  unix_domains = { { name = 'local' } },
  default_gui_startup_args = { 'connect', 'local' },
}
```

See `docs/multiplexing.md` for more.

### Key Bindings

Map actions:

```lua
local wezterm = require 'wezterm'
return {
  keys = {
    { key = 'Enter', mods = 'ALT', action = wezterm.action.ToggleFullScreen },
    { key = 'D', mods = 'CTRL|SHIFT', action = wezterm.action.SplitHorizontal({ domain = 'CurrentPaneDomain' }) },
  },
}
```

Further examples: `docs/config/keys.md`.

### Color Schemes

You can inline a scheme or point to a `color_schemes` table; dynamic overrides are covered in `docs/config/appearance.md`.

### Performance Tips

- Disable unused ligatures: choose a font variant without them if you don't need them.
- Reduce animation: set `animation_fps = 1` if you prefer static UI.
- GPU adapter: set `webgpu_power_preference = 'HighPerformance'` for discrete GPUs.

## Updating Docs

When adding new config keys, update relevant pages under `docs/config/` and reference them here where appropriate. Run:

```bash
mdbook build docs
```

## CI Considerations

CI can run heavier variants (full test suite, full clippy, deny advisories) while local hooks stay fast. Consider adding a `pre-push` hook with: `cargo test --workspace --all-features` for more exhaustive checks.

## Troubleshooting

If hooks feel slow:

- Use `SKIP=ruff-format pre-commit run` to skip.
- Run `pre-commit gc` occasionally to prune old environments.

If Doxygen output seems sparse:

- Ensure filter script is executable (`chmod +x ci/doxygen-rust-filter.sh` on Unix) and present in PATH.
- Try enabling `EXTRACT_PRIVATE = YES` temporarily (not recommended for publication).

---
This document will evolve; contributions welcome.
