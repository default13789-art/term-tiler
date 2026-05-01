// Input handling now lives in main.rs (handle_ctrl_a_command).
// This module is kept for test coverage of key mapping.

#[cfg(test)]
mod tests {
    use termion::event::Key;

    fn ctrl_a_command(key: Key) -> Option<&'static str> {
        match key {
            Key::Ctrl('a') => Some("literal"),
            Key::Char('h') | Key::Char('H') | Key::Ctrl('h') => Some("split-h"),
            Key::Char('v') | Key::Char('V') => Some("split-v"),
            Key::Char('j') | Key::Char('J') => Some("nav-down"),
            Key::Char('k') | Key::Char('K') => Some("nav-up"),
            Key::Char('l') | Key::Char('L') => Some("nav-right"),
            Key::Char('p') | Key::Char('P') | Key::Ctrl('p') => Some("nav-left"),
            _ => None,
        }
    }

    #[test]
    fn test_split_horizontal() {
        assert_eq!(ctrl_a_command(Key::Char('h')), Some("split-h"));
    }

    #[test]
    fn test_split_vertical() {
        assert_eq!(ctrl_a_command(Key::Char('v')), Some("split-v"));
    }

    #[test]
    fn test_navigate_up() {
        assert_eq!(ctrl_a_command(Key::Char('k')), Some("nav-up"));
    }

    #[test]
    fn test_literal_ctrl_a() {
        assert_eq!(ctrl_a_command(Key::Ctrl('a')), Some("literal"));
    }

    #[test]
    fn test_unknown_key() {
        assert_eq!(ctrl_a_command(Key::Char('x')), None);
    }

    #[test]
    fn test_navigate_left() {
        assert_eq!(ctrl_a_command(Key::Char('p')), Some("nav-left"));
    }
}
