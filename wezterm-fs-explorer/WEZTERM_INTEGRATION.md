# WezTerm Integration Guide

Complete guide for integrating `wezterm-fs-explorer` into your WezTerm workflow.

## Quick Start

### 1. Build and Install

```powershell
# Windows
cd C:\Users\david\wezterm\wezterm-fs-explorer
cargo build --release
copy target\release\wezterm-fs-explorer.exe C:\Users\david\.local\bin\
```

```bash
# Linux/macOS
cd ~/wezterm/wezterm-fs-explorer
cargo build --release
sudo cp target/release/wezterm-fs-explorer /usr/local/bin/
```

### 2. Basic WezTerm Configuration

Add to `~/.wezterm.lua` or `C:\Users\david\.wezterm.lua`:

```lua
local wezterm = require 'wezterm'
local act = wezterm.action

local config = {}

-- Filesystem Explorer Integration
config.keys = {
  -- Open file explorer in current directory
  {
    key = 'e',
    mods = 'CTRL|SHIFT',
    action = act.SpawnCommandInNewTab {
      args = { 'wezterm-fs-explorer' },
    },
  },
}

return config
```

## Integration Patterns

### Pattern 1: Inline File Picker

Open explorer, select file, and return to edit:

```lua
local function file_picker(window, pane)
  local cwd = pane:get_current_working_dir()
  local path = cwd and cwd.file_path or os.getenv("HOME")

  -- Run explorer and capture output
  local success, stdout, stderr = wezterm.run_child_process({
    'wezterm-fs-explorer',
    path
  })

  if success and stdout ~= "" then
    -- Parse selected files (one per line)
    for file_path in stdout:gmatch("[^\r\n]+") do
      if file_path ~= "" then
        -- Open in Neovim
        pane:send_text("nvim '" .. file_path .. "'\r")
        break -- Open first file only
      end
    end
  else
    wezterm.log_error("File picker failed: " .. tostring(stderr))
  end
end

return {
  keys = {
    {
      key = 'f',
      mods = 'CTRL|SHIFT',
      action = wezterm.action_callback(file_picker),
    },
  }
}
```

### Pattern 2: Split Pane Navigator

Open explorer in split pane for side-by-side navigation:

```lua
local function split_file_navigator(window, pane)
  local tab = window:mux_window():active_tab()
  local cwd = pane:get_current_working_dir()

  -- Create vertical split
  local new_pane = tab:split_pane({
    direction = 'Right',
    size = 0.4,
    cwd = cwd,
  })

  -- Launch explorer in split
  new_pane:send_text('wezterm-fs-explorer\n')
end

return {
  keys = {
    {
      key = 'n',
      mods = 'CTRL|SHIFT',
      action = wezterm.action_callback(split_file_navigator),
    },
  }
}
```

### Pattern 3: Directory Quick Jump

Navigate directories quickly with JSON mode:

```lua
local function quick_cd(window, pane)
  local home = os.getenv("HOME") or os.getenv("USERPROFILE")

  local success, stdout = wezterm.run_child_process({
    'wezterm-fs-explorer',
    '--json',
    home
  })

  if success and stdout ~= "" then
    local ok, paths = pcall(wezterm.json_parse, stdout)
    if ok and paths and #paths > 0 then
      local selected = paths[1]
      -- Check if directory
      if selected:match("/$") or wezterm.glob(selected .. "/*")[1] then
        pane:send_text("cd '" .. selected .. "'\r")
        pane:send_text("clear\r")
      else
        -- If file, cd to parent directory
        local parent = selected:match("(.*/)")
        if parent then
          pane:send_text("cd '" .. parent .. "'\r")
        end
      end
    end
  end
end

return {
  keys = {
    {
      key = 'd',
      mods = 'CTRL|SHIFT',
      action = wezterm.action_callback(quick_cd),
    },
  }
}
```

### Pattern 4: Multi-File Operations

Select multiple files and perform batch operations:

```lua
local function batch_file_processor(window, pane)
  local cwd = pane:get_current_working_dir()
  local path = cwd and cwd.file_path or "."

  local success, stdout = wezterm.run_child_process({
    'wezterm-fs-explorer',
    '--json',
    path
  })

  if success and stdout ~= "" then
    local ok, paths = pcall(wezterm.json_parse, stdout)
    if ok and paths and #paths > 0 then
      -- Example: Create tar archive of selected files
      local files = table.concat(paths, " ")
      pane:send_text("tar -czf selected.tar.gz " .. files .. "\r")

      -- Example: Open all in editor
      -- pane:send_text("nvim -p " .. files .. "\r")

      -- Example: Copy to destination
      -- pane:send_text("cp " .. files .. " /destination/\r")
    end
  end
end

return {
  keys = {
    {
      key = 'b',
      mods = 'CTRL|SHIFT',
      action = wezterm.action_callback(batch_file_processor),
    },
  }
}
```

## Advanced Configurations

### Configuration 1: Context-Aware Explorer

Different explorer behavior based on context:

```lua
local function context_aware_explorer(window, pane)
  local cwd = pane:get_current_working_dir()
  local path = cwd and cwd.file_path or os.getenv("HOME")

  -- Detect project type
  local is_git_repo = wezterm.glob(path .. "/.git")[1] ~= nil
  local is_rust_project = wezterm.glob(path .. "/Cargo.toml")[1] ~= nil
  local is_node_project = wezterm.glob(path .. "/package.json")[1] ~= nil

  local success, stdout = wezterm.run_child_process({
    'wezterm-fs-explorer',
    path
  })

  if success and stdout ~= "" then
    for file_path in stdout:gmatch("[^\r\n]+") do
      local ext = file_path:match("^.+%.(.+)$")

      -- Context-aware actions
      if ext == "rs" and is_rust_project then
        pane:send_text("cargo run --bin " .. file_path:match("([^/]+)%.rs$") .. "\r")
      elseif ext == "js" or ext == "ts" and is_node_project then
        pane:send_text("node " .. file_path .. "\r")
      elseif ext == "py" then
        pane:send_text("python " .. file_path .. "\r")
      else
        pane:send_text("$EDITOR '" .. file_path .. "'\r")
      end
      break
    end
  end
end
```

### Configuration 2: Workspace Manager

Manage multiple project workspaces:

```lua
local workspaces = {
  { name = "Projects", path = "/home/user/projects" },
  { name = "Documents", path = "/home/user/documents" },
  { name = "Config", path = "/home/user/.config" },
}

local function workspace_picker(window, pane)
  -- Show workspace menu
  local choices = {}
  for i, ws in ipairs(workspaces) do
    table.insert(choices, {
      id = tostring(i),
      label = ws.name .. " (" .. ws.path .. ")",
    })
  end

  window:perform_action(
    act.InputSelector {
      action = wezterm.action_callback(function(inner_window, inner_pane, id, label)
        if id then
          local workspace = workspaces[tonumber(id)]
          local success, stdout = wezterm.run_child_process({
            'wezterm-fs-explorer',
            workspace.path
          })

          if success and stdout ~= "" then
            for file_path in stdout:gmatch("[^\r\n]+") do
              inner_pane:send_text("nvim '" .. file_path .. "'\r")
              break
            end
          end
        end
      end),
      title = "Select Workspace",
      choices = choices,
    },
    pane
  )
end

return {
  keys = {
    {
      key = 'w',
      mods = 'CTRL|SHIFT',
      action = wezterm.action_callback(workspace_picker),
    },
  }
}
```

### Configuration 3: Project File Finder

Quick file finder for project files:

```lua
local function project_file_finder(window, pane)
  local cwd = pane:get_current_working_dir()
  local path = cwd and cwd.file_path or "."

  -- Find project root (git repo or current dir)
  local project_root = path
  while project_root ~= "/" do
    if wezterm.glob(project_root .. "/.git")[1] then
      break
    end
    local parent = project_root:match("(.*/)[^/]+/?$")
    if not parent or parent == project_root then
      break
    end
    project_root = parent:sub(1, -2)
  end

  local success, stdout = wezterm.run_child_process({
    'wezterm-fs-explorer',
    '--json',
    project_root
  })

  if success and stdout ~= "" then
    local ok, paths = pcall(wezterm.json_parse, stdout)
    if ok and paths and #paths > 0 then
      -- Open in preferred editor
      local files = table.concat(paths, " ")
      pane:send_text("nvim " .. files .. "\r")
    end
  end
end
```

## Shell Integration

### Bash/Zsh Integration

Add to `~/.bashrc` or `~/.zshrc`:

```bash
# File explorer function
fe() {
  local selected=$(wezterm-fs-explorer "${1:-.}")
  if [ -n "$selected" ]; then
    $EDITOR "$selected"
  fi
}

# Directory navigation
fcd() {
  local selected=$(wezterm-fs-explorer --json "${1:-.}" | jq -r '.[0]')
  if [ -n "$selected" ]; then
    if [ -d "$selected" ]; then
      cd "$selected"
    else
      cd "$(dirname "$selected")"
    fi
  fi
}

# Multi-file selection
fsel() {
  wezterm-fs-explorer --json "${1:-.}" | jq -r '.[]'
}

# Keybinding: Ctrl+O for file explorer
bind '"\C-o":"fe\n"'

# Keybinding: Ctrl+G for directory jump
bind '"\C-g":"fcd\n"'
```

### PowerShell Integration

Add to PowerShell profile (`$PROFILE`):

```powershell
# File explorer function
function fe {
    param([string]$Path = ".")
    $selected = wezterm-fs-explorer $Path
    if ($selected) {
        & $env:EDITOR $selected
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

# Multi-file selection
function fsel {
    param([string]$Path = ".")
    $json = wezterm-fs-explorer --json $Path
    if ($json) {
        ($json | ConvertFrom-Json)
    }
}

# Set up key handlers (PSReadLine)
Set-PSReadLineKeyHandler -Key Ctrl+o -ScriptBlock {
    [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
    [Microsoft.PowerShell.PSConsoleReadLine]::Insert("fe")
    [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
}

Set-PSReadLineKeyHandler -Key Ctrl+g -ScriptBlock {
    [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
    [Microsoft.PowerShell.PSConsoleReadLine]::Insert("fcd")
    [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
}
```

## Neovim Integration

### Neovim Terminal Integration

Add to `~/.config/nvim/init.lua`:

```lua
-- File explorer in terminal buffer
vim.api.nvim_create_user_command('Explorer', function()
  vim.cmd('term wezterm-fs-explorer')
  vim.cmd('startinsert')
end, {})

-- Quick file picker
vim.api.nvim_create_user_command('FilePicker', function()
  local handle = io.popen('wezterm-fs-explorer --json ' .. vim.fn.getcwd())
  local result = handle:read("*a")
  handle:close()

  local ok, paths = pcall(vim.json.decode, result)
  if ok and paths and #paths > 0 then
    for _, path in ipairs(paths) do
      vim.cmd('edit ' .. path)
    end
  end
end, {})

-- Keybindings
vim.keymap.set('n', '<leader>e', ':Explorer<CR>', { noremap = true })
vim.keymap.set('n', '<leader>fe', ':FilePicker<CR>', { noremap = true })
```

## Complete Example Configuration

Full `wezterm.lua` with all integration patterns:

```lua
local wezterm = require 'wezterm'
local act = wezterm.action

local config = {}

-- Helper function to run explorer
local function run_explorer(path, json_mode)
  local args = { 'wezterm-fs-explorer' }
  if json_mode then
    table.insert(args, '--json')
  end
  table.insert(args, path)

  local success, stdout, stderr = wezterm.run_child_process(args)
  return success, stdout, stderr
end

-- File picker
local function file_picker(window, pane)
  local cwd = pane:get_current_working_dir()
  local path = cwd and cwd.file_path or os.getenv("HOME")

  local success, stdout = run_explorer(path, false)
  if success and stdout ~= "" then
    for file_path in stdout:gmatch("[^\r\n]+") do
      pane:send_text("$EDITOR '" .. file_path .. "'\r")
      break
    end
  end
end

-- Directory jumper
local function quick_cd(window, pane)
  local home = os.getenv("HOME") or os.getenv("USERPROFILE")
  local success, stdout = run_explorer(home, true)

  if success and stdout ~= "" then
    local ok, paths = pcall(wezterm.json_parse, stdout)
    if ok and paths and #paths > 0 then
      pane:send_text("cd '" .. paths[1] .. "'\r")
    end
  end
end

-- Split navigator
local function split_navigator(window, pane)
  local tab = window:mux_window():active_tab()
  local new_pane = tab:split_pane({
    direction = 'Right',
    size = 0.4,
    cwd = pane:get_current_working_dir(),
  })
  new_pane:send_text('wezterm-fs-explorer\n')
end

-- Key bindings
config.keys = {
  -- File operations
  { key = 'e', mods = 'CTRL|SHIFT', action = wezterm.action_callback(file_picker) },
  { key = 'd', mods = 'CTRL|SHIFT', action = wezterm.action_callback(quick_cd) },
  { key = 'n', mods = 'CTRL|SHIFT', action = wezterm.action_callback(split_navigator) },

  -- Direct spawn in new tab
  {
    key = 'o',
    mods = 'CTRL|SHIFT',
    action = act.SpawnCommandInNewTab {
      args = { 'wezterm-fs-explorer' },
    },
  },
}

-- Font with Nerd Font support
config.font = wezterm.font 'JetBrains Mono Nerd Font'
config.font_size = 12.0

return config
```

## Troubleshooting

### Issue: Explorer doesn't open
**Solution**: Check if binary is in PATH
```bash
which wezterm-fs-explorer  # Unix
where.exe wezterm-fs-explorer  # Windows
```

### Issue: No output captured
**Solution**: Check file selection (press Enter after selecting)

### Issue: Icons not displaying
**Solution**: Install and configure Nerd Font in WezTerm

### Issue: Git status slow
**Solution**: Use smaller repositories or disable git integration

## Performance Tips

1. **Lazy Loading**: Only launch explorer when needed
2. **Reuse Panes**: Don't spawn new panes for each operation
3. **Cache Results**: Store frequently accessed paths
4. **JSON Mode**: Use `--json` for programmatic access (faster parsing)

## Next Steps

- Explore keybindings customization
- Create custom workflows
- Integrate with your editor
- Build shell functions
- Create project templates

For more information, see the main [README.md](README.md).