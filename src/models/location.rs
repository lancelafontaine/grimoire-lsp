use lsp_types::Position;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub in_range: bool,
    pub line_position: u32,
    pub start_char_position: i64,
    pub end_char_position: i64,
}

const DEFAULT_LINE_POSITION: u32 = 0;
const DEFAULT_CHAR_POSITION: i64 = -1;

impl Location {
    pub fn next(&mut self, c: char) {
        if c == '\n' {
            self.in_range = false;
            self.line_position += 1;
            self.start_char_position = DEFAULT_CHAR_POSITION;
            self.end_char_position = DEFAULT_CHAR_POSITION;
        } else {
            if !self.in_range {
                self.start_char_position += 1;
            }
            self.end_char_position += 1;
        }
    }

    pub fn in_range(&mut self) {
        self.in_range = true;
    }

    pub fn resume(&mut self) {
        self.in_range = false;
        self.start_char_position = self.end_char_position;
    }

    pub fn contains(&self, position: &Position) -> bool {
        position.line == self.line_position
            && self.start_char_position >= 0
            && position.character >= self.start_char_position as u32
            && self.end_char_position >= 0
            && position.character <= self.end_char_position as u32
    }
}

impl Default for Location {
    fn default() -> Self {
        Self {
            in_range: false,
            line_position: DEFAULT_LINE_POSITION,
            start_char_position: DEFAULT_CHAR_POSITION,
            end_char_position: DEFAULT_CHAR_POSITION,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn location_default() {
        let location = Location::default();
        assert_eq!(location.line_position, DEFAULT_LINE_POSITION);
        assert_eq!(location.start_char_position, DEFAULT_CHAR_POSITION);
        assert_eq!(location.end_char_position, DEFAULT_CHAR_POSITION);
    }

    #[test]
    fn location_next_newline() {
        let mut location = Location::default();
        location.next('\n');
        assert_eq!(location.line_position, 1);
        assert_eq!(location.start_char_position, -1);
        assert_eq!(location.end_char_position, -1);
    }

    #[test]
    fn location_next_not_newline() {
        let mut location = Location::default();
        location.next('a');
        assert_eq!(location.line_position, 0);
        assert_eq!(location.start_char_position, 0);
        assert_eq!(location.end_char_position, 0);
    }

    #[test]
    fn location_in_range() {
        let mut location = Location::default();
        location.next('a');
        location.next('b');
        assert_eq!(location.start_char_position, 1);
        assert_eq!(location.end_char_position, 1);

        location.in_range();
        assert_eq!(location.start_char_position, 1);
        assert_eq!(location.end_char_position, 1);

        location.next('c');
        assert_eq!(location.start_char_position, 1);
        assert_eq!(location.end_char_position, 2);

        location.next('\n');
        assert_eq!(location.start_char_position, -1);
        assert_eq!(location.end_char_position, -1);
    }

    #[test]
    fn location_resume() {
        let mut location = Location::default();
        location.next('a');
        location.next('b');
        assert_eq!(location.start_char_position, 1);
        assert_eq!(location.end_char_position, 1);

        location.in_range();
        location.next('c');
        assert_eq!(location.start_char_position, 1);
        assert_eq!(location.end_char_position, 2);

        location.resume();
        assert_eq!(location.start_char_position, 2);
        assert_eq!(location.end_char_position, 2);

        location.next('d');
        assert_eq!(location.start_char_position, 3);
        assert_eq!(location.end_char_position, 3);
    }

    #[test]
    fn location_contains_true() {
        let mut location = Location::default();
        location.next('a');
        location.in_range();
        location.next('b');
        location.next('c');
        assert_eq!(location.start_char_position, 0);
        assert_eq!(location.end_char_position, 2);

        let assertion1 = location.contains(&Position {
            line: 0,
            character: 1,
        });
        assert!(assertion1);

        let assertion2 = location.contains(&Position {
            line: 1,
            character: 1,
        });
        assert!(!assertion2);

        let assertion3 = location.contains(&Position {
            line: 0,
            character: 10,
        });
        assert!(!assertion3);
    }
}
