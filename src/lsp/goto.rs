use lsp_types::{Location, Position, Range, Url};

pub fn find_markdown_reference(url: Url, position: Position) -> Option<Location> {
    // TODO: use the built index to resolve the title to the correct file

    let resolved_position = Position {
        character: 0,
        line: 0,
    };
    let resolved_url =
        Url::parse("file:/Users/lancelafontaine/code/grimoire-lsp/src/sample/README.md").unwrap();
    let _location = Location {
        uri: resolved_url,
        range: Range {
            start: resolved_position,
            end: resolved_position,
        },
    };

    let mut response_position = position;
    response_position.line = 0;

    let response_uri = url;
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
