# Quick Reference Card

## Launch Commands

```bash
wezterm-fs-explorer                    # Current directory
wezterm-fs-explorer /path/to/dir       # Specific directory
wezterm-fs-explorer --json /path       # JSON output mode
wezterm-fs-explorer --help             # Show help
wezterm-fs-explorer --version          # Show version
```

## Navigation Keys

| Key | Action | Description |
|-----|--------|-------------|
| `j` or `↓` | Move down | Navigate to next entry |
| `k` or `↑` | Move up | Navigate to previous entry |
| `h` or `←` | Parent | Go to parent directory |
| `l` or `→` | Enter | Enter selected directory |
| `g` | Top | Jump to first entry |
| `G` | Bottom | Jump to last entry |
| `Ctrl+d` | Page Down | Scroll down one page |
| `Ctrl+u` | Page Up | Scroll up one page |

## File Operations

| Key | Action | Description |
|-----|--------|-------------|
| `Space` | Select | Toggle selection on current entry |
| `Enter` | Open | Output selected paths and exit |
| `d` | Delete | Delete with confirmation |
| `r` | Rename | Rename current entry |
| `c` | Copy | Copy to new location |
| `m` | Move | Move to new location |
| `n` | New | Create new file or directory |

## Display & Search

| Key | Action | Description |
|-----|--------|-------------|
| `/` | Search | Filter entries by name |
| `.` | Toggle Hidden | Show/hide hidden files |
| `Tab` | Toggle Preview | Show/hide preview pane |
| `?` | Help | Show keybindings help |

## Exit

| Key | Action | Description |
|-----|--------|-------------|
| `q` | Quit | Exit without selection |
| `Esc` | Quit | Exit without selection |
| `Ctrl+c` | Force Quit | Immediate exit |

## Visual Indicators

### File Type Icons

- `` - Directory
- `` - Symlink
- `` - Text file
- `` - Rust source
- `` - Python source
- `` - JavaScript
- `` - TypeScript
- `` - Markdown
- `` - JSON
- `` - Archive
- `` - Image
- `` - Video
- `` - Audio

### Git Status

- `M` - Modified
- `A` - Added
- `D` - Deleted
- `R` - Renamed
- `?` - Untracked
- `!` - Ignored

### Selection Marker

- `✓` - Selected entry
- ` ` - Not selected

## Color Coding

- **Blue** - Directories
- **Cyan** - Symlinks
- **Yellow** - Source code
- **Green** - Shell scripts
- **Magenta** - Media files
- **Red** - Archives
- **White** - Regular files

## Input Modes

### Search Mode (`/`)
- Type to filter entries
- `Backspace` to delete character
- `Esc` to cancel
- `Enter` to apply filter

### Rename Mode (`r`)
- Type new name
- `Enter` to confirm
- `Esc` to cancel

### Create Mode (`n`)
- Type name
- End with `/` for directory
- `Enter` to create
- `Esc` to cancel

### Copy/Move Mode (`c`/`m`)
- Type destination path
- `Enter` to confirm
- `Esc` to cancel

### Delete Confirmation
- `y` to confirm deletion
- `n` or `Esc` to cancel

## Preview Pane Info

When preview is enabled (`Tab`):

### For Files
- Name
- Type
- Size
- Modified date
- Permissions
- Extension
- Content preview (for small text files)

### For Directories
- Name
- Type
- Item count
- Permissions
- Modified date

## Shell Integration

### Bash/Zsh
```bash
fe() { $EDITOR "$(wezterm-fs-explorer "${1:-.}")"; }
fcd() { cd "$(wezterm-fs-explorer --json "${1:-.}" | jq -r '.[0]')"; }
```

### PowerShell
```powershell
function fe { param($Path = "."); & $env:EDITOR (wezterm-fs-explorer $Path) }
function fcd { param($Path = "."); Set-Location (wezterm-fs-explorer --json $Path | ConvertFrom-Json)[0] }
```

## WezTerm Keybindings

Add to `~/.wezterm.lua`:

```lua
-- File picker
{ key = 'e', mods = 'CTRL|SHIFT', action = ... }

-- Directory navigator
{ key = 'd', mods = 'CTRL|SHIFT', action = ... }

-- Split pane
{ key = 'n', mods = 'CTRL|SHIFT', action = ... }
```

## Performance Hints

- **Startup**: < 50ms
- **Load 1K files**: ~10ms
- **Load 10K files**: ~100ms
- **Navigation**: < 1ms
- **Search**: ~5ms for 10K files

## Memory Usage

- **Idle**: ~8 MB
- **1K files**: ~15 MB
- **10K files**: ~45 MB
- **100K files**: ~300 MB

## Tips & Tricks

1. **Multi-select**: Use `Space` to select multiple files, then `Enter` to open all
2. **Quick navigate**: Type `/` and search pattern to filter, then navigate
3. **Hidden files**: Press `.` to toggle visibility of dotfiles
4. **Preview**: Use `Tab` to see file details before opening
5. **Parent dir**: Press `h` repeatedly to navigate up the directory tree
6. **Batch operations**: Select multiple files, copy paths with `Enter`, then process in shell

## Common Workflows

### Find and Edit
1. `wezterm-fs-explorer /projects`
2. Navigate with `j/k` or search with `/`
3. Press `Enter` on file
4. Opens in `$EDITOR`

### Multi-file Edit
1. Launch explorer
2. Navigate and press `Space` on each file
3. Press `Enter` to output all paths
4. Opens all in editor tabs

### Directory Navigation
1. `wezterm-fs-explorer --json ~`
2. Navigate to directory
3. Press `Enter`
4. Use output in `cd` command

### File Cleanup
1. Launch explorer
2. Navigate to files
3. Press `d` to delete
4. Press `y` to confirm

### Batch Rename
1. Select files with `Space`
2. Note file names
3. Exit and use shell rename tools
4. (Built-in batch rename coming in v0.2)

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Icons not showing | Install Nerd Font in WezTerm |
| Git status slow | Work on smaller repos or disable git |
| Can't see hidden files | Press `.` to toggle |
| Preview not working | Press `Tab` to enable |
| Exit not working | Use `Ctrl+c` to force quit |

## Environment Variables

```bash
# Set default editor
export EDITOR=nvim

# Disable git integration (future)
export WEZTERM_EXPLORER_GIT=0

# Custom theme (future)
export WEZTERM_EXPLORER_THEME=dark
```

## Exit Codes

- `0` - Success, files selected
- `1` - Error occurred
- `130` - User cancelled (Ctrl+C)

## JSON Output Format

```json
[
  "/path/to/file1.txt",
  "/path/to/file2.txt"
]
```

Use with:
```bash
jq -r '.[]' < selected.json
```

---

**Version**: 0.1.0
**License**: MIT
**Documentation**: [README.md](README.md)
**Integration**: [WEZTERM_INTEGRATION.md](WEZTERM_INTEGRATION.md)