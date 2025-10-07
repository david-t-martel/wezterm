# Testing Guide for WezTerm Filesystem Explorer

## Pre-Build Testing Checklist

Before building the project, verify these prerequisites:

### 1. Environment Setup

```powershell
# Windows
rustc --version          # Should show 1.70+
cargo --version          # Should match rustc
git --version            # For git integration

# Verify Nerd Font installed
# Check WezTerm font config includes Nerd Font
```

```bash
# Linux/macOS
rustc --version
cargo --version
git --version

# Check for required build tools
gcc --version  # or clang --version
pkg-config --version
```

### 2. Dependency Check

```bash
# Verify Cargo.toml dependencies
cargo tree

# Check for conflicts
cargo check --all-features

# Verify dependency versions
cargo outdated  # (requires cargo-outdated)
```

## Build Testing

### 1. Debug Build

```powershell
# Windows
cd C:\Users\david\wezterm\wezterm-fs-explorer
cargo clean
cargo build

# Verify binary exists
Test-Path target\debug\wezterm-fs-explorer.exe

# Check binary size
(Get-Item target\debug\wezterm-fs-explorer.exe).Length / 1MB
```

```bash
# Linux/macOS
cd ~/wezterm/wezterm-fs-explorer
cargo clean
cargo build

# Verify binary
ls -lh target/debug/wezterm-fs-explorer

# Check dependencies
ldd target/debug/wezterm-fs-explorer  # Linux
otool -L target/debug/wezterm-fs-explorer  # macOS
```

### 2. Release Build

```bash
# Build optimized version
cargo build --release

# Verify optimizations applied
cargo rustc --release -- --print cfg | grep opt

# Check binary size (should be < 5MB)
ls -lh target/release/wezterm-fs-explorer
```

### 3. Build Profiles

```bash
# Test release-fast profile
cargo build --profile release-fast

# Compare sizes
ls -lh target/*/wezterm-fs-explorer*

# Run benchmarks on each
time cargo run --release -- /large/directory
time cargo run --profile release-fast -- /large/directory
```

## Functional Testing

### 1. Basic Functionality

```bash
# Test help output
./target/release/wezterm-fs-explorer --help

# Test version
./target/release/wezterm-fs-explorer --version

# Test current directory
./target/release/wezterm-fs-explorer .

# Test specific directory
./target/release/wezterm-fs-explorer ~/projects

# Test JSON mode
./target/release/wezterm-fs-explorer --json . | jq .
```

### 2. Navigation Testing

**Test Case: Basic Navigation**
1. Launch explorer in test directory
2. Press `j` to move down
3. Press `k` to move up
4. Press `g` to go to top
5. Press `G` to go to bottom
6. Press `h` to go to parent
7. Press `l` to enter directory

**Expected**: Smooth navigation, correct selection highlighting

**Test Case: Search**
1. Launch explorer
2. Press `/`
3. Type search term
4. Verify filtered results
5. Press `Esc` to cancel

**Expected**: Real-time filtering, correct matches

### 3. File Operations Testing

#### Setup Test Environment

```bash
# Create test directory structure
mkdir -p test_explorer/{dir1,dir2,dir3}
cd test_explorer
echo "content1" > file1.txt
echo "content2" > file2.txt
echo "content3" > dir1/file3.txt
ln -s file1.txt symlink1.txt  # Unix only
touch .hidden_file
```

**Test Case: File Selection**
1. Launch in test directory
2. Navigate to file1.txt
3. Press `Space` to select
4. Verify checkmark appears
5. Navigate to file2.txt
6. Press `Space` to select
7. Press `Enter` to output
8. Verify both files in output

**Test Case: File Deletion**
1. Navigate to file to delete
2. Press `d`
3. Confirm with `y`
4. Verify file removed from list
5. Verify file deleted from filesystem

**Test Case: File Rename**
1. Navigate to file
2. Press `r`
3. Type new name
4. Press `Enter`
5. Verify renamed in list and filesystem

**Test Case: New File Creation**
1. Press `n`
2. Type "newfile.txt"
3. Press `Enter`
4. Verify file appears in list
5. Verify file exists on filesystem

**Test Case: New Directory Creation**
1. Press `n`
2. Type "newdir/"
3. Press `Enter`
4. Verify directory appears in list
5. Verify directory exists on filesystem

### 4. Display Testing

**Test Case: Hidden Files**
1. Launch in directory with hidden files
2. Press `.` to toggle
3. Verify hidden files appear
4. Press `.` again
5. Verify hidden files disappear

**Test Case: Preview Pane**
1. Launch explorer
2. Press `Tab`
3. Verify preview pane appears
4. Navigate to different files
5. Verify preview updates
6. Press `Tab` again
7. Verify preview pane disappears

**Test Case: Icons and Colors**
1. Launch in directory with various file types
2. Verify icons appear correctly:
   - `` for directories
   - `` for .rs files
   - `` for .py files
   - `` for .js files
3. Verify color coding matches file types

### 5. Git Integration Testing

**Setup Git Test**
```bash
# Create git repository
mkdir git_test && cd git_test
git init
echo "tracked" > tracked.txt
git add tracked.txt
git commit -m "Initial commit"

# Modify file
echo "modified" > tracked.txt

# Create new file
echo "new" > untracked.txt

# Stage a file
echo "staged" > staged.txt
git add staged.txt
```

**Test Case: Git Status**
1. Launch in git repository
2. Verify git indicators appear:
   - `M` for modified tracked.txt
   - `?` for untracked.txt
   - `A` for staged.txt
3. Navigate to different files
4. Verify indicators remain correct

### 6. Performance Testing

**Test Case: Startup Time**
```bash
# Measure startup time
time ./target/release/wezterm-fs-explorer --version

# Should be < 50ms
```

**Test Case: Large Directory**
```bash
# Create large directory
mkdir large_test
cd large_test
for i in {1..10000}; do touch "file_$i.txt"; done

# Test loading
time ../target/release/wezterm-fs-explorer .

# Should load in < 200ms
```

**Test Case: Memory Usage**
```powershell
# Windows: Monitor memory while running
$process = Start-Process -FilePath ".\target\release\wezterm-fs-explorer.exe" -PassThru
Start-Sleep 2
Get-Process -Id $process.Id | Select-Object WS, PM
$process | Stop-Process
```

```bash
# Linux: Monitor memory
/usr/bin/time -v ./target/release/wezterm-fs-explorer . 2>&1 | grep "Maximum resident"

# Should be < 100MB for 10K files
```

### 7. Error Handling Testing

**Test Case: Non-existent Directory**
```bash
./target/release/wezterm-fs-explorer /nonexistent
# Expected: Clear error message, exit code 1
echo $?  # Should be 1
```

**Test Case: Permission Denied**
```bash
# Unix only
sudo mkdir /root/test_dir
sudo chmod 000 /root/test_dir
./target/release/wezterm-fs-explorer /root/test_dir
# Expected: Permission error, graceful exit
```

**Test Case: Symlink Cycles**
```bash
# Unix only
ln -s . cyclic_symlink
./target/release/wezterm-fs-explorer .
# Expected: Handle gracefully, show symlink
```

### 8. Cross-Platform Testing

**Windows-Specific Tests**
```powershell
# Test Windows paths
.\target\release\wezterm-fs-explorer.exe C:\Windows\System32

# Test UNC paths
.\target\release\wezterm-fs-explorer.exe \\server\share

# Test long paths
.\target\release\wezterm-fs-explorer.exe "C:\Very\Long\Path\..."
```

**Unix-Specific Tests**
```bash
# Test absolute paths
./target/release/wezterm-fs-explorer /usr/local

# Test home directory
./target/release/wezterm-fs-explorer ~

# Test relative paths
./target/release/wezterm-fs-explorer ../..

# Test paths with spaces
./target/release/wezterm-fs-explorer "/path with spaces/"
```

## Integration Testing

### 1. WezTerm Integration

**Test Case: Direct Launch**
```lua
-- In wezterm.lua
wezterm.run_child_process({'wezterm-fs-explorer', '/home/user'})
```

**Test Case: Capture Output**
```lua
local success, stdout, stderr = wezterm.run_child_process({
  'wezterm-fs-explorer',
  '--json',
  '.'
})
assert(success == true)
assert(stdout ~= "")
```

### 2. Shell Integration

**Test Case: Bash Function**
```bash
fe() { $EDITOR "$(wezterm-fs-explorer "${1:-.}")"; }
fe ~/projects
# Expected: Opens file in editor
```

**Test Case: PowerShell Function**
```powershell
function fe { param($Path = "."); & $env:EDITOR (wezterm-fs-explorer $Path) }
fe C:\projects
# Expected: Opens file in editor
```

### 3. JSON Output Testing

**Test Case: JSON Format**
```bash
output=$(./target/release/wezterm-fs-explorer --json .)
echo "$output" | jq .
# Expected: Valid JSON array of paths
```

**Test Case: JSON Piping**
```bash
./target/release/wezterm-fs-explorer --json . | jq -r '.[]' | head -n 5
# Expected: List of file paths
```

## Regression Testing

### Before Each Release

1. **Build Tests**
   - [ ] Clean build succeeds
   - [ ] Release build succeeds
   - [ ] All profiles build successfully
   - [ ] Binary size within expected range

2. **Core Functionality**
   - [ ] Basic navigation works
   - [ ] File selection works
   - [ ] Search/filter works
   - [ ] All file operations work

3. **Display**
   - [ ] Icons render correctly
   - [ ] Colors display properly
   - [ ] Preview pane works
   - [ ] Hidden files toggle works

4. **Git Integration**
   - [ ] Git status detects repository
   - [ ] Status indicators show correctly
   - [ ] Works in non-git directories

5. **Performance**
   - [ ] Startup < 50ms
   - [ ] 10K files load < 200ms
   - [ ] Memory < 100MB for 10K files
   - [ ] Navigation remains responsive

6. **Error Handling**
   - [ ] Non-existent paths handled
   - [ ] Permission errors caught
   - [ ] Invalid input handled

7. **Integration**
   - [ ] WezTerm integration works
   - [ ] Shell functions work
   - [ ] JSON output valid

## Continuous Testing

### Automated Test Script

```bash
#!/bin/bash
# test_all.sh

set -e

echo "Building project..."
cargo build --release

echo "Running unit tests..."
cargo test

echo "Testing basic functionality..."
./target/release/wezterm-fs-explorer --version
./target/release/wezterm-fs-explorer --help

echo "Testing JSON output..."
output=$(./target/release/wezterm-fs-explorer --json .)
echo "$output" | jq . > /dev/null

echo "Performance test..."
time ./target/release/wezterm-fs-explorer /usr

echo "All tests passed!"
```

## Benchmark Testing

```bash
# Install criterion for benchmarks
cargo install cargo-criterion

# Run benchmarks
cargo bench

# Compare benchmarks
cargo bench -- --baseline old
cargo bench -- --baseline new
```

## Manual Testing Checklist

Print this checklist for manual testing:

- [ ] Build completes without errors
- [ ] Binary launches successfully
- [ ] Help text displays correctly
- [ ] Version displays correctly
- [ ] Can navigate with j/k keys
- [ ] Can enter directories with l
- [ ] Can go to parent with h
- [ ] Can jump to top with g
- [ ] Can jump to bottom with G
- [ ] Search filters correctly
- [ ] File selection toggles
- [ ] Multi-select works
- [ ] Delete confirms before removing
- [ ] Rename changes filename
- [ ] New file creates correctly
- [ ] New directory creates correctly
- [ ] Hidden files toggle works
- [ ] Preview pane toggles
- [ ] Icons display correctly
- [ ] Colors display correctly
- [ ] Git status shows correctly
- [ ] JSON output is valid
- [ ] Quit with q works
- [ ] Force quit with Ctrl+c works
- [ ] Exit codes correct

## Bug Reporting Template

When finding bugs, report with:

```markdown
**Environment:**
- OS: [Windows 11 / Ubuntu 22.04 / macOS 13]
- Rust version: [1.70.0]
- Terminal: [WezTerm 20231231]
- Binary size: [3.2 MB]

**Steps to Reproduce:**
1. Launch explorer in directory X
2. Press key Y
3. Observe behavior Z

**Expected Behavior:**
[What should happen]

**Actual Behavior:**
[What actually happened]

**Logs/Output:**
```
[Paste any error messages]
```

**Additional Context:**
[Any other relevant information]
```

## Next Steps

After testing passes:
1. Tag release in git
2. Build final binaries
3. Generate documentation
4. Update changelog
5. Create GitHub release

For questions or issues, see [README.md](README.md) or open a GitHub issue.