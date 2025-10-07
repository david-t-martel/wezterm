-- ============================================================================
-- WEZTERM UTILITIES INTEGRATION MODULE
-- Provides unified interface for wezterm-fs-explorer, wezterm-watch, and wedit
-- ============================================================================

local wezterm = require('wezterm')
local act = wezterm.action

local M = {}

-- ============================================================================
-- CONFIGURATION
-- ============================================================================

M.config = {
  -- Binary locations (will be resolved from PATH if not absolute)
  binaries = {
    explorer = 'wezterm-fs-explorer',
    watch = 'wezterm-watch',
    editor = 'wedit',
  },

  -- Default options for each utility
  explorer = {
    start_path = nil, -- nil = current working directory
    show_hidden = false,
    git_status = true,
  },

  watch = {
    recursive = true,
    git_aware = true,
    debounce_ms = 300,
  },

  editor = {
    theme = 'default',
    line_numbers = true,
    auto_save = false,
  },
}

-- ============================================================================
-- UTILITY FUNCTIONS
-- ============================================================================

--- Get the absolute path to a binary
--- @param name string Binary name or path
--- @return string|nil Absolute path or nil if not found
local function resolve_binary(name)
  -- If absolute path, return as-is
  if name:match('^[/\\]') or name:match('^[A-Za-z]:') then
    return name
  end

  -- Try to find in PATH
  local extensions = { '', '.exe', '.cmd', '.bat' }

  for _, ext in ipairs(extensions) do
    local binary_name = name .. ext
    -- wezterm will search PATH for us
    return binary_name
  end

  return name
end

--- Get current pane's working directory
--- @param pane any WezTerm pane object
--- @return string|nil Current working directory or nil
local function get_cwd(pane)
  local cwd_uri = pane:get_current_working_dir()

  if cwd_uri then
    return cwd_uri.file_path
  end

  return nil
end

--- Create a new split pane with a command
--- @param window any WezTerm window object
--- @param pane any WezTerm pane object
--- @param args table Command arguments
--- @param direction string Split direction ('Right', 'Down', 'Left', 'Up')
--- @return boolean Success status
local function spawn_split(window, pane, args, direction)
  direction = direction or 'Right'

  local split_action = act.SplitPane({
    direction = direction,
    command = { args = args },
    size = { Percent = 50 },
  })

  window:perform_action(split_action, pane)
  return true
end

--- Create a new tab with a command
--- @param window any WezTerm window object
--- @param pane any WezTerm pane object
--- @param args table Command arguments
--- @param title string|nil Tab title
--- @return boolean Success status
local function spawn_tab(window, pane, args, title)
  local spawn_action = act.SpawnCommandInNewTab({
    args = args,
    label = title,
  })

  window:perform_action(spawn_action, pane)
  return true
end

-- ============================================================================
-- FILE EXPLORER
-- ============================================================================

--- Launch filesystem explorer
--- @param options table|nil Options for explorer
--- @return function Action callback
function M.launch_explorer(options)
  options = options or {}

  return wezterm.action_callback(function(window, pane)
    local cwd = options.start_path or get_cwd(pane) or wezterm.home_dir

    local args = { resolve_binary(M.config.binaries.explorer) }

    -- Add path argument
    table.insert(args, cwd)

    -- Add options
    if options.show_hidden or M.config.explorer.show_hidden then
      table.insert(args, '--hidden')
    end

    if options.git_status ~= false and M.config.explorer.git_status then
      table.insert(args, '--git')
    end

    -- Spawn in split or tab
    if options.split then
      spawn_split(window, pane, args, options.direction)
    else
      spawn_tab(window, pane, args, 'Explorer')
    end
  end)
end

--- Quick explorer in split pane
--- @return function Action callback
function M.explorer_split()
  return M.launch_explorer({ split = true, direction = 'Right' })
end

--- Quick explorer in new tab
--- @return function Action callback
function M.explorer_tab()
  return M.launch_explorer({ split = false })
end

-- ============================================================================
-- FILE WATCHER
-- ============================================================================

--- Launch file watcher
--- @param options table|nil Options for watcher
--- @return function Action callback
function M.launch_watcher(options)
  options = options or {}

  return wezterm.action_callback(function(window, pane)
    local cwd = options.watch_path or get_cwd(pane) or wezterm.home_dir

    local args = { resolve_binary(M.config.binaries.watch) }

    -- Add path to watch
    table.insert(args, cwd)

    -- Add options
    if options.recursive ~= false and M.config.watch.recursive then
      table.insert(args, '--recursive')
    end

    if options.git_aware ~= false and M.config.watch.git_aware then
      table.insert(args, '--git')
    end

    if options.debounce_ms or M.config.watch.debounce_ms then
      table.insert(args, '--debounce')
      table.insert(args, tostring(options.debounce_ms or M.config.watch.debounce_ms))
    end

    -- Add pattern filter if provided
    if options.pattern then
      table.insert(args, '--pattern')
      table.insert(args, options.pattern)
    end

    -- Spawn in split or tab
    if options.split then
      spawn_split(window, pane, args, options.direction or 'Down')
    else
      spawn_tab(window, pane, args, 'Watcher')
    end
  end)
end

--- Quick watcher in split pane
--- @return function Action callback
function M.watcher_split()
  return M.launch_watcher({ split = true, direction = 'Down' })
end

--- Quick watcher in new tab
--- @return function Action callback
function M.watcher_tab()
  return M.launch_watcher({ split = false })
end

-- ============================================================================
-- TEXT EDITOR
-- ============================================================================

--- Launch text editor
--- @param file_path string|nil Path to file to edit
--- @param options table|nil Editor options
--- @return function Action callback
function M.launch_editor(file_path, options)
  options = options or {}

  return wezterm.action_callback(function(window, pane)
    local args = { resolve_binary(M.config.binaries.editor) }

    -- Add file path if provided
    if file_path then
      table.insert(args, file_path)
    end

    -- Add options
    if options.line_numbers or M.config.editor.line_numbers then
      table.insert(args, '--line-numbers')
    end

    if options.theme or M.config.editor.theme then
      table.insert(args, '--theme')
      table.insert(args, options.theme or M.config.editor.theme)
    end

    -- Spawn in split or tab
    if options.split then
      spawn_split(window, pane, args, options.direction)
    else
      spawn_tab(window, pane, args, 'Editor')
    end
  end)
end

--- Quick editor in split pane
--- @param file_path string|nil Path to file
--- @return function Action callback
function M.editor_split(file_path)
  return M.launch_editor(file_path, { split = true, direction = 'Right' })
end

--- Quick editor in new tab
--- @param file_path string|nil Path to file
--- @return function Action callback
function M.editor_tab(file_path)
  return M.launch_editor(file_path, { split = false })
end

-- ============================================================================
-- KEYBINDING HELPERS
-- ============================================================================

--- Get default keybindings for utilities
--- @return table Array of keybinding definitions
function M.get_default_keys()
  return {
    -- File Explorer
    {
      key = 'e',
      mods = 'ALT',
      action = M.explorer_split(),
      description = 'Open file explorer in split',
    },
    {
      key = 'E',
      mods = 'ALT|SHIFT',
      action = M.explorer_tab(),
      description = 'Open file explorer in tab',
    },

    -- File Watcher
    {
      key = 'w',
      mods = 'ALT',
      action = M.watcher_split(),
      description = 'Open file watcher in split',
    },
    {
      key = 'W',
      mods = 'ALT|SHIFT',
      action = M.watcher_tab(),
      description = 'Open file watcher in tab',
    },

    -- Text Editor
    {
      key = 'e',
      mods = 'CTRL|ALT',
      action = M.editor_split(nil),
      description = 'Open text editor in split',
    },
    {
      key = 'E',
      mods = 'CTRL|ALT|SHIFT',
      action = M.editor_tab(nil),
      description = 'Open text editor in tab',
    },
  }
end

--- Get launch menu entries
--- @return table Array of launch menu items
function M.get_launch_menu_items()
  return {
    {
      label = 'File Explorer',
      args = { resolve_binary(M.config.binaries.explorer) },
    },
    {
      label = 'File Watcher',
      args = { resolve_binary(M.config.binaries.watch) },
    },
    {
      label = 'Text Editor',
      args = { resolve_binary(M.config.binaries.editor) },
    },
  }
end

-- ============================================================================
-- INTEGRATION HELPERS
-- ============================================================================

--- Apply utility keybindings to config
--- @param config table WezTerm configuration object
--- @param custom_keys table|nil Custom keybindings to merge
function M.apply_keys(config, custom_keys)
  config.keys = config.keys or {}

  -- Add default keys
  local default_keys = M.get_default_keys()
  for _, key in ipairs(default_keys) do
    table.insert(config.keys, {
      key = key.key,
      mods = key.mods,
      action = key.action,
    })
  end

  -- Add custom keys if provided
  if custom_keys then
    for _, key in ipairs(custom_keys) do
      table.insert(config.keys, key)
    end
  end
end

--- Apply utility launch menu items to config
--- @param config table WezTerm configuration object
function M.apply_launch_menu(config)
  config.launch_menu = config.launch_menu or {}

  local menu_items = M.get_launch_menu_items()
  for _, item in ipairs(menu_items) do
    table.insert(config.launch_menu, item)
  end
end

--- Apply all utility integrations to config
--- @param config table WezTerm configuration object
--- @param options table|nil Integration options
function M.setup(config, options)
  options = options or {}

  -- Merge configuration
  if options.binaries then
    for k, v in pairs(options.binaries) do
      M.config.binaries[k] = v
    end
  end

  if options.explorer then
    for k, v in pairs(options.explorer) do
      M.config.explorer[k] = v
    end
  end

  if options.watch then
    for k, v in pairs(options.watch) do
      M.config.watch[k] = v
    end
  end

  if options.editor then
    for k, v in pairs(options.editor) do
      M.config.editor[k] = v
    end
  end

  -- Apply integrations
  if options.keys ~= false then
    M.apply_keys(config, options.custom_keys)
  end

  if options.launch_menu ~= false then
    M.apply_launch_menu(config)
  end

  wezterm.log_info('WezTerm utilities integration loaded')
end

return M