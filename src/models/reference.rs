use crate::models::Location;
use crate::repositories::ReferencesRepository;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Reference {
    path: PathBuf,
    header: String,
    header_location: Location,
    persisted: bool,
}

impl Reference {
    pub fn new(path: PathBuf, header: String, header_location: Location) -> Self {
        Self {
            path,
            header,
            header_location,
            persisted: false,
        }
    }

    pub fn serializable_path(&self) -> crate::Result<String> {
        Ok(String::from(self.path.to_str().ok_or_else(|| {
            crate::errors::path_cannot_convert_to_string(&self.path)
        })?))
    }

    pub fn header(&self) -> String {
        self.header.clone()
    }

    pub fn location(&self) -> &Location {
        &self.header_location
    }

    pub fn upsert(&mut self, repository: &ReferencesRepository) -> crate::Result<()> {
        repository.upsert_reference(self)?;
        self.persisted = true;
        Ok(())
    }
}

#[cfg(test)]
pub mod mocks {
    use super::*;
    use tempfile::Builder;

    impl Reference {
        pub fn mock() -> Self {
            let tmp_dir = Builder::new().prefix("grimoire").tempdir().unwrap();
            let path = tmp_dir.path().join("root");
            Reference::new(path, String::from("A Nice Reference"), Location::default())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ctx::Database;
    use tempfile::Builder;

    #[test]
    fn reference_new() -> crate::Result<()> {
        let tmp_dir = Builder::new().prefix("grimoire").tempdir()?;
        let header = String::from("A Nice Reference");
        let path = tmp_dir.path().join("root");
        let reference = Reference::new(path, header.clone(), Location::default());
        assert_eq!(reference.header(), header);
        Ok(())
    }

    #[test]
    fn reference_header() -> crate::Result<()> {
        let tmp_dir = Builder::new().prefix("grimoire").tempdir()?;
        let header = String::from("A Nice Reference");
        let path = tmp_dir.path().join("root");
        let reference = Reference::new(path, header.clone(), Location::default());
        assert_eq!(reference.header(), header);
        Ok(())
    }

    #[test]
    fn reference_serializable_path() -> crate::Result<()> {
        let tmp_dir = Builder::new().prefix("grimoire").tempdir()?;
        let header = String::from("A Nice Reference");
        let path = tmp_dir.path().join("root");
        let reference = Reference::new(path.clone(), header, Location::default());
        assert_eq!(
            reference.serializable_path()?,
            path.as_os_str()
                .to_str()
                .ok_or_else(|| crate::errors::path_cannot_convert_to_string(&path))?
        );
        Ok(())
    }

    #[test]
    fn reference_location() {
        let reference = Reference::mock();
        assert_eq!(
            reference.location().line_position,
            Location::default().line_position
        );
        assert_eq!(
            reference.location().char_position,
            Location::default().char_position
        );
    }

    #[test]
    fn reference_upsert() -> crate::Result<()> {
        let db = Database::mock();
        let mut reference = Reference::mock();
        assert!(!reference.persisted);

        db.execute(|repositories| {
            let references_repository = repositories.references();
            reference.upsert(&references_repository).map(|_| ())
        })?;
        assert!(reference.persisted);
        Ok(())
    }
}
