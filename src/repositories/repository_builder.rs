use crate::ctx::DatabaseTable;
use crate::repositories::{FilesRepository, ReferencesRepository};

#[derive(Debug)]
pub struct RepositoryBuilder {
    files_table: DatabaseTable,
    references_table: DatabaseTable,
}

impl RepositoryBuilder {
    pub fn new(conn: &crate::ctx::Connection) -> crate::Result<Self> {
        let files_table = conn.open_tree(b"files")?;
        let references_table = conn.open_tree(b"references")?;

        Ok(Self {
            files_table,
            references_table,
        })
    }

    pub fn clear(&self) -> crate::Result<()> {
        self.files_table.clear()?;
        self.references_table.clear()?;
        Ok(())
    }
    pub fn flush(&self) -> crate::Result<()> {
        self.files_table.flush()?;
        self.references_table.flush()?;
        Ok(())
    }

    pub fn files(&self) -> FilesRepository {
        FilesRepository::new(&self.files_table)
    }

    pub fn references(&self) -> ReferencesRepository {
        ReferencesRepository::new(&self.references_table)
    }
}

#[cfg(test)]
pub mod mocks {
    use super::*;
    use crate::ctx::Database;

    impl RepositoryBuilder {
        pub fn mock() -> Self {
            let conn = Database::mock_connection();
            RepositoryBuilder::new(&conn).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ctx::Database;

    #[test]
    fn test_repository_builder_new() {
        let conn = Database::mock_connection();
        let repository_builder = RepositoryBuilder::new(&conn);
        assert!(repository_builder.is_ok())
    }

    #[test]
    fn test_repository_builder_clear() {
        let repository_builder = RepositoryBuilder::mock();
        assert!(repository_builder.clear().is_ok())
    }

    #[test]
    fn test_repository_builder_flush() {
        let repository_builder = RepositoryBuilder::mock();
        assert!(repository_builder.flush().is_ok())
    }

    #[test]
    fn test_repository_builder_files() {
        RepositoryBuilder::mock().files();
    }

    #[test]
    fn test_repository_builder_references() {
        RepositoryBuilder::mock().references();
    }
}
