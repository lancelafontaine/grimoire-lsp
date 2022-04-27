use crate::models::File;
use anyhow::anyhow;
use std::path::Path;

pub type Error = anyhow::Error;

pub fn project_uninitialized() -> Error {
    let msg = "A Grimoire project hasn't been initialized yet.";
    log::error!("{}", msg);
    anyhow!(msg)
}

pub fn project_already_initialized() -> Error {
    let msg = "A Grimoire project has already been initialized.";
    log::error!("{}", msg);
    anyhow!(msg)
}

pub fn invalid_path_from_url(url: lsp_types::Url) -> Error {
    let msg = format!("Invalid path extracted from url {url:?}");
    log::error!("{}", msg);
    anyhow!(msg)
}

pub fn invalid_overlapping_references_in_file(file: &File) -> Error {
    let msg = format!("Invalid overlapping references in file: {file:?}");
    log::error!("{}", msg);
    anyhow!(msg)
}

pub fn markdown_header_not_found_during_parsing(path: &Path) -> Error {
    let msg = format!("A header for file {path:?} could not be found.");
    log::warn!("{}", msg);
    anyhow!(msg)
}

pub fn path_cannot_convert_to_string(path: &Path) -> Error {
    let msg = format!("The following path {path:?} could not represented as a string.");
    log::warn!("{}", msg);
    anyhow!(msg)
}

pub fn file_with_duplicate_header_created(old_file: &File, new_file: &File) -> Error {
    let new_file_path = new_file.path();
    let new_file_header = new_file.header();
    let old_file_path = old_file.path();
    let msg = format!("A file {new_file_path:?} was attempted to be saved with header {new_file_header:?}, but another file with the same header already existed at {old_file_path:?}");
    log::warn!("{}", msg);
    anyhow!(msg)
}

#[cfg(test)]
pub mod mocks {
    use super::*;

    pub fn mock_error() -> Error {
        anyhow!("A test error")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::File;
    use std::path::PathBuf;

    #[test]
    fn test_project_uninitialized() {
        assert_eq!(
            project_uninitialized().to_string(),
            "A Grimoire project hasn't been initialized yet."
        );
    }

    #[test]
    fn test_project_already_initialized() {
        assert_eq!(
            project_already_initialized().to_string(),
            "A Grimoire project has already been initialized."
        );
    }

    #[test]
    fn test_markdown_header_not_found_during_parsing() {
        let path_buf = PathBuf::from("~/foo.md");
        assert_eq!(
            markdown_header_not_found_during_parsing(&path_buf).to_string(),
            "A header for file \"~/foo.md\" could not be found."
        );
    }
    #[test]
    fn test_path_cannot_convert_to_string() {
        let path_buf = PathBuf::from("~");
        assert_eq!(
            path_cannot_convert_to_string(&path_buf).to_string(),
            "The following path \"~\" could not represented as a string."
        );
    }
    #[test]
    fn test_file_with_duplicate_header_created() {
        let old_file = File::mock(None);
        let new_file = File::mock(None);
        assert_eq!(
            file_with_duplicate_header_created(&old_file, &new_file).to_string(),
            format!("A file {:?} was attempted to be saved with header {:?}, but another file with the same header already existed at {:?}", new_file.serializable_path().unwrap(), new_file.header(), old_file.serializable_path().unwrap())
        );
    }

    #[test]
    fn test_invalid_path_from_url() {
        let url = lsp_types::Url::parse("file:/testing").unwrap();
        assert_eq!(
            invalid_path_from_url(url.clone()).to_string(),
            format!("Invalid path extracted from url {:?}", url)
        );
    }

    #[test]
    fn test_invalid_overlapping_references_in_file() {
        let file = File::mock(None);
        assert_eq!(
            invalid_overlapping_references_in_file(&file).to_string(),
            format!("Invalid overlapping references in file: {:?}", file)
        );
    }
}
