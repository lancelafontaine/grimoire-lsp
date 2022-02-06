use crate::models::Location;
use crate::parsers::Parser;

pub struct ReferenceParser {
    prefix: Option<Option<()>>,
    reference: Option<ReferenceParserPayload>,
    references: Vec<ReferenceParserPayload>,
    suffix: Option<Option<()>>,
    location: Location,
    done: bool,
}

impl ReferenceParser {
    pub fn new() -> Self {
        Self {
            prefix: None,
            reference: None,
            references: vec![],
            suffix: None,
            location: Location::default(),
            done: false,
        }
    }

    pub fn call(self) -> Vec<ReferenceParserPayload> {
        self.references
    }
}

impl Default for ReferenceParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for ReferenceParser {
    fn next(&mut self, c: char) {
        self.location.next(c);
        match self.prefix {
            None => match c {
                '[' => self.prefix = Some(None),
                _ => return,
            },
            Some(prefix) => match prefix {
                None => match c {
                    '[' => self.prefix = Some(Some(())),
                    _ => self.prefix = None,
                },
                Some(_) => match &mut self.reference {
                    None => match c {
                        ' ' => return,
                        '\n' => self.prefix = None,
                        c => {
                            self.reference =
                                Some(ReferenceParserPayload::from(c, self.location.clone()))
                        }
                    },
                    Some(payload) => match c {
                        '\n' => {
                            self.prefix = None;
                            self.suffix = None;
                            self.reference = None;
                        }
                        ']' => match self.suffix {
                            None => self.suffix = Some(None),
                            Some(_) => self.done = true,
                        },
                        c => payload.push(c),
                    },
                },
            },
        }
        if self.done {
            self.prefix = None;
            self.suffix = None;
            self.references.push(self.reference.take().unwrap());
            self.done = false
        }
    }
}

#[derive(Debug)]
pub struct ReferenceParserPayload {
    pub header: String,
    pub location: Location,
}

impl ReferenceParserPayload {
    fn from(c: char, location: Location) -> Self {
        Self {
            header: String::from(c),
            location,
        }
    }

    fn push(&mut self, c: char) {
        self.header.push(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_parser_new() {
        assert!(!ReferenceParser::new().done);
    }

    #[test]
    fn test_reference_parser_default() {
        assert!(!ReferenceParser::default().done);
    }

    #[test]
    fn test_reference_parser_location_char_position_no_newline() {
        let mut parser = ReferenceParser::new();
        assert_eq!(parser.location.line_position, 1);
        assert_eq!(parser.location.char_position, 1);
        parser.next('a');
        assert_eq!(parser.location.line_position, 1);
        assert_eq!(parser.location.char_position, 2);
    }

    #[test]
    fn test_reference_parser_location_char_position_newline() {
        let mut parser = ReferenceParser::new();
        assert_eq!(parser.location.line_position, 1);
        assert_eq!(parser.location.char_position, 1);
        parser.next('\n');
        assert_eq!(parser.location.line_position, 2);
        assert_eq!(parser.location.char_position, 1);
    }

    #[test]
    fn test_reference_parser_next_initial_state() {
        let mut parser = ReferenceParser::new();
        parser.next('\n');
        assert!(!parser.done);
        assert!(parser.prefix.is_none());
        assert!(parser.suffix.is_none());
        assert!(parser.reference.is_none());
        assert!(parser.references.is_empty());
    }

    #[test]
    fn test_reference_parser_next_prefix_first_state() {
        let mut parser = ReferenceParser::new();
        parser.next('[');
        assert!(!parser.done);
        assert!(parser.prefix.is_some());
        assert!(parser.prefix.unwrap().is_none());
        assert!(parser.suffix.is_none());
        assert!(parser.reference.is_none());
        assert!(parser.references.is_empty());

        parser.next('\n');
        assert!(!parser.done);
        assert!(parser.prefix.is_none());
        assert!(parser.suffix.is_none());
        assert!(parser.reference.is_none());
        assert!(parser.references.is_empty());
    }

    #[test]
    fn test_reference_parser_next_prefix_final_state() {
        let mut parser = ReferenceParser::new();
        parser.next('[');
        parser.next('[');
        assert!(!parser.done);
        assert!(parser.prefix.is_some());
        assert!(parser.prefix.unwrap().is_some());
        assert!(parser.suffix.is_none());
        assert!(parser.reference.is_none());
        assert!(parser.references.is_empty());

        parser.next('\n');
        assert!(!parser.done);
        assert!(parser.prefix.is_none());
        assert!(parser.suffix.is_none());
        assert!(parser.reference.is_none());
        assert!(parser.references.is_empty());
    }

    #[test]
    fn test_reference_parser_next_reference_capture_state() {
        let mut parser = ReferenceParser::new();
        parser.next('[');
        parser.next('[');
        parser.next('a');
        parser.next('b');
        parser.next('c');
        assert!(!parser.done);
        assert!(parser.prefix.is_some());
        assert!(parser.prefix.unwrap().is_some());
        assert!(parser.suffix.is_none());
        assert!(parser.reference.is_some());
        assert_eq!(
            parser.reference.as_ref().unwrap().header,
            String::from("abc")
        );
        assert!(parser.references.is_empty());

        parser.next('\n');
        assert!(!parser.done);
        assert!(parser.prefix.is_none());
        assert!(parser.suffix.is_none());
        assert!(parser.reference.is_none());
        assert!(parser.references.is_empty());
    }

    #[test]
    fn test_reference_parser_next_suffix_first_state() {
        let mut parser = ReferenceParser::new();
        parser.next('[');
        parser.next('[');
        parser.next('a');
        parser.next('b');
        parser.next('c');
        parser.next(']');
        assert!(!parser.done);
        assert!(parser.prefix.is_some());
        assert!(parser.prefix.unwrap().is_some());
        assert!(parser.reference.is_some());
        assert_eq!(
            parser.reference.as_ref().unwrap().header,
            String::from("abc")
        );
        assert!(parser.references.is_empty());
        assert!(parser.suffix.is_some());
        assert!(parser.suffix.unwrap().is_none());

        parser.next('\n');
        assert!(!parser.done);
        assert!(parser.prefix.is_none());
        assert!(parser.suffix.is_none());
        assert!(parser.reference.is_none());
        assert!(parser.references.is_empty());
    }

    #[test]
    fn test_reference_parser_next_suffix_final_state() {
        let mut parser = ReferenceParser::new();
        parser.next('[');
        parser.next('[');
        parser.next('a');
        parser.next('b');
        parser.next('c');
        parser.next(']');
        parser.next(']');
        assert!(!parser.done);
        assert!(parser.prefix.is_none());
        assert!(parser.reference.is_none());
        assert_eq!(parser.references.len(), 1);
        assert_eq!(
            parser.references.first().unwrap().header,
            String::from("abc")
        );
        assert!(parser.suffix.is_none());

        parser.next('\n');
        assert!(!parser.done);
        assert!(parser.prefix.is_none());
        assert!(parser.reference.is_none());
        assert_eq!(parser.references.len(), 1);
        assert_eq!(
            parser.references.first().unwrap().header,
            String::from("abc")
        );
        assert!(parser.suffix.is_none());
    }

    #[test]
    fn test_reference_parser_call_in_non_final_state() {
        let mut parser = ReferenceParser::new();
        parser.next('[');
        parser.next('[');
        parser.next('a');
        parser.next('b');
        parser.next('c');
        parser.next(']');
        assert!(parser.call().is_empty());
    }

    #[test]
    fn test_reference_parser_call_in_final_state() {
        let mut parser = ReferenceParser::new();
        parser.next('[');
        parser.next('[');
        parser.next('a');
        parser.next('b');
        parser.next('c');
        parser.next(']');
        parser.next(']');
        assert!(!parser.call().is_empty());
    }
}
