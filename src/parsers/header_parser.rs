use crate::models::Location;
use crate::parsers::Parser;

pub struct HeaderParser {
    prefix: Option<Option<()>>,
    payload: Option<HeaderParserPayload>,
    location: Location,
    done: bool,
}

impl HeaderParser {
    pub fn new() -> Self {
        Self {
            prefix: None,
            payload: None,
            location: Location::default(),
            done: false,
        }
    }

    pub fn call(self) -> Option<HeaderParserPayload> {
        if self.done {
            return self.payload.map(|p| p.trim());
        }
        None
    }
}

impl Default for HeaderParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for HeaderParser {
    fn next(&mut self, c: char) {
        if self.done {
            return;
        }
        self.location.next(c);
        match self.prefix {
            None => {
                if c == '#' {
                    self.location.in_range();
                    self.prefix = Some(None);
                }
            }
            Some(prefix) => match prefix {
                None => match c {
                    ' ' => self.prefix = Some(Some(())),
                    _ => {
                        self.location.resume();
                        self.prefix = None
                    }
                },
                Some(_) => match &mut self.payload {
                    None => match c {
                        ' ' => {}
                        '\n' => self.prefix = None,
                        c => {
                            self.payload = Some(HeaderParserPayload::from(c, self.location.clone()))
                        }
                    },
                    Some(payload) => match c {
                        '\n' => {
                            self.done = true;
                        }
                        c => payload.push(c),
                    },
                },
            },
        }
    }
}

#[derive(Debug)]
pub struct HeaderParserPayload {
    pub header: String,
    pub location: Location,
}

impl HeaderParserPayload {
    fn from(c: char, location: Location) -> Self {
        Self {
            header: String::from(c),
            location,
        }
    }

    fn push(&mut self, c: char) {
        self.header.push(c);
        self.location.next(c);
    }

    fn trim(self) -> Self {
        Self {
            header: self.header.trim_end().to_string(),
            location: self.location,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_parser_new() {
        assert!(!HeaderParser::new().done);
    }

    #[test]
    fn test_header_parser_default() {
        assert!(!HeaderParser::default().done);
    }

    #[test]
    fn test_header_parser_location_char_position_no_newline() {
        let mut parser = HeaderParser::new();
        parser.next('a');
        assert_eq!(parser.location.line_position, 0);
        assert_eq!(parser.location.start_char_position, 0);
        assert_eq!(parser.location.end_char_position, 0);
    }

    #[test]
    fn test_header_parser_location_char_position_newline() {
        let mut parser = HeaderParser::new();
        parser.next('\n');
        assert_eq!(parser.location.line_position, 1);
        assert_eq!(parser.location.start_char_position, -1);
        assert_eq!(parser.location.end_char_position, -1);
        parser.next('a');
        assert_eq!(parser.location.line_position, 1);
        assert_eq!(parser.location.start_char_position, 0);
        assert_eq!(parser.location.end_char_position, 0);
    }

    #[test]
    fn test_header_parser_location_in_range() {
        let mut parser = HeaderParser::new();
        parser.next('#');
        parser.next(' ');
        parser.next('A');
        assert!(parser.location.in_range);
    }
    #[test]
    fn test_header_parser_location_out_of_range() {
        let mut parser = HeaderParser::new();
        parser.next('#');
        parser.next(' ');
        parser.next('A');
        parser.next('\n');
        assert!(!parser.location.in_range);
    }

    #[test]
    fn test_header_parser_next_initial_state() {
        let mut parser = HeaderParser::new();
        parser.next('a');
        assert!(!parser.done);
        assert!(parser.prefix.is_none());
        assert!(parser.payload.is_none());
    }

    #[test]
    fn test_header_parser_next_prefix_first_state() {
        let mut parser = HeaderParser::new();
        parser.next('#');
        assert!(!parser.done);
        assert!(parser.prefix.is_some());
        assert!(parser.prefix.unwrap().is_none());
        assert!(parser.payload.is_none());

        parser.next('a');
        assert!(!parser.done);
        assert!(parser.prefix.is_none());
        assert!(parser.payload.is_none());
    }

    #[test]
    fn test_header_parser_next_prefix_final_state() {
        let mut parser = HeaderParser::new();
        parser.next('#');
        parser.next(' ');
        assert!(!parser.done);
        assert!(parser.prefix.is_some());
        assert!(parser.prefix.unwrap().is_some());
        assert!(parser.payload.is_none());
    }

    #[test]
    fn test_header_parser_next_header_capture_state() {
        let mut parser = HeaderParser::new();
        parser.next('#');
        parser.next(' ');
        parser.next('a');
        parser.next('b');
        parser.next('c');
        assert!(!parser.done);
        assert!(parser.prefix.is_some());
        assert!(parser.prefix.unwrap().is_some());
        assert!(parser.payload.is_some());
        assert_eq!(parser.payload.unwrap().header, String::from("abc"));
    }

    #[test]
    fn test_header_parser_next_header_final_state() {
        let mut parser = HeaderParser::new();
        parser.next('#');
        parser.next(' ');
        parser.next('a');
        parser.next('b');
        parser.next('c');
        parser.next('\n');
        assert!(parser.done);
        assert!(parser.prefix.is_some());
        assert!(parser.prefix.unwrap().is_some());
        assert!(parser.payload.is_some());
        assert_eq!(parser.payload.as_ref().unwrap().header, String::from("abc"));

        parser.next('#');
        parser.next(' ');
        parser.next('d');
        parser.next('e');
        parser.next('f');
        parser.next('\n');
        assert!(parser.done);
        assert!(parser.prefix.is_some());
        assert!(parser.prefix.unwrap().is_some());
        assert!(parser.payload.is_some());
        assert_eq!(parser.payload.as_ref().unwrap().header, String::from("abc"));
    }

    #[test]
    fn test_header_parser_call_in_non_final_state() {
        let mut parser = HeaderParser::new();
        parser.next('#');
        parser.next(' ');
        assert!(parser.call().is_none());
    }

    #[test]
    fn test_header_parser_call_in_final_state() {
        let mut parser = HeaderParser::new();
        parser.next('#');
        parser.next(' ');
        parser.next('a');
        parser.next('b');
        parser.next('c');
        parser.next('\n');
        let payload = parser.call();
        assert!(payload.is_some());
        assert_eq!(payload.unwrap().header, String::from("abc"))
    }
}
