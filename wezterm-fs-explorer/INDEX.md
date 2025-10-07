# WezTerm Filesystem Explorer - Documentation Index

Welcome to the complete documentation for `wezterm-fs-explorer`. This index will help you find the information you need.

## 🚀 New User? Start Here

1. **[GETTING_STARTED.md](GETTING_STARTED.md)** - Complete installation and setup guide (15 min read)
   - Prerequisites and installation
   - First use and basic navigation
   - Shell integration examples
   - Troubleshooting common issues

2. **[QUICKREF.md](QUICKREF.md)** - Quick reference card (5 min read)
   - All keybindings in one place
   - Command-line options
   - Visual indicators explained
   - Performance characteristics

## 📚 Core Documentation

### User Guides

| Document | Purpose | Read When |
|----------|---------|-----------|
| [README.md](README.md) | Main documentation, features overview | You want to understand what the tool can do |
| [GETTING_STARTED.md](GETTING_STARTED.md) | Step-by-step installation guide | You're installing for the first time |
| [QUICKREF.md](QUICKREF.md) | Quick reference card | You need to look up a keybinding |
| [WEZTERM_INTEGRATION.md](WEZTERM_INTEGRATION.md) | WezTerm integration patterns | You want to integrate with WezTerm |

### Developer Guides

| Document | Purpose | Read When |
|----------|---------|-----------|
| [BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md) | Detailed build guide | You're building from source |
| [TESTING.md](TESTING.md) | Comprehensive testing guide | You're contributing or testing |
| [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) | Architecture and design | You want to understand the code |
| [CHANGELOG.md](CHANGELOG.md) | Version history | You want to see what's changed |

### Reference

| Document | Purpose | Read When |
|----------|---------|-----------|
| [LICENSE](LICENSE) | MIT License | You need to know licensing terms |
| [INDEX.md](INDEX.md) | This file - navigation guide | You're looking for documentation |

## 🎯 Quick Navigation by Task

### Installation Tasks

| What You Want to Do | Read This | Time |
|---------------------|-----------|------|
| Install for the first time | [GETTING_STARTED.md](GETTING_STARTED.md) | 15 min |
| Build from source | [BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md) | 30 min |
| Troubleshoot build issues | [BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md#troubleshooting) | 10 min |
| Verify installation | [GETTING_STARTED.md](GETTING_STARTED.md#verification) | 2 min |

### Usage Tasks

| What You Want to Do | Read This | Time |
|---------------------|-----------|------|
| Learn basic navigation | [QUICKREF.md](QUICKREF.md#navigation-keys) | 5 min |
| Set up shell functions | [GETTING_STARTED.md](GETTING_STARTED.md#shell-integration) | 5 min |
| Learn all keybindings | [QUICKREF.md](QUICKREF.md) | 10 min |
| Understand file operations | [QUICKREF.md](QUICKREF.md#file-operations) | 5 min |

### Integration Tasks

| What You Want to Do | Read This | Time |
|---------------------|-----------|------|
| Integrate with WezTerm | [WEZTERM_INTEGRATION.md](WEZTERM_INTEGRATION.md) | 20 min |
| Set up keybindings | [examples/wezterm-config-example.lua](examples/wezterm-config-example.lua) | 10 min |
| Create custom workflows | [WEZTERM_INTEGRATION.md](WEZTERM_INTEGRATION.md#integration-patterns) | 30 min |
| Use JSON output mode | [QUICKREF.md](QUICKREF.md#json-output-format) | 5 min |

### Development Tasks

| What You Want to Do | Read This | Time |
|---------------------|-----------|------|
| Understand architecture | [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) | 20 min |
| Run tests | [TESTING.md](TESTING.md) | 30 min |
| Contribute code | [BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md) + [TESTING.md](TESTING.md) | 1 hour |
| Debug issues | [TESTING.md](TESTING.md#troubleshooting) | Varies |

## 📖 Documentation by Role

### 👤 End Users

If you just want to use the tool:

1. **[GETTING_STARTED.md](GETTING_STARTED.md)** - Install and set up
2. **[QUICKREF.md](QUICKREF.md)** - Learn keybindings
3. **[WEZTERM_INTEGRATION.md](WEZTERM_INTEGRATION.md)** - Integrate with WezTerm
4. Keep [QUICKREF.md](QUICKREF.md) handy as a reference

**Total Reading Time**: ~30 minutes

### 💻 Power Users

If you want to customize and integrate deeply:

1. **[GETTING_STARTED.md](GETTING_STARTED.md)** - Install and set up
2. **[QUICKREF.md](QUICKREF.md)** - Learn all features
3. **[WEZTERM_INTEGRATION.md](WEZTERM_INTEGRATION.md)** - Study all integration patterns
4. **[examples/wezterm-config-example.lua](examples/wezterm-config-example.lua)** - Copy and customize
5. **[README.md](README.md)** - Understand advanced features

**Total Reading Time**: ~1.5 hours

### 🔧 Developers

If you want to contribute or modify the code:

1. **[BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md)** - Set up development environment
2. **[PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)** - Understand architecture
3. **[TESTING.md](TESTING.md)** - Learn testing practices
4. **[CHANGELOG.md](CHANGELOG.md)** - Review history
5. Source code in `src/` directory

**Total Reading Time**: ~2-3 hours

### 🏢 System Administrators

If you're deploying for a team:

1. **[BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md)** - Build for deployment
2. **[GETTING_STARTED.md](GETTING_STARTED.md)** - Understand installation
3. **[WEZTERM_INTEGRATION.md](WEZTERM_INTEGRATION.md)** - Plan integration strategy
4. **[TESTING.md](TESTING.md)** - Verify deployment

**Total Reading Time**: ~1 hour

## 📂 File Organization

```
wezterm-fs-explorer/
│
├── 📄 INDEX.md (you are here)          - Navigation guide
├── 📄 README.md                        - Main documentation
├── 📄 GETTING_STARTED.md              - Installation guide
├── 📄 QUICKREF.md                      - Quick reference
├── 📄 LICENSE                          - MIT License
│
├── 🔧 Build & Development
│   ├── BUILD_INSTRUCTIONS.md          - Detailed build guide
│   ├── TESTING.md                     - Testing guide
│   ├── PROJECT_SUMMARY.md             - Architecture overview
│   ├── CHANGELOG.md                   - Version history
│   ├── Cargo.toml                     - Rust dependencies
│   └── build.ps1                      - Build script
│
├── 🔌 Integration
│   ├── WEZTERM_INTEGRATION.md         - WezTerm integration guide
│   └── examples/
│       └── wezterm-config-example.lua - Complete config example
│
└── 💻 Source Code
    └── src/
        ├── main.rs                    - Entry point
        ├── app.rs                     - Application logic
        ├── ui.rs                      - UI rendering
        ├── file_entry.rs              - File handling
        ├── git_status.rs              - Git integration
        ├── icons.rs                   - Icon mapping
        ├── operations.rs              - File operations
        ├── error.rs                   - Error types
        └── keybindings.rs             - Key mappings
```

## 🎓 Learning Path

### Beginner Path (30 minutes)

1. Read [GETTING_STARTED.md](GETTING_STARTED.md) (15 min)
2. Install and test the tool (10 min)
3. Skim [QUICKREF.md](QUICKREF.md) (5 min)
4. Start using with basic navigation (j/k/l/h)

**Goal**: Basic file browsing

### Intermediate Path (1 hour)

1. Complete Beginner Path
2. Set up shell integration from [GETTING_STARTED.md](GETTING_STARTED.md#shell-integration) (10 min)
3. Read [QUICKREF.md](QUICKREF.md) fully (10 min)
4. Try file operations (delete, rename, etc.) (10 min)
5. Read [WEZTERM_INTEGRATION.md](WEZTERM_INTEGRATION.md) basics (20 min)

**Goal**: Efficient workflow integration

### Advanced Path (2 hours)

1. Complete Intermediate Path
2. Read [WEZTERM_INTEGRATION.md](WEZTERM_INTEGRATION.md) fully (30 min)
3. Study [examples/wezterm-config-example.lua](examples/wezterm-config-example.lua) (15 min)
4. Customize your own integration (30 min)
5. Read [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) (20 min)

**Goal**: Custom workflows and deep integration

### Expert Path (4+ hours)

1. Complete Advanced Path
2. Read [BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md) (30 min)
3. Read [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) fully (30 min)
4. Review source code (1-2 hours)
5. Read [TESTING.md](TESTING.md) (30 min)
6. Consider contributing features

**Goal**: Code contributions and customization

## 🔍 Finding Information

### By Keyword

| Keyword | Document | Section |
|---------|----------|---------|
| Install | [GETTING_STARTED.md](GETTING_STARTED.md) | Installation |
| Keybindings | [QUICKREF.md](QUICKREF.md) | Navigation Keys, File Operations |
| WezTerm | [WEZTERM_INTEGRATION.md](WEZTERM_INTEGRATION.md) | All sections |
| Shell | [GETTING_STARTED.md](GETTING_STARTED.md) | Shell Integration |
| Build | [BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md) | Build Methods |
| Test | [TESTING.md](TESTING.md) | All sections |
| Git | [QUICKREF.md](QUICKREF.md) | Git Status |
| JSON | [QUICKREF.md](QUICKREF.md) | JSON Output Format |
| Performance | [QUICKREF.md](QUICKREF.md) | Performance Hints |
| Icons | [GETTING_STARTED.md](GETTING_STARTED.md) | Nerd Font installation |
| Architecture | [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) | Technical Implementation |

### By Error Message

| Error | Document | Section |
|-------|----------|---------|
| "command not found" | [GETTING_STARTED.md](GETTING_STARTED.md) | Troubleshooting |
| "linker error" | [BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md) | Troubleshooting |
| "Icons not displaying" | [GETTING_STARTED.md](GETTING_STARTED.md) | Troubleshooting |
| "Permission denied" | [TESTING.md](TESTING.md) | Error Handling Testing |
| Git status slow | [QUICKREF.md](QUICKREF.md) | Troubleshooting |

## 📊 Documentation Statistics

| Document | Lines | Reading Time | Target Audience |
|----------|-------|--------------|-----------------|
| INDEX.md | ~400 | 5 min | Everyone |
| README.md | ~500 | 20 min | End users |
| GETTING_STARTED.md | ~450 | 15 min | New users |
| QUICKREF.md | ~350 | 5 min | Daily reference |
| WEZTERM_INTEGRATION.md | ~700 | 30 min | Power users |
| BUILD_INSTRUCTIONS.md | ~450 | 20 min | Developers |
| TESTING.md | ~550 | 30 min | Contributors |
| PROJECT_SUMMARY.md | ~350 | 20 min | Developers |
| CHANGELOG.md | ~300 | 10 min | All users |
| **Total** | **~4,050** | **~2.5 hours** | - |

## 🎯 Common Scenarios

### "I just want to try it out"
→ [GETTING_STARTED.md](GETTING_STARTED.md) → Quick Start section

### "How do I press this key?"
→ [QUICKREF.md](QUICKREF.md) → Navigation Keys

### "It won't compile"
→ [BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md) → Troubleshooting

### "How do I integrate with WezTerm?"
→ [WEZTERM_INTEGRATION.md](WEZTERM_INTEGRATION.md) → Integration Patterns

### "I want to customize it"
→ [examples/wezterm-config-example.lua](examples/wezterm-config-example.lua)

### "What changed in the new version?"
→ [CHANGELOG.md](CHANGELOG.md)

### "I found a bug"
→ [TESTING.md](TESTING.md) → Bug Reporting Template

### "I want to contribute"
→ [BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md) + [TESTING.md](TESTING.md)

## 📞 Getting Help

1. **Check this INDEX** - Find the right document
2. **Read the relevant documentation** - Most questions are answered
3. **Review examples** - `examples/` directory has working code
4. **Check QUICKREF.md** - Quick answers for common questions
5. **Open a GitHub issue** - If documentation doesn't help

## 🗺️ Next Steps

Based on your role, jump to:

- **New User?** → [GETTING_STARTED.md](GETTING_STARTED.md)
- **Need Quick Reference?** → [QUICKREF.md](QUICKREF.md)
- **Want Full Documentation?** → [README.md](README.md)
- **Integrating with WezTerm?** → [WEZTERM_INTEGRATION.md](WEZTERM_INTEGRATION.md)
- **Building from Source?** → [BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md)
- **Contributing?** → [TESTING.md](TESTING.md) + [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)

---

**Document Version**: 1.0
**Last Updated**: 2024-01-XX
**Project Version**: 0.1.0

Happy exploring! 🚀