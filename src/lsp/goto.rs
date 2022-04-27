use lsp_types::{Location, Position, Range, Url};

use crate::ctx::Context;
use crate::models::{File, Reference};

pub fn find_markdown_references(
    context: &Context,
    url: Url,
    position: Position,
) -> crate::Result<Vec<Location>> {
    let path = match url.to_file_path() {
        Ok(path) => path,
        Err(_) => return Err(crate::errors::invalid_path_from_url(url)),
    };
    let source_file = match File::parse_from_path(path) {
        Ok(source_file) => source_file,
        Err(err) => return Err(err),
    };
    let references: Vec<&Reference> = source_file
        .references()
        .iter()
        .filter(|reference| reference.location().contains(&position))
        .collect();
    if references.len() > 1 {
        return Err(crate::errors::invalid_overlapping_references_in_file(
            &source_file,
        ));
    }
    if references.is_empty() {
        let mut files = Vec::new();
        context.db().execute(|repository| {
            files = repository.files().find_all()?;
            Ok(())
        })?;

        return files
            .iter()
            .map(lsp_location_from_file)
            .collect::<crate::Result<Vec<Location>>>();
    }
    let reference = references[0];

    let mut file_option = None;
    context.db().execute(|repository| {
        file_option = repository.files().find(&reference.header())?;
        Ok(())
    })?;

    let file = match file_option {
        Some(file) => file,
        None => return Ok(Vec::new()),
    };

    Ok(vec![lsp_location_from_file(&file)?])
}

fn lsp_location_from_file(file: &File) -> crate::Result<Location> {
    let position = Position {
        line: file.header_location().line_position,
        character: file.header_location().start_char_position as u32,
    };
    let path = file.serializable_path()?;
    let mut prefixed_path = String::from("file:");
    prefixed_path.push_str(&path);
    let url = Url::parse(&prefixed_path)?;
    Ok(Location {
        uri: url,
        range: Range {
            start: position,
            end: position,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_location_from_file() {
        let file = File::mock(None);
        let location = lsp_location_from_file(&file);
        assert!(location.is_ok())
    }
}
