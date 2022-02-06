use crate::models::{Location, Reference};
use crate::parsers::{HeaderParser, Parser, ReferenceParser};
use crate::repositories::FilesRepository;
use serde::{Deserialize, Serialize};
use std::fs::File as FsFile;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    path: PathBuf,
    header: String,
    header_location: Location,
    references: Vec<Reference>,
    persisted: bool,
}

impl File {
    pub fn parse_from_path(path: PathBuf) -> crate::Result<Self> {
        let mut header_parser = HeaderParser::new();
        let mut reference_parser = ReferenceParser::new();

        let file = BufReader::new(FsFile::open(&path)?);
        for line in file.lines() {
            for c in line?.chars() {
                header_parser.next(c);
                reference_parser.next(c);
            }
            header_parser.next('\n');
            reference_parser.next('\n')
        }

        let header_parser_payload = header_parser
            .call()
            .ok_or_else(|| crate::errors::markdown_header_not_found_during_parsing(&path))?;
        let references = reference_parser
            .call()
            .into_iter()
            .map(|payload| Reference::new(path.clone(), payload.header, payload.location))
            .collect::<Vec<Reference>>();

        let parsed_markdown = Self {
            path,
            header: header_parser_payload.header,
            header_location: header_parser_payload.location,
            references,
            persisted: false,
        };
        Ok(parsed_markdown)
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    pub fn serializable_path(&self) -> crate::Result<String> {
        Ok(String::from(self.path.to_str().ok_or_else(|| {
            crate::errors::path_cannot_convert_to_string(&self.path)
        })?))
    }

    pub fn header(&self) -> String {
        self.header.clone()
    }

    pub fn header_location(&self) -> &Location {
        &self.header_location
    }

    pub fn references(&self) -> &Vec<Reference> {
        &self.references
    }

    pub fn references_mut(&mut self) -> &mut Vec<Reference> {
        &mut self.references
    }

    pub fn persisted(&self) -> bool {
        self.persisted
    }

    pub fn create(&mut self, repository: &FilesRepository) -> crate::Result<()> {
        repository.create_file(self)?;
        self.persisted = true;
        Ok(())
    }
}

#[cfg(test)]
pub mod mocks {
    use super::*;
    use crate::ctx::Database;
    use tempfile::TempDir;

    impl File {
        pub fn mock(data: Option<String>) -> Self {
            let (_temp_dir, path) = Self::mock_disk_file(data);
            File::parse_from_path(path).unwrap()
        }

        pub fn mock_disk_file(mut data: Option<String>) -> (TempDir, PathBuf) {
            let (tmp_dir, path) = Database::mock_tmp_file("test.md");

            if data.is_none() {
                data = Some(String::from(
                    "\
                    # I am a title

                    [[Test Reference]]\
                ",
                ));
            }
            std::fs::write(path.clone(), data.unwrap()).unwrap();
            (tmp_dir, path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ctx::Database;

    #[test]
    fn file_parse_from_path_success() {
        let data = String::from(
            "\
            # I am a title

            [[Test Reference]]\
        ",
        );
        let (_tmp_dir, path) = File::mock_disk_file(Some(data));
        let file = File::parse_from_path(path);
        assert!(file.is_ok());
    }

    #[test]
    fn file_parse_from_path_failure_header_not_found() {
        let data = String::from(
            "\
            [[Test Reference]]\
        ",
        );
        let (_tmp_dir, path) = File::mock_disk_file(Some(data));
        let file = File::parse_from_path(path.clone());
        assert_eq!(
            file.unwrap_err().to_string(),
            crate::errors::markdown_header_not_found_during_parsing(&path).to_string()
        );
    }

    #[test]
    fn file_path() {
        let data = String::from(
            "\
            # I am a title

            [[Test Reference]]\
        ",
        );
        let (_tmp_dir, path) = File::mock_disk_file(Some(data));
        let file = File::parse_from_path(path.clone()).unwrap();
        assert_eq!(file.path(), &path);
    }

    #[test]
    fn file_serializable_path() {
        let data = String::from(
            "\
            # I am a title

            [[Test Reference]]\
        ",
        );
        let (_tmp_dir, path) = File::mock_disk_file(Some(data));
        let file = File::parse_from_path(path.clone()).unwrap();
        assert_eq!(
            file.serializable_path().unwrap(),
            path.as_os_str().to_str().unwrap()
        );
    }

    #[test]
    fn file_persisted() {
        let file = File::mock(None);
        assert!(!file.persisted());
    }

    #[test]
    fn file_header() {
        let data = String::from(
            "\
            # I am a title

            [[Test Reference]]\
        ",
        );
        let (_tmp_dir, path) = File::mock_disk_file(Some(data));
        let file = File::parse_from_path(path).unwrap();
        assert_eq!(file.header(), String::from("I am a title"));
    }

    #[test]
    fn file_header_location() {
        let data = String::from(
            "
            # I am a title

            [[Test Reference]]\
        ",
        );
        let (_tmp_dir, path) = File::mock_disk_file(Some(data));
        let file = File::parse_from_path(path).unwrap();
        let location = Location {
            line_position: 2,
            char_position: 16,
        };
        assert_eq!(file.header_location().line_position, location.line_position);
        assert_eq!(file.header_location().char_position, location.char_position);
    }

    #[test]
    fn file_references() {
        let data = String::from(
            "
            # I am a title

            [[Reference1]]
        ",
        );
        let (_tmp_dir, path) = File::mock_disk_file(Some(data));
        let file = File::parse_from_path(path.clone()).unwrap();
        let location = Location {
            line_position: 4,
            char_position: 16,
        };
        let reference = Reference::new(path, String::from("Reference1"), location);
        assert_eq!(file.references().len(), 1);
        assert_eq!(
            file.references().first().unwrap().header(),
            reference.header()
        );
        assert_eq!(
            file.references().first().unwrap().location(),
            reference.location()
        );
    }
    #[test]
    fn file_references_mut() {
        let data = String::from(
            "
            # I am a title

            [[Reference1]]
        ",
        );
        let (_tmp_dir, path) = File::mock_disk_file(Some(data));
        let mut file = File::parse_from_path(path.clone()).unwrap();
        let location = Location {
            line_position: 4,
            char_position: 16,
        };
        let reference = Reference::new(path, String::from("Reference1"), location);
        assert_eq!(file.references_mut().len(), 1);
        assert_eq!(
            file.references().first().unwrap().header(),
            reference.header()
        );
        assert_eq!(
            file.references().first().unwrap().location(),
            reference.location()
        );
    }

    #[test]
    fn file_create() {
        let db = Database::mock();
        let mut file = File::mock(None);
        assert!(!file.persisted);

        db.execute(|repositories| {
            let files_repository = repositories.files();
            file.create(&files_repository).map(|_| ())
        })
        .unwrap();
        assert!(file.persisted);
    }
}
