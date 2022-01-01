use lsp_types::{Url, Position, Location, Range};

pub fn find_markdown_reference(url: Url, position: Position) -> Option<Location> {
    // TODO: build an index and use the index to resolve the title to the correct file

    let resolved_position = Position {
        character: 0,
        line: 0,
    };
    let resolved_url = Url::parse("file:/Users/lancelafontaine/code/grimoire-lsp/src/sample/README.md").unwrap();

    let location = Location {
        uri: resolved_url,
        range: Range {
            start: resolved_position,
            end: resolved_position,
        },
    };


    let mut response_position = position.clone();
    response_position.line = 0;

    let response_uri = url.clone();
    let response_range = Range {
        start: response_position,
        end: response_position,
    };
    let response_location = Location {
        uri: response_uri,
        range: response_range,
    };

    Some(response_location)
}

// TODO On grimoire lsp init, create a config file, a sqlite db, and a log file

