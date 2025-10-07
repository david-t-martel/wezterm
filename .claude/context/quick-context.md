# WezTerm Quick Context

## Current Task
- ✅ Comprehensive architectural analysis completed
- ✅ Custom utilities (wezterm-fs-explorer, wezterm-watch) documented
- ✅ Architecture knowledge base created for AI assistant reference
- ✅ AI Assistant Module design reviewed and implementation roadmap defined
- ⏳ Build system optimization and quality checks

## Immediate Goals
1. ✅ Repository forked and configured
2. ✅ CLAUDE.md comprehensive development guide created
3. ✅ Architecture knowledge base indexed (`.claude/context/wezterm-architecture-knowledge.md`)
4. ⏳ Build system verification and sccache statistics
5. ⏳ Quality checks (fmt, clippy, tests)
6. ⏳ Commit and push accumulated changes

## Recent Decisions
- ✅ Created comprehensive architecture documentation
- ✅ Documented dual GPU rendering backend (OpenGL/WebGPU)
- ✅ Analyzed trait-based extensibility patterns
- ✅ Reviewed AI module design (mistral.rs + MCP protocol)
- Use CLAUDE.md format for Claude Code documentation
- Maintain cargo nextest for testing (not cargo test)
- Follow existing Rust workspace patterns
- Respect GPU acceleration architecture

## Active Blockers
- None currently identified
- Ready for build verification and quality checks

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