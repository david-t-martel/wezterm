# WezTerm Quick Context

## Current Task
- ✅ Comprehensive architectural analysis completed
- ✅ Custom utilities (wezterm-fs-explorer, wezterm-watch) documented
- ✅ Architecture knowledge base created for AI assistant reference
- ✅ AI Assistant Module design reviewed and implementation roadmap defined
- ✅ Build system optimization and quality checks completed

## Immediate Goals
1. ✅ Repository forked and configured
2. ✅ CLAUDE.md comprehensive development guide created
3. ✅ Architecture knowledge base indexed (`.claude/context/wezterm-architecture-knowledge.md`)
4. ✅ Build system verification and sccache statistics (23.12% hit rate, 4 GiB cache)
5. ✅ Quality checks completed (fmt: ✅, clippy: ✅, all utilities: 0 warnings)
6. ✅ All changes committed and pushed to fork

## Recent Decisions
- ✅ Created comprehensive architecture documentation
- ✅ Documented dual GPU rendering backend (OpenGL/WebGPU)
- ✅ Analyzed trait-based extensibility patterns
- ✅ Reviewed AI module design (mistral.rs + MCP protocol)
- ✅ Fixed all clippy warnings in custom utilities (wezterm-fs-explorer: 10→0, wezterm-watch: 9→0)
- ✅ Fixed critical clippy issue in wezterm-dynamic/derive (wrong_self_convention)
- ✅ Corrected invalid clippy.toml configuration
- Use CLAUDE.md format for Claude Code documentation
- Maintain cargo nextest for testing (not cargo test)
- Follow existing Rust workspace patterns
- Respect GPU acceleration architecture
- Utilize parallel rust-pro agents for accelerated code quality improvements

## Active Blockers
- None - All quality checks passing
- Repository ready for development work

## Key Commands
```bash
# Build
make build

# Test
make test

# Format
make fmt

# Quick check
cargo check
```

## Working Directory
`C:\Users\david\wezterm`

## Repository
github.com/david-t-martel/wezterm (forked from wez/wezterm)