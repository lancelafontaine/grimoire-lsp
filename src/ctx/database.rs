use crate::ctx::ProjectRoot;
use crate::repositories::RepositoryBuilder;

pub type DatabaseTable = sled::Tree;
pub type Connection = sled::Db;

#[derive(Debug)]
pub struct Database {
    repository_builder: RepositoryBuilder,
}

impl Database {
    pub fn new<T>(args: T) -> crate::Result<Self>
    where
        T: Into<DatabaseArgs>,
    {
        let args = args.into();
        let conn = args.conn?;
        let repository_builder = RepositoryBuilder::new(&conn)?;
        Ok(Self { repository_builder })
    }

    pub fn drop(&self) -> crate::Result<()> {
        self.execute(|_| {
            self.repository_builder.clear()?;
            Ok(())
        })?;
        Ok(())
    }

    pub fn execute<F>(&self, mut func: F) -> crate::Result<()>
    where
        F: FnMut(&RepositoryBuilder) -> crate::Result<()>,
    {
        // TODO: incorporate sled's transactions here
        let result = func(&self.repository_builder);
        self.repository_builder.flush()?;
        result
    }
}

pub struct DatabaseArgs {
    conn: crate::Result<Connection>,
}

impl DatabaseArgs {
    fn new_connection(project_root: &ProjectRoot) -> crate::Result<Connection> {
        fn to_connection(str_path: crate::Result<&str>) -> crate::Result<Connection> {
            Ok(sled::open(str_path?)?)
        }

        let path = project_root.db_file_path();
        let str_path: crate::Result<&str> = path
            .as_os_str()
            .to_str()
            .ok_or_else(|| crate::errors::path_cannot_convert_to_string(&path));

        to_connection(str_path)
    }
}

impl From<crate::Result<&ProjectRoot>> for DatabaseArgs {
    fn from(project_root: crate::Result<&ProjectRoot>) -> Self {
        Self {
            conn: project_root.and_then(Self::new_connection),
        }
    }
}

impl From<Connection> for DatabaseArgs {
    fn from(conn: Connection) -> Self {
        Self { conn: Ok(conn) }
    }
}

#[cfg(test)]
pub mod mocks {
    use super::*;
    use std::path::PathBuf;
    use tempfile::{Builder, TempDir};

    impl Database {
        pub fn mock() -> Self {
            Self::new(Self::mock_connection()).unwrap()
        }
        pub fn mock_connection() -> Connection {
            let tmp_dir = Builder::new().prefix("grimoire").tempdir().unwrap();
            let path = tmp_dir.path().join("sled");
            sled::open(path).unwrap()
        }
        pub fn mock_tmp_file(name: &str) -> (TempDir, PathBuf) {
            let tmp_dir = Builder::new().prefix("grimoire").tempdir().unwrap();
            let path = tmp_dir.path().join(name);
            (tmp_dir, path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_args_from_connection() {
        let conn = Database::mock_connection();
        let args: DatabaseArgs = conn.into();
        assert!(args.conn.is_ok());
    }

    #[test]
    fn test_database_args_from_project_root() {
        let project_root = ProjectRoot::mock();
        let project_root_result = Ok(&project_root);
        let args: DatabaseArgs = project_root_result.into();
        assert!(args.conn.is_ok());
    }

    #[test]
    fn test_database_new() {
        let conn = Database::mock_connection();
        let db = Database::new(conn);
        assert!(db.is_ok());
    }

    #[test]
    fn test_database_execute() {
        let db = Database::mock();
        let result = db.execute(|_| Ok(()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_database_drop() {
        let db = Database::mock();
        assert!(db.drop().is_ok());
    }
}
