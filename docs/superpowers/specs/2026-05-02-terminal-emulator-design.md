# Terminal Emulator Design

**Date:** 2026-05-02  
**Type:** Terminal Emulator with Tiling Support  
**Language:** Rust  

## Overview

A CPU-only terminal emulator that supports multiple tiled terminal panes (like Ghostty) without GPU dependency. Features include splitting terminals horizontally/vertically, navigating between panes with arrow keys, and automatic cleanup when panes exit.

## Architecture

### System Components

1. **PTY Manager** - Spawns and manages terminal processes using pseudoterminals
2. **Tiling Layout Engine** - Manages grid of terminal panes, handles splits and navigation  
3. **Terminal Renderer** - Renders terminal content to host terminal using `termion`

### Main Loop
Read from all active PTYs → Parse output → Update internal state → Render to host terminal → Handle user input → Update layout/forward to focused PTY

## Components

### PTY Manager

**Responsibilities:**
- Uses `pty` crate to fork child processes (shell, etc.)
- Maintains bidirectional pipes: parent→child (stdin), child→parent (stdout/stderr)
- Thread per PTY reading output into a buffer
- Clean shutdown: send SIGTERM when pane exits

**Dependencies:** `pty` crate

### Tiling Layout Engine

**Responsibilities:**
- Grid-based coordinate system (row, col)
- Each pane tracks: position (x, y), size (width, height), associated PTY
- Split operation: creates new pane, adjusts dimensions of existing panes
- Navigation: maintains focus state, moves to adjacent pane based on direction
- Array-based storage for simplicity

**Key Operations:**
- `split_horizontal()`: Create new pane below current, divide vertical space
- `split_vertical()`: Create new pane right of current, divide horizontal space
- `navigate(direction)`: Move focus to adjacent pane if exists
- `close_pane(id)`: Remove pane and reclaim space

### Terminal Renderer

**Responsibilities:**
- Uses `termion` for host terminal interaction
- Maintains screen buffer (grid of cells with character + style)
- ANSI escape sequence parser for colors, cursor movements
- Optimized rendering: only redraw changed cells
- Cursor positioning for focused pane's virtual cursor

**Dependencies:** `termion` crate

## Keyboard Shortcuts

- **Split horizontal:** `Ctrl+Shift+H`
- **Split vertical:** `Ctrl+Shift+V`
- **Navigate panes:** `Ctrl+Shift+Arrow` keys
- **Close pane:** Type `exit` in terminal (automatic cleanup)

## Data Flow

### Terminal Output Flow
PTY → Reader Thread → Parse ANSI → Update Pane Buffer → Mark Dirty Cells → Main Loop → Render Changes to Host

### User Input Flow
Host Terminal → Main Loop → If Navigation: Update Focus; If Split: Create Pane/Adjust Layout; Otherwise: Forward to Focused PTY

### Split Operation Flow
User presses split shortcut → Layout Engine creates new PTY → Adjusts dimensions (divide space) → Spawns new pane → Updates focus

### Navigation Flow
User presses arrow keys → Layout Engine finds adjacent pane in direction → If exists: Update focus and render highlight; If not: Do nothing

## Error Handling

- **PTY spawn failure:** Show error message in host terminal, continue with existing panes
- **Invalid ANSI sequences:** Skip unsupported sequences, log to stderr
- **Split failure (no space):** Show error message, don't create pane
- **Child process crash:** Mark pane as dead, show "Process exited" message, allow closing

## Testing Strategy

### Unit Tests
- Layout engine (split, navigation, boundary cases)
- ANSI escape sequence parser
- Buffer management

### Integration Tests
- PTY spawning and basic I/O forwarding
- Multi-pane coordination
- Keyboard input handling

### Manual Testing
- Run shell in panes
- Test all keyboard shortcuts
- Verify exit behavior
- Stress test with multiple active panes

### Performance Testing
- Verify responsiveness with multiple active panes
- Check memory usage over time
- Measure render performance

## Constraints

- No GPU dependency (CPU-only rendering)
- Compatible with older systems
- Single binary deployment
- Minimal external dependencies

## Success Criteria

- Terminal panes can be split horizontally and vertically
- Navigation between panes works with arrow keys
- Panes close cleanly when processes exit
- Performance remains acceptable with multiple panes
- Works on older hardware without GPU acceleration
