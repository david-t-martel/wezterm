# Getting Started with WezTerm Filesystem Explorer

This guide will get you up and running with `wezterm-fs-explorer` in under 5 minutes.

## Quick Start (TL;DR)

```powershell
# Windows
cd C:\Users\david\wezterm\wezterm-fs-explorer
.\build.ps1 -Release -Install
wezterm-fs-explorer
```

```bash
# Linux/macOS
cd ~/wezterm/wezterm-fs-explorer
cargo build --release
sudo cp target/release/wezterm-fs-explorer /usr/local/bin/
wezterm-fs-explorer
```

## Step-by-Step Installation

### Prerequisites

#### 1. Install Rust

**Windows:**
```powershell
# Option 1: Using winget
winget install Rustlang.Rustup

# Option 2: Direct download
# Visit https://rustup.rs/ and download rustup-init.exe
# Run the installer and follow prompts
```

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify installation:
```bash
rustc --version  # Should show 1.70.0 or higher
cargo --version
```

#### 2. Install Git (Optional, for git status features)

**Windows:**
```powershell
winget install Git.Git
```

**Linux:**
```bash
sudo apt-get install git  # Debian/Ubuntu
sudo yum install git      # CentOS/RHEL
```

**macOS:**
```bash
brew install git
```

#### 3. Install Nerd Font (Recommended)

Download and install a Nerd Font for icon display:

**Windows:**
1. Visit https://www.nerdfonts.com/font-downloads
2. Download "JetBrains Mono Nerd Font"
3. Extract and install fonts
4. Configure WezTerm to use the font (see below)

**Linux:**
```bash
mkdir -p ~/.local/share/fonts
cd ~/.local/share/fonts
curl -fLo "JetBrains Mono Nerd Font.ttf" \
  https://github.com/ryanoasis/nerd-fonts/releases/download/v3.0.0/JetBrainsMono.zip
unzip JetBrainsMono.zip -d JetBrainsMono
fc-cache -fv
```

**macOS:**
```bash
brew tap homebrew/cask-fonts
brew install --cask font-jetbrains-mono-nerd-font
```

### Building the Project

#### Windows Build

```powershell
# Navigate to project directory
cd C:\Users\david\wezterm\wezterm-fs-explorer

# Check for compilation errors first
cargo check

# Build release version (optimized)
cargo build --release

# Or use the PowerShell build script
.\build.ps1 -Release

# Verify binary was created
Test-Path target\release\wezterm-fs-explorer.exe

# Check binary size (should be 3-5 MB)
(Get-Item target\release\wezterm-fs-explorer.exe).Length / 1MB
```

#### Linux/macOS Build

```bash
# Navigate to project directory
cd ~/wezterm/wezterm-fs-explorer

# Check for compilation errors
cargo check

# Build release version
cargo build --release

# Verify binary
ls -lh target/release/wezterm-fs-explorer

# Expected output: ~3-5 MB binary
```

**Note**: First build may take 5-10 minutes to compile dependencies.

### Installation

#### Windows Installation

**Option 1: User bin directory (Recommended)**
```powershell
# Create local bin directory if it doesn't exist
$binDir = "$env:USERPROFILE\.local\bin"
New-Item -ItemType Directory -Path $binDir -Force

# Copy binary
Copy-Item target\release\wezterm-fs-explorer.exe $binDir\

# Add to PATH (PowerShell profile)
$profilePath = $PROFILE
if (-not (Test-Path $profilePath)) {
    New-Item -Path $profilePath -ItemType File -Force
}
Add-Content $profilePath "`n# WezTerm Explorer"
Add-Content $profilePath "`$env:PATH += ';$binDir'"

# Reload profile
. $PROFILE
```

**Option 2: Using build script**
```powershell
.\build.ps1 -Release -Install
```

**Option 3: System-wide (Requires Admin)**
```powershell
# Run as Administrator
Copy-Item target\release\wezterm-fs-explorer.exe C:\Windows\System32\
```

#### Linux Installation

**Option 1: System-wide**
```bash
sudo cp target/release/wezterm-fs-explorer /usr/local/bin/
sudo chmod +x /usr/local/bin/wezterm-fs-explorer
```

**Option 2: User-local**
```bash
mkdir -p ~/.local/bin
cp target/release/wezterm-fs-explorer ~/.local/bin/

# Add to PATH (if not already)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### macOS Installation

```bash
# System-wide
sudo cp target/release/wezterm-fs-explorer /usr/local/bin/

# Or user-local
mkdir -p ~/.local/bin
cp target/release/wezterm-fs-explorer ~/.local/bin/
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Verification

Test the installation:

```bash
# Check version
wezterm-fs-explorer --version
# Expected: wezterm-fs-explorer 0.1.0

# Show help
wezterm-fs-explorer --help

# Test in current directory
wezterm-fs-explorer .

# Test with keyboard: j/k to navigate, q to quit
```

If you see a file listing with icons and colors, you're all set! 🎉

## First Use

### Basic Navigation

1. Launch the explorer:
   ```bash
   wezterm-fs-explorer
   ```

2. Navigate with keyboard:
   - `j` or `↓` - Move down
   - `k` or `↑` - Move up
   - `l` or `→` - Enter directory
   - `h` or `←` - Go to parent
   - `g` - Jump to top
   - `G` - Jump to bottom

3. Select a file and press `Enter` to output its path

4. Press `q` to quit

### Common First Tasks

#### Task 1: Browse and Open a File

```bash
# Launch in projects directory
wezterm-fs-explorer ~/projects

# Navigate with j/k
# Press Enter on a file
# The path will be output, which you can use in your editor
```

To make this easier, create a shell function (see below).

#### Task 2: Search for Files

```bash
# Launch explorer
wezterm-fs-explorer

# Press / to search
# Type part of filename
# Navigate filtered results
# Press Enter to select
```

#### Task 3: Toggle Hidden Files

```bash
# Launch explorer
wezterm-fs-explorer

# Press . (period) to toggle hidden files
# Now you can see .gitignore, .env, etc.
```

#### Task 4: View File Details

```bash
# Launch explorer
wezterm-fs-explorer

# Press Tab to show preview pane
# Navigate to files to see details
# Press Tab again to hide
```

## Shell Integration

For the best experience, add these functions to your shell:

### Bash/Zsh

Add to `~/.bashrc` or `~/.zshrc`:

```bash
# File explorer function
fe() {
    local selected=$(wezterm-fs-explorer "${1:-.}")
    if [ -n "$selected" ]; then
        ${EDITOR:-nvim} "$selected"
    fi
}

# Directory navigation
fcd() {
    local dir=$(wezterm-fs-explorer --json "${1:-.}" | jq -r '.[0]')
    if [ -n "$dir" ] && [ -d "$dir" ]; then
        cd "$dir"
    elif [ -n "$dir" ]; then
        cd "$(dirname "$dir")"
    fi
}

# Keybindings (optional)
bind '"\C-o":"fe\n"'  # Ctrl+O for file explorer
```

Then reload:
```bash
source ~/.bashrc  # or source ~/.zshrc
```

### PowerShell

Add to PowerShell profile (`notepad $PROFILE`):

```powershell
# File explorer function
function fe {
    param([string]$Path = ".")
    $selected = wezterm-fs-explorer $Path
    if ($selected) {
        $editor = if ($env:EDITOR) { $env:EDITOR } else { "nvim" }
        & $editor $selected
    }
}

# Directory navigation
function fcd {
    param([string]$Path = ".")
    $json = wezterm-fs-explorer --json $Path
    if ($json) {
        $paths = $json | ConvertFrom-Json
        if ($paths -and $paths.Count -gt 0) {
            $selected = $paths[0]
            if (Test-Path $selected -PathType Container) {
                Set-Location $selected
            } else {
                Set-Location (Split-Path $selected)
            }
        }
    }
}

# Keybindings (requires PSReadLine)
Set-PSReadLineKeyHandler -Key Ctrl+o -ScriptBlock {
    [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
    [Microsoft.PowerShell.PSConsoleReadLine]::Insert("fe")
    [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
}
```

Then reload:
```powershell
. $PROFILE
```

### Usage After Shell Integration

```bash
# Open file explorer and edit selected file
fe
fe ~/projects
fe /var/log

# Navigate to directory
fcd
fcd ~/projects

# With keybinding (if configured)
# Press Ctrl+O to launch file explorer
```

## WezTerm Integration

### Basic Configuration

Add to `~/.wezterm.lua` or `C:\Users\david\.wezterm.lua`:

```lua
local wezterm = require 'wezterm'
local config = {}

-- Font with Nerd Font support
config.font = wezterm.font 'JetBrains Mono Nerd Font'
config.font_size = 12.0

-- File explorer keybinding
config.keys = {
    {
        key = 'e',
        mods = 'CTRL|SHIFT',
        action = wezterm.action.SpawnCommandInNewTab {
            args = { 'wezterm-fs-explorer' },
        },
    },
}

return config
```

Now press `Ctrl+Shift+E` in WezTerm to launch the explorer!

For more advanced integration, see `examples/wezterm-config-example.lua`.

## Troubleshooting

### Issue: "wezterm-fs-explorer: command not found"

**Solution**: Ensure the binary is in your PATH.

```powershell
# Windows: Check PATH
$env:PATH -split ';' | Select-String "\.local\\bin"

# Add if missing
$env:PATH += ";$env:USERPROFILE\.local\bin"
```

```bash
# Linux/macOS: Check PATH
echo $PATH | grep -o "[^:]*local[^:]*bin"

# Add if missing
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Issue: Icons not displaying

**Solution**: Install a Nerd Font and configure WezTerm.

```lua
-- In wezterm.lua
config.font = wezterm.font 'JetBrains Mono Nerd Font'
```

Restart WezTerm after changing configuration.

### Issue: Build fails with "linker error"

**Windows Solution**: Install Visual Studio Build Tools

```powershell
winget install Microsoft.VisualStudio.2022.BuildTools
```

**Linux Solution**: Install build essentials

```bash
sudo apt-get install build-essential pkg-config libssl-dev
```

### Issue: Git status not working

**Solution**: Git integration requires libgit2. If git2 fails to compile:

```bash
# Install git
sudo apt-get install git libgit2-dev  # Linux
brew install git libgit2               # macOS
```

Or disable git integration by commenting out `git2` in Cargo.toml.

### Issue: Slow performance in large directories

This is expected behavior. Performance tips:

- Use search (`/`) to filter results
- Avoid directories with >100K files
- Git status can be slow in large repos

## Next Steps

Now that you have the explorer installed:

1. **Learn the keybindings**: See [QUICKREF.md](QUICKREF.md) for a complete reference

2. **Set up advanced integration**: Check [WEZTERM_INTEGRATION.md](WEZTERM_INTEGRATION.md) for advanced examples

3. **Customize your workflow**: Add shell functions and keybindings that suit your needs

4. **Explore features**:
   - Multi-file selection (Space)
   - Search/filter (/)
   - Hidden files toggle (.)
   - Preview pane (Tab)
   - File operations (d/r/c/m/n)

## Getting Help

- **Quick reference**: [QUICKREF.md](QUICKREF.md)
- **Full documentation**: [README.md](README.md)
- **Integration guide**: [WEZTERM_INTEGRATION.md](WEZTERM_INTEGRATION.md)
- **Build issues**: [BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md)
- **Testing**: [TESTING.md](TESTING.md)

## Tips for Beginners

1. **Start simple**: Just use `j/k` to navigate and `Enter` to select
2. **Learn one feature at a time**: Add more keybindings as you get comfortable
3. **Use shell functions**: They make the tool much more convenient
4. **Configure WezTerm**: Proper font configuration is essential for icons
5. **Practice**: The Vim-like keybindings become second nature quickly

## Example Workflow

Here's a typical workflow after installation:

```bash
# 1. Navigate to project
cd ~/projects/myapp

# 2. Launch explorer (or press Ctrl+O if configured)
fe

# 3. Navigate with j/k, search with /
# Press / and type "config"
# Navigate to config file

# 4. Press Enter to open in editor
# File opens in $EDITOR (nvim, vim, code, etc.)

# 5. Edit file, save, exit editor

# 6. Back in terminal, continue working
```

Congratulations! You're now ready to use wezterm-fs-explorer! 🚀