use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub line_position: u64,
    pub char_position: u64,
}

const DEFAULT_LINE_POSITION: u64 = 1;
const DEFAULT_CHAR_POSITION: u64 = 1;

impl Location {
    pub fn next(&mut self, c: char) {
        if c == '\n' {
            self.line_position += 1;
            self.char_position = DEFAULT_CHAR_POSITION;
        } else {
            self.char_position += 1;
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Self {
            line_position: DEFAULT_CHAR_POSITION,
            char_position: DEFAULT_LINE_POSITION,
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
        assert_eq!(location.char_position, DEFAULT_CHAR_POSITION);
    }

    #[test]
    fn location_next_newline() {
        let mut location = Location::default();
        assert_eq!(location.line_position, 1);
        assert_eq!(location.char_position, 1);

        location.next('\n');
        assert_eq!(location.line_position, 2);
        assert_eq!(location.char_position, 1);
    }

    #[test]
    fn location_next_not_newline() {
        let mut location = Location::default();
        assert_eq!(location.line_position, 1);
        assert_eq!(location.char_position, 1);

        location.next('a');
        assert_eq!(location.line_position, 1);
        assert_eq!(location.char_position, 2);
    }
}
