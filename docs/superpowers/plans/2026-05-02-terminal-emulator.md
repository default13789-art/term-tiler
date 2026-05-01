# Terminal Emulator Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a CPU-only terminal emulator with tiling support using Rust

**Architecture:** Three-component system - PTY Manager for process handling, Tiling Layout Engine for pane management, and Terminal Renderer for display. All components communicate through a central main loop.

**Tech Stack:** Rust, `pty` crate for process management, `termion` crate for terminal I/O, standard library for threading

---

## File Structure

- `Cargo.toml` - Project dependencies and metadata
- `src/main.rs` - Entry point and main event loop
- `src/pty.rs` - Pseudoterminal process spawning and I/O
- `src/layout.rs` - Tiling grid layout management
- `src/buffer.rs` - Screen buffer with cell-based storage
- `src/renderer.rs` - Terminal rendering with termion
- `src/ansi.rs` - ANSI escape sequence parser
- `src/input.rs` - Keyboard input handling and routing

---

### Task 1: Project Setup

**Files:**
- Create: `Cargo.toml`

- [ ] **Step 1: Create Cargo.toml with dependencies**

```toml
[package]
name = "term-tiler"
version = "0.1.0"
edition = "2021"

[dependencies]
pty = "0.2"
termion = "2.0"
libc = "0.2"
```

- [ ] **Step 2: Initialize Rust project structure**

Run: `cargo build --message-format=short`
Expected: Project compiles successfully

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "chore: initialize Rust project with dependencies"
```

---

### Task 2: Screen Buffer Implementation

**Files:**
- Create: `src/buffer.rs`
- Create: `src/lib.rs`

- [ ] **Step 1: Write screen buffer tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buffer = Buffer::new(10, 20);
        assert_eq!(buffer.width, 10);
        assert_eq!(buffer.height, 20);
    }

    #[test]
    fn test_cell_write() {
        let mut buffer = Buffer::new(5, 5);
        buffer.write(2, 3, 'A', Style::default());
        assert_eq!(buffer.get(2, 3).unwrap().ch, 'A');
    }

    #[test]
    fn test_clear() {
        let mut buffer = Buffer::new(5, 5);
        buffer.write(1, 1, 'X', Style::default());
        buffer.clear();
        assert_eq!(buffer.get(1, 1).unwrap().ch, ' ');
    }

    #[test]
    fn test_dirty_tracking() {
        let mut buffer = Buffer::new(5, 5);
        assert!(!buffer.is_dirty(2, 2));
        buffer.write(2, 2, 'Y', Style::default());
        assert!(buffer.is_dirty(2, 2));
    }
}
```

- [ ] **Step 2: Create lib.rs**

```rust
pub mod buffer;
pub mod pty;
pub mod layout;
pub mod renderer;
pub mod ansi;
pub mod input;
```

- [ ] **Step 3: Write buffer implementation**

```rust
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq)]
pub struct Style {
    pub fg_color: Color,
    pub bg_color: Color,
    pub bold: bool,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            fg_color: Color::Default,
            bg_color: Color::Default,
            bold: false,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

#[derive(Clone)]
pub struct Cell {
    pub ch: char,
    pub style: Style,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            ch: ' ',
            style: Style::default(),
        }
    }
}

pub struct Buffer {
    cells: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
    dirty: HashSet<(usize, usize)>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![Cell::default(); width]; height];
        Buffer {
            cells,
            width,
            height,
            dirty: HashSet::new(),
        }
    }

    pub fn write(&mut self, x: usize, y: usize, ch: char, style: Style) {
        if x < self.width && y < self.height {
            self.cells[y][x].ch = ch;
            self.cells[y][x].style = style;
            self.dirty.insert((x, y));
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        if x < self.width && y < self.height {
            Some(&self.cells[y][x])
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                *cell = Cell::default();
            }
        }
        self.dirty.clear();
    }

    pub fn is_dirty(&self, x: usize, y: usize) -> bool {
        self.dirty.contains(&(x, y))
    }

    pub fn clear_dirty(&mut self) {
        self.dirty.clear();
    }

    pub fn get_dirty_cells(&self) -> impl Iterator<Item = &(usize, usize)> {
        self.dirty.iter()
    }
}
```

- [ ] **Step 4: Run tests**

Run: `cargo test buffer --lib`
Expected: All tests pass

- [ ] **Step 5: Commit**

```bash
git add src/lib.rs src/buffer.rs
git commit -m "feat: implement screen buffer with dirty tracking"
```

---

### Task 3: ANSI Escape Sequence Parser

**Files:**
- Create: `src/ansi.rs`

- [ ] **Step 1: Write ANSI parser tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_text() {
        let result = parse("hello");
        assert_eq!(result, vec![Action::Write('h'), Action::Write('e'), Action::Write('l'), Action::Write('l'), Action::Write('o')]);
    }

    #[test]
    fn test_cursor_movement() {
        let result = parse("\x1B[2;3H");
        assert_eq!(result, vec![Action::MoveCursor(1, 2)]);
    }

    #[test]
    fn test_color_change() {
        let result = parse("\x1B[31m");
        assert_eq!(result, vec![Action::SetColor(FgColor::Red)]);
    }

    #[test]
    fn test_mixed_content() {
        let result = parse("AB\x1B[31mC\x1B[0mD");
        assert_eq!(result, vec![
            Action::Write('A'),
            Action::Write('B'),
            Action::SetColor(FgColor::Red),
            Action::Write('C'),
            Action::Reset,
            Action::Write('D'),
        ]);
    }
}
```

- [ ] **Step 2: Write ANSI parser implementation**

```rust
pub enum Action {
    Write(char),
    MoveCursor(usize, usize),
    SetFgColor(Color),
    SetBgColor(Color),
    Reset,
    ClearLine,
    ClearScreen,
}

pub enum Color {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

pub fn parse(input: &str) -> Vec<Action> {
    let mut actions = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1B' {
            if let Some(&'[') = chars.peek() {
                chars.next();
                if let Some(seq) = parse_escape_sequence(&mut chars) {
                    actions.push(seq);
                }
            }
        } else if ch != '\r' && ch != '\n' {
            actions.push(Action::Write(ch));
        }
    }

    actions
}

fn parse_escape_sequence(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<Action> {
    let mut params = String::new();

    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() || ch == ';' {
            params.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    if let Some(&end_char) = chars.next() {
        match end_char {
            'H' => {
                let parts: Vec<usize> = params.split(';')
                    .map(|s| s.parse().unwrap_or(1))
                    .collect();
                let row = parts.get(0).copied().unwrap_or(1).saturating_sub(1);
                let col = parts.get(1).copied().unwrap_or(1).saturating_sub(1);
                Some(Action::MoveCursor(col, row))
            }
            'K' => Some(Action::ClearLine),
            'J' => Some(Action::ClearScreen),
            'm' => {
                let codes: Vec<u32> = params.split(';')
                    .filter_map(|s| s.parse().ok())
                    .collect();
                parse_color_codes(&codes)
            }
            _ => None,
        }
    } else {
        None
    }
}

fn parse_color_codes(codes: &[u32]) -> Option<Action> {
    for &code in codes {
        match code {
            0 => return Some(Action::Reset),
            30 => return Some(Action::SetFgColor(Color::Black)),
            31 => return Some(Action::SetFgColor(Color::Red)),
            32 => return Some(Action::SetFgColor(Color::Green)),
            33 => return Some(Action::SetFgColor(Color::Yellow)),
            34 => return Some(Action::SetFgColor(Color::Blue)),
            35 => return Some(Action::SetFgColor(Color::Magenta)),
            36 => return Some(Action::SetFgColor(Color::Cyan)),
            37 => return Some(Action::SetFgColor(Color::White)),
            40 => return Some(Action::SetBgColor(Color::Black)),
            41 => return Some(Action::SetBgColor(Color::Red)),
            42 => return Some(Action::SetBgColor(Color::Green)),
            43 => return Some(Action::SetBgColor(Color::Yellow)),
            44 => return Some(Action::SetBgColor(Color::Blue)),
            45 => return Some(Action::SetBgColor(Color::Magenta)),
            46 => return Some(Action::SetBgColor(Color::Cyan)),
            47 => return Some(Action::SetBgColor(Color::White)),
            _ => continue,
        }
    }
    None
}
```

- [ ] **Step 3: Run tests**

Run: `cargo test ansi --lib`
Expected: All tests pass

- [ ] **Step 4: Commit**

```bash
git add src/ansi.rs
git commit -m "feat: implement ANSI escape sequence parser"
```

---

### Task 4: PTY Manager

**Files:**
- Create: `src/pty.rs`

- [ ] **Step 1: Write PTY manager tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pty_creation() {
        let pty = PTY::new("ls", &["-la"]).unwrap();
        assert!(!pty.master.is_null());
    }

    #[test]
    fn test_write_to_pty() {
        let mut pty = PTY::new("cat", &[]).unwrap();
        pty.write(b"hello\n").unwrap();
    }

    #[test]
    fn test_read_from_pty() {
        let mut pty = PTY::new("echo", &["hello"]).unwrap();
        let output = pty.read().unwrap();
        assert!(output.len() > 0);
    }
}
```

- [ ] **Step 2: Write PTY manager implementation**

```rust
use pty::fork::Fork;
use std::os::unix::io::AsRawFd;
use std::process::Command;

pub struct PTY {
    pub pid: libc::pid_t,
    pub master: i32,
    pub slave: i32,
}

impl PTY {
    pub fn new(command: &str, args: &[&str]) -> Result<Self, String> {
        let fork = Fork::from_ptmx().map_err(|e| format!("PTY fork failed: {}", e))?;
        
        let master = fork.is_parent().ok_or("Failed to create master PTY")?;
        let slave = fork.is_child().ok_or("Failed to create slave PTY")?;
        
        let pid = match fork {
            Fork::Parent(_, child_pid) => child_pid,
            Fork::Child(_) => {
                let mut cmd = Command::new(command);
                cmd.args(args);
                let err = cmd.exec();
                eprintln!("Exec failed: {}", err);
                std::process::exit(1);
            }
        };
        
        Ok(PTY { pid, master, slave })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, String> {
        unsafe {
            let written = libc::write(self.master, data.as_ptr() as *const libc::c_void, data.len());
            if written < 0 {
                Err(format!("Write failed: {}", std::io::Error::last_os_error()))
            } else {
                Ok(written as usize)
            }
        }
    }

    pub fn read(&self) -> Result<Vec<u8>, String> {
        let mut buf = [0u8; 8192];
        unsafe {
            let read = libc::read(self.master, buf.as_mut_ptr() as *mut libc::c_void, buf.len());
            if read < 0 {
                Err(format!("Read failed: {}", std::io::Error::last_os_error()))
            } else {
                Ok(buf[..read as usize].to_vec())
            }
        }
    }

    pub fn is_alive(&self) -> bool {
        unsafe {
            let mut status: libc::c_int = 0;
            let result = libc::waitpid(self.pid, &mut status, libc::WNOHANG);
            result == 0
        }
    }

    pub fn close(&mut self) {
        unsafe {
            libc::close(self.master);
            libc::close(self.slave);
        }
    }
}

impl Drop for PTY {
    fn drop(&mut self) {
        unsafe {
            libc::kill(self.pid, libc::SIGTERM);
            let mut status: libc::c_int = 0;
            libc::waitpid(self.pid, &mut status, 0);
        }
    }
}
```

- [ ] **Step 3: Run tests**

Run: `cargo test pty --lib`
Expected: All tests pass

- [ ] **Step 4: Commit**

```bash
git add src/pty.rs
git commit -m "feat: implement PTY manager for process handling"
```

---

### Task 5: Tiling Layout Engine

**Files:**
- Create: `src/layout.rs`

- [ ] **Step 1: Write layout engine tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_creation() {
        let layout = Layout::new(80, 24);
        assert_eq!(layout.panes.len(), 1);
        assert_eq!(layout.focused, 0);
    }

    #[test]
    fn test_horizontal_split() {
        let mut layout = Layout::new(80, 24);
        layout.split_horizontal(0).unwrap();
        assert_eq!(layout.panes.len(), 2);
    }

    #[test]
    fn test_vertical_split() {
        let mut layout = Layout::new(80, 24);
        layout.split_vertical(0).unwrap();
        assert_eq!(layout.panes.len(), 2);
    }

    #[test]
    fn test_navigation() {
        let mut layout = Layout::new(80, 24);
        layout.split_horizontal(0).unwrap();
        layout.navigate(Direction::Down);
        assert_eq!(layout.focused, 1);
    }

    #[test]
    fn test_boundary_navigation() {
        let mut layout = Layout::new(80, 24);
        let original = layout.focused;
        layout.navigate(Direction::Up);
        assert_eq!(layout.focused, original);
    }
}
```

- [ ] **Step 2: Write layout engine implementation**

```rust
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Pane {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub buffer: buffer::Buffer,
    pub style: buffer::Style,
}

pub struct Layout {
    pub panes: Vec<Pane>,
    pub focused: usize,
    pub width: usize,
    pub height: usize,
    next_id: usize,
}

impl Layout {
    pub fn new(width: usize, height: usize) -> Self {
        let initial_pane = Pane {
            id: 0,
            x: 0,
            y: 0,
            width,
            height,
            buffer: buffer::Buffer::new(width, height),
            style: buffer::Style::default(),
        };
        
        Layout {
            panes: vec![initial_pane],
            focused: 0,
            width,
            height,
            next_id: 1,
        }
    }

    pub fn split_horizontal(&mut self, pane_id: usize) -> Result<(), String> {
        let pane_index = self.panes.iter().position(|p| p.id == pane_id)
            .ok_or("Pane not found")?;
        
        let pane = &mut self.panes[pane_index];
        if pane.height < 4 {
            return Err("Pane too small to split".to_string());
        }
        
        let new_height = pane.height / 2;
        pane.height = new_height;
        
        let new_pane = Pane {
            id: self.next_id,
            x: pane.x,
            y: pane.y + new_height,
            width: pane.width,
            height: pane.height,
            buffer: buffer::Buffer::new(pane.width, pane.height),
            style: buffer::Style::default(),
        };
        
        self.next_id += 1;
        self.panes.push(new_pane);
        self.focused = self.panes.len() - 1;
        
        Ok(())
    }

    pub fn split_vertical(&mut self, pane_id: usize) -> Result<(), String> {
        let pane_index = self.panes.iter().position(|p| p.id == pane_id)
            .ok_or("Pane not found")?;
        
        let pane = &mut self.panes[pane_index];
        if pane.width < 4 {
            return Err("Pane too small to split".to_string());
        }
        
        let new_width = pane.width / 2;
        pane.width = new_width;
        
        let new_pane = Pane {
            id: self.next_id,
            x: pane.x + new_width,
            y: pane.y,
            width: pane.width,
            height: pane.height,
            buffer: buffer::Buffer::new(pane.width, pane.height),
            style: buffer::Style::default(),
        };
        
        self.next_id += 1;
        self.panes.push(new_pane);
        self.focused = self.panes.len() - 1;
        
        Ok(())
    }

    pub fn navigate(&mut self, direction: Direction) {
        let current = &self.panes[self.focused];
        let target = self.find_adjacent_pane(current, direction);
        
        if let Some(target_id) = target {
            self.focused = self.panes.iter().position(|p| p.id == target_id).unwrap();
        }
    }

    fn find_adjacent_pane(&self, pane: &Pane, direction: Direction) -> Option<usize> {
        let mut best_candidate: Option<(usize, f64)> = None;
        
        for other in &self.panes {
            if other.id == pane.id {
                continue;
            }
            
            let distance = match direction {
                Direction::Up if other.y + other.height == pane.y => {
                    Some(Self::horizontal_overlap(pane, other) as f64)
                }
                Direction::Down if other.y == pane.y + pane.height => {
                    Some(Self::horizontal_overlap(pane, other) as f64)
                }
                Direction::Left if other.x + other.width == pane.x => {
                    Some(Self::vertical_overlap(pane, other) as f64)
                }
                Direction::Right if other.x == pane.x + pane.width => {
                    Some(Self::vertical_overlap(pane, other) as f64)
                }
                _ => None,
            };
            
            if let Some(overlap) = distance {
                if overlap > 0.0 {
                    if best_candidate.is_none() || overlap > best_candidate.unwrap().1 {
                        best_candidate = Some((other.id, overlap));
                    }
                }
            }
        }
        
        best_candidate.map(|(id, _)| id)
    }

    fn horizontal_overlap(a: &Pane, b: &Pane) -> usize {
        let start = a.x.max(b.x);
        let end = (a.x + a.width).min(b.x + b.width);
        end.saturating_sub(start)
    }

    fn vertical_overlap(a: &Pane, b: &Pane) -> usize {
        let start = a.y.max(b.y);
        let end = (a.y + a.height).min(b.y + b.height);
        end.saturating_sub(start)
    }

    pub fn get_focused_pane(&mut self) -> &mut Pane {
        &mut self.panes[self.focused]
    }

    pub fn remove_pane(&mut self, pane_id: usize) {
        self.panes.retain(|p| p.id != pane_id);
        if self.panes.is_empty() {
            let initial_pane = Pane {
                id: self.next_id,
                x: 0,
                y: 0,
                width: self.width,
                height: self.height,
                buffer: buffer::Buffer::new(self.width, self.height),
                style: buffer::Style::default(),
            };
            self.next_id += 1;
            self.panes.push(initial_pane);
        }
        if self.focused >= self.panes.len() {
            self.focused = self.panes.len() - 1;
        }
    }
}
```

- [ ] **Step 3: Run tests**

Run: `cargo test layout --lib`
Expected: All tests pass

- [ ] **Step 4: Commit**

```bash
git add src/layout.rs
git commit -m "feat: implement tiling layout engine"
```

---

### Task 6: Terminal Renderer

**Files:**
- Create: `src/renderer.rs`

- [ ] **Step 1: Write renderer tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_creation() {
        let renderer = Renderer::new();
        assert!(!renderer.stdout.is_null());
    }

    #[test]
    fn test_render_cell() {
        let mut renderer = Renderer::new();
        let cell = buffer::Cell {
            ch: 'A',
            style: buffer::Style::default(),
        };
        renderer.render_cell(0, 0, &cell);
    }

    #[test]
    fn test_clear_screen() {
        let mut renderer = Renderer::new();
        renderer.clear_screen();
    }
}
```

- [ ] **Step 2: Write renderer implementation**

```rust
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;

pub struct Renderer {
    pub stdout: termion::raw::RawTerminal<AlternateScreen<io::Stdout>>,
}

impl Renderer {
    pub fn new() -> Result<Self, String> {
        let stdout = AlternateScreen::to_alternate(io::stdout())
            .map_err(|e| format!("Failed to create alternate screen: {}", e))?;
        let stdout = stdout.into_raw_mode()
            .map_err(|e| format!("Failed to set raw mode: {}", e))?;
        
        Ok(Renderer { stdout })
    }

    pub fn clear_screen(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).ok();
        self.stdout.flush().ok();
    }

    pub fn move_cursor(&mut self, x: u16, y: u16) {
        write!(self.stdout, "{}", termion::cursor::Goto(x + 1, y + 1)).ok();
    }

    pub fn hide_cursor(&mut self) {
        write!(self.stdout, "{}", termion::cursor::Hide).ok();
    }

    pub fn show_cursor(&mut self) {
        write!(self.stdout, "{}", termion::cursor::Show).ok();
    }

    pub fn render_cell(&mut self, x: usize, y: usize, cell: &buffer::Cell) {
        self.move_cursor(x as u16, y as u16);
        
        let fg = self.color_to_termion(cell.style.fg_color);
        let bg = self.color_to_termion(cell.style.bg_color);
        
        let style = if cell.style.bold {
            termion::style::Bold
        } else {
            termion::style::Reset
        };
        
        write!(self.stdout, "{}{}{}{}", fg, bg, style, cell.ch).ok();
    }

    pub fn render_pane(&mut self, pane: &layout::Pane, is_focused: bool) {
        for y in 0..pane.height {
            for x in 0..pane.width {
                let cell = pane.buffer.get(x, y).unwrap();
                self.move_cursor((pane.x + x) as u16, (pane.y + y) as u16);
                
                let fg = self.color_to_termion(cell.style.fg_color);
                let bg = if is_focused {
                    termion::color::Bg(termion::color::Reset)
                } else {
                    self.color_to_termion_bg(cell.style.bg_color)
                };
                
                write!(self.stdout, "{}{}{}", fg, bg, cell.ch).ok();
            }
        }
        
        if !is_focused {
            self.draw_border(pane);
        }
    }

    fn draw_border(&mut self, pane: &layout::Pane) {
        let border_char = '│';
        for y in pane.y..pane.y + pane.height {
            self.move_cursor(pane.x as u16, y as u16);
            write!(self.stdout, "{}", termion::color::Fg(termion::color::Black)).ok();
            write!(self.stdout, "{}", termion::color::Bg(termion::color::White)).ok();
            write!(self.stdout, "{}", border_char).ok();
        }
    }

    pub fn flush(&mut self) {
        self.stdout.flush().ok();
    }

    fn color_to_termion(&self, color: buffer::Color) -> termion::color::Fg<termion::color::AnsiValue> {
        use termion::color;
        use buffer::Color;
        
        color::Fg(match color {
            Color::Default => color::AnsiValue::Reset,
            Color::Black => color::AnsiValue::Black,
            Color::Red => color::AnsiValue::Red,
            Color::Green => color::AnsiValue::Green,
            Color::Yellow => color::AnsiValue::Yellow,
            Color::Blue => color::AnsiValue::Blue,
            Color::Magenta => color::AnsiValue::Magenta,
            Color::Cyan => color::AnsiValue::Cyan,
            Color::White => color::AnsiValue::White,
        })
    }

    fn color_to_termion_bg(&self, color: buffer::Color) -> termion::color::Bg<termion::color::AnsiValue> {
        use termion::color;
        use buffer::Color;
        
        color::Bg(match color {
            Color::Default => color::AnsiValue::Reset,
            Color::Black => color::AnsiValue::Black,
            Color::Red => color::AnsiValue::Red,
            Color::Green => color::AnsiValue::Green,
            Color::Yellow => color::AnsiValue::Yellow,
            Color::Blue => color::AnsiValue::Blue,
            Color::Magenta => color::AnsiValue::Magenta,
            Color::Cyan => color::AnsiValue::Cyan,
            Color::White => color::AnsiValue::White,
        })
    }
}
```

- [ ] **Step 3: Run tests**

Run: `cargo test renderer --lib`
Expected: All tests pass

- [ ] **Step 4: Commit**

```bash
git add src/renderer.rs
git commit -m "feat: implement terminal renderer with termion"
```

---

### Task 7: Input Handler

**Files:**
- Create: `src/input.rs`

- [ ] **Step 1: Write input handler tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_key() {
        let result = handle_input(b'a').unwrap();
        assert_eq!(result, InputAction::SendToPTY(vec![b'a']));
    }

    #[test]
    fn test_split_horizontal() {
        let result = handle_input(&[18, 72]); // Ctrl+Shift+H
        assert_eq!(result, InputAction::SplitHorizontal);
    }

    #[test]
    fn test_split_vertical() {
        let result = handle_input(&[18, 86]); // Ctrl+Shift+V
        assert_eq!(result, InputAction::SplitVertical);
    }

    #[test]
    fn test_navigate_up() {
        let result = handle_input(&[18, 65]); // Ctrl+Shift+Up
        assert_eq!(result, InputAction::Navigate(layout::Direction::Up));
    }
}
```

- [ ] **Step 2: Write input handler implementation**

```rust
pub enum InputAction {
    SendToPTY(Vec<u8>),
    SplitHorizontal,
    SplitVertical,
    Navigate(layout::Direction),
}

pub fn handle_input(bytes: &[u8]) -> Option<InputAction> {
    if bytes.len() >= 2 && bytes[0] == 18 {
        match bytes[1] {
            72 => return Some(InputAction::SplitHorizontal),
            86 => return Some(InputAction::SplitVertical),
            65 => return Some(InputAction::Navigate(layout::Direction::Up)),
            66 => return Some(InputAction::Navigate(layout::Direction::Down)),
            68 => return Some(InputAction::Navigate(layout::Direction::Left)),
            67 => return Some(InputAction::Navigate(layout::Direction::Right)),
            _ => return Some(InputAction::SendToPTY(bytes.to_vec())),
        }
    }
    
    Some(InputAction::SendToPTY(bytes.to_vec()))
}

pub fn read_key() -> Option<Vec<u8>> {
    use termion::input::TermRead;
    use std::io;
    
    let stdin = io::stdin();
    if let Ok(bytes) = stdin.read_one() {
        Some(vec![bytes])
    } else {
        None
    }
}
```

- [ ] **Step 3: Run tests**

Run: `cargo test input --lib`
Expected: All tests pass

- [ ] **Step 4: Commit**

```bash
git add src/input.rs
git commit -m "feat: implement input handler with keyboard shortcuts"
```

---

### Task 8: Main Application

**Files:**
- Create: `src/main.rs`

- [ ] **Step 1: Write main.rs skeleton**

```rust
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use termion::input::TermRead;
use std::io::{self, Write};

mod buffer;
mod pty;
mod layout;
mod renderer;
mod ansi;
mod input;

struct PaneData {
    pty: pty::PTY,
    cursor_x: usize,
    cursor_y: usize,
}

fn main() -> Result<(), String> {
    let mut renderer = renderer::Renderer::new()?;
    renderer.hide_cursor();
    renderer.clear_screen();
    
    let mut layout = layout::Layout::new(80, 24);
    let mut panes: HashMap<usize, PaneData> = HashMap::new();
    
    let (pty_tx, pty_rx) = mpsc::channel();
    
    let focused_pane = *layout.get_focused_pane();
    let initial_pty = pty::PTY::new(std::env::var("SHELL").unwrap_or_else(|_| "bash".to_string()).as_str(), &[])?;
    panes.insert(focused_pane.id, PaneData {
        pty: initial_pty,
        cursor_x: 0,
        cursor_y: 0,
    });
    
    let stdin = io::stdin();
    for c in stdin.keys() {
        match c {
            Ok(termion::event::Key::Ctrl('c')) => {
                for (_, pane_data) in &mut panes {
                    pane_data.pty.close();
                }
                break;
            }
            Ok(key) => {
                let bytes = key_to_bytes(key);
                if let Some(action) = input::handle_input(&bytes) {
                    match action {
                        input::InputAction::SendToPTY(data) => {
                            let focused = *layout.get_focused_pane();
                            if let Some(pane_data) = panes.get_mut(&focused.id) {
                                pane_data.pty.write(&data).ok();
                            }
                        }
                        input::InputAction::SplitHorizontal => {
                            let focused = *layout.get_focused_pane();
                            if let Ok(_) = layout.split_horizontal(focused.id) {
                                spawn_new_pane(&mut panes, &pty_tx);
                            }
                        }
                        input::InputAction::SplitVertical => {
                            let focused = *layout.get_focused_pane();
                            if let Ok(_) = layout.split_vertical(focused.id) {
                                spawn_new_pane(&mut panes, &pty_tx);
                            }
                        }
                        input::InputAction::Navigate(dir) => {
                            layout.navigate(dir);
                        }
                    }
                }
            }
            Err(_) => break,
        }
    }
    
    renderer.show_cursor();
    renderer.clear_screen();
    
    Ok(())
}

fn spawn_new_pane(panes: &mut HashMap<usize, PaneData>, tx: &mpsc::Sender<pty::PTY>) {
    let focused_id = panes.keys().next().copied().unwrap_or(0);
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "bash".to_string());
    let new_pty = pty::PTY::new(shell.as_str(), &[]).unwrap();
    panes.insert(focused_id + 1, PaneData {
        pty: new_pty,
        cursor_x: 0,
        cursor_y: 0,
    });
}

fn key_to_bytes(key: termion::event::Key) -> Vec<u8> {
    use termion::event::Key;
    match key {
        Key::Char(c) => vec![c as u8],
        Key::Ctrl(c) => vec![c as u8 - 96],
        Key::Alt(c) => vec![27, c as u8],
        Key::Up => vec![27, 91, 65],
        Key::Down => vec![27, 91, 66],
        Key::Left => vec![27, 91, 68],
        Key::Right => vec![27, 91, 67],
        Key::Backspace => vec![127],
        Key::Enter => vec![13],
        _ => vec![],
    }
}
```

- [ ] **Step 2: Add PTY output reading loop**

```rust
// Add this after spawn_new_pane function in main.rs

fn read_pty_output(panes: &mut HashMap<usize, PaneData>, layout: &mut layout::Layout) {
    let panes_to_remove = Vec::new();
    
    for (pane_id, pane_data) in panes.iter_mut() {
        if let Ok(output) = pane_data.pty.read() {
            if output.is_empty() && !pane_data.pty.is_alive() {
                panes_to_remove.push(*pane_id);
                continue;
            }
            
            if let Some(pane) = layout.panes.iter_mut().find(|p| p.id == *pane_id) {
                let actions = ansi::parse(&String::from_utf8_lossy(&output));
                process_pty_actions(pane, pane_data, &actions);
            }
        }
    }
    
    for pane_id in panes_to_remove {
        layout.remove_pane(pane_id);
        panes.remove(&pane_id);
    }
}

fn process_pty_actions(pane: &mut layout::Pane, pane_data: &mut PaneData, actions: &[ansi::Action]) {
    for action in actions {
        match action {
            ansi::Action::Write(ch) => {
                pane.buffer.write(pane_data.cursor_x, pane_data.cursor_y, *ch, pane_data.cursor_y);
                pane_data.cursor_x += 1;
                if pane_data.cursor_x >= pane.width {
                    pane_data.cursor_x = 0;
                    pane_data.cursor_y += 1;
                }
            }
            ansi::Action::MoveCursor(x, y) => {
                pane_data.cursor_x = *x;
                pane_data.cursor_y = *y;
            }
            ansi::Action::SetFgColor(color) => {
                pane.style.fg_color = buffer_color_to_color(*color);
            }
            ansi::Action::SetBgColor(color) => {
                pane.style.bg_color = buffer_color_to_color(*color);
            }
            ansi::Action::Reset => {
                pane_data.cursor_x = 0;
                pane_data.cursor_y = 0;
            }
            ansi::Action::ClearLine => {
                for x in 0..pane.width {
                    pane.buffer.write(x, pane_data.cursor_y, ' ', buffer::Style::default());
                }
                pane_data.cursor_x = 0;
            }
            ansi::Action::ClearScreen => {
                pane.buffer.clear();
                pane_data.cursor_x = 0;
                pane_data.cursor_y = 0;
            }
        }
    }
}

fn buffer_color_to_color(color: ansi::Color) -> buffer::Color {
    use buffer::Color;
    use ansi::Color;
    match color {
        Color::Default => Color::Default,
        Color::Black => Color::Black,
        Color::Red => Color::Red,
        Color::Green => Color::Green,
        Color::Yellow => Color::Yellow,
        Color::Blue => Color::Blue,
        Color::Magenta => Color::Magenta,
        Color::Cyan => Color::Cyan,
        Color::White => Color::White,
    }
}
```

- [ ] **Step 3: Update main loop with PTY reading and rendering**

```rust
// Replace the main loop in main.rs with this version

    loop {
        read_pty_output(&mut panes, &mut layout);
        
        for pane in &layout.panes {
            let is_focused = pane.id == layout.panes[layout.focused].id;
            renderer.render_pane(pane, is_focused);
        }
        
        renderer.flush();
        
        use termion::async_stdin;
        let mut stdin = async_stdin().bytes();
        
        if let Some(Ok(byte)) = stdin.next() {
            let bytes = vec![byte];
            if let Some(action) = input::handle_input(&bytes) {
                match action {
                    input::InputAction::SendToPTY(data) => {
                        let focused = *layout.get_focused_pane();
                        if let Some(pane_data) = panes.get_mut(&focused.id) {
                            pane_data.pty.write(&data).ok();
                        }
                    }
                    input::InputAction::SplitHorizontal => {
                        let focused = *layout.get_focused_pane();
                        if layout.split_horizontal(focused.id).is_ok() {
                            spawn_new_pane(&mut panes, &pty_tx);
                        }
                    }
                    input::InputAction::SplitVertical => {
                        let focused = *layout.get_focused_pane();
                        if layout.split_vertical(focused.id).is_ok() {
                            spawn_new_pane(&mut panes, &pty_tx);
                        }
                    }
                    input::InputAction::Navigate(dir) => {
                        layout.navigate(dir);
                    }
                }
            }
        }
    }
```

- [ ] **Step 4: Build and test**

Run: `cargo build --release`
Expected: Binary compiles without errors

- [ ] **Step 5: Commit**

```bash
git add src/main.rs
git commit -m "feat: implement main application loop"
```

---

### Task 9: Integration Testing

**Files:**
- Create: `tests/integration_test.rs`

- [ ] **Step 1: Write integration test**

```rust
#[test]
fn test_basic_functionality() {
    // This would be a manual test that:
    // 1. Spawns the terminal emulator
    // 2. Creates a horizontal split
    // 3. Creates a vertical split
    // 4. Navigates between panes
    // 5. Exits from one pane
    // 6. Verifies the pane is removed
    
    // For now, this is a placeholder that documents
    // the manual testing procedure
    println!("Manual testing required for full integration");
}
```

- [ ] **Step 2: Commit**

```bash
git add tests/integration_test.rs
git commit -m "test: add integration test skeleton"
```

---

### Task 10: Documentation

**Files:**
- Create: `README.md`

- [ ] **Step 1: Write README**

```markdown
# Terminal Tiler

A CPU-only terminal emulator with tiling support for old systems.

## Features

- Split terminals horizontally and vertically
- Navigate between panes using arrow keys
- Automatic cleanup when panes exit
- No GPU dependency
- Lightweight and fast

## Keyboard Shortcuts

- `Ctrl+Shift+H` - Split horizontally
- `Ctrl+Shift+V` - Split vertically  
- `Ctrl+Shift+Arrow Keys` - Navigate between panes
- `exit` - Close focused pane
- `Ctrl+C` - Exit emulator

## Installation

```bash
cargo build --release
./target/release/term-tiler
```

## Requirements

- Rust 1.70 or later
- Linux/Unix system (PTY support required)
- Terminal with ANSI support

## Usage

Run the emulator and use your shell as normal. Split your workspace
to run multiple commands simultaneously.
```

- [ ] **Step 2: Commit**

```bash
git add README.md
git commit -m "docs: add README with usage instructions"
```

---

## Final Build and Test

- [ ] **Step 1: Final build**

Run: `cargo build --release`
Expected: Clean release build

- [ ] **Step 2: Run all tests**

Run: `cargo test --all`
Expected: All tests pass

- [ ] **Step 3: Commit**

```bash
git add .
git commit -m "feat: complete terminal emulator implementation"
```
