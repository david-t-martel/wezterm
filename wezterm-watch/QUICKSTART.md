# wezterm-watch Quick Start Guide

## Installation

```powershell
# Navigate to project
cd C:\Users\david\wezterm\wezterm-watch

# Build release binary
.\build.ps1

# Or manually:
$env:RUSTC_WRAPPER = ""
cargo build --release
```

**Binary location:**
```
C:\Users\david\.cargo\shared-target\release\wezterm-watch.exe
```

## Basic Usage

```bash
# Watch current directory
wezterm-watch .

# Watch with specific format
wezterm-watch . --format json
wezterm-watch . --format pretty
wezterm-watch . --format events
wezterm-watch . --format summary

# Show git status and exit
wezterm-watch . --status

# Custom debounce interval (milliseconds)
wezterm-watch . --interval 200

# Add ignore patterns
wezterm-watch . --ignore "*.log" --ignore "tmp/"

# Disable git integration
wezterm-watch . --no-git
```

## Output Format Examples

### Pretty (Default)
```
Branch: main
Status: 0 ahead, 0 behind
Files: 2 modified, 0 staged, 1 untracked

[M] MODIFIED src/main.rs
[?] CREATED test.txt
[D] DELETED old_file.rs
```

### JSON
```json
{
  "event_type": "modified",
  "path": "src/main.rs",
  "git_status": "M",
  "timestamp": 1704067200
}
```

### Events
```
M ~ src/main.rs
? + test.txt
D - old.rs
```

### Summary
```
[main] ↑0 ↓0 | M:2 S:0 U:1
```

## WezTerm Integration (Minimal)

Add to your `wezterm.lua`:

```lua
local wezterm = require 'wezterm'
local config = {}

-- Key binding to start watcher
config.keys = {
  {
    key = 'W',
    mods = 'CTRL|SHIFT',
    action = wezterm.action.SpawnCommandInNewTab {
      args = { 'wezterm-watch', '.', '--format', 'pretty' },
    },
  },
}

return config
```

## Git Status Indicators

- `M` - Modified
- `A` - Added
- `D` - Deleted
- `R` - Renamed
- `?` - Untracked
- `U` - Conflicted
- `S` - Staged

## Common Patterns

### Development Workflow
```bash
# Watch project with git integration
wezterm-watch . --format pretty --interval 150
```

### Build System Integration
```bash
# Watch for changes, trigger on events
wezterm-watch . --format json | while read event; do
    # Parse JSON and trigger build
done
```

### Large Project
```bash
# Reduce overhead for large repos
wezterm-watch . --interval 300 --format events
```

### CI/CD
```bash
# Minimal overhead, structured output
wezterm-watch . --format json --no-git
```

## Performance Tips

1. **Increase debounce interval** for large projects: `--interval 300`
2. **Disable git** if not needed: `--no-git`
3. **Use events format** for fastest output: `--format events`
4. **Add ignore patterns** to reduce monitoring: `--ignore "build/"`

## Troubleshooting

### High CPU Usage
```bash
# Increase debounce interval
wezterm-watch . --interval 500

# Disable git integration
wezterm-watch . --no-git
```

### Events Not Detected
```bash
# Disable gitignore to see all files
wezterm-watch . --no-gitignore

# Verify you're in the right directory
wezterm-watch . --verbose
```

### Build Issues
```powershell
# Clean and rebuild
cargo clean
.\build.ps1
```

## Quick Reference

| Command | Description |
|---------|-------------|
| `wezterm-watch .` | Watch current directory |
| `--format <FORMAT>` | Set output format (json/pretty/events/summary) |
| `--interval <MS>` | Debounce interval in milliseconds |
| `--git` | Force enable git integration |
| `--no-git` | Disable git integration |
| `--status` | Show git status and exit |
| `--ignore <PATTERN>` | Add ignore pattern |
| `--no-gitignore` | Disable .gitignore parsing |
| `--verbose` | Enable verbose output |
| `--help` | Show all options |

## Next Steps

- Read [README.md](README.md) for detailed documentation
- Check [PERFORMANCE.md](PERFORMANCE.md) for benchmarks
- See [wezterm-integration.lua](wezterm-integration.lua) for advanced WezTerm integration
- Read [IMPLEMENTATION.md](IMPLEMENTATION.md) for architecture details