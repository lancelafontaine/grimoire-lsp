use std::path::PathBuf;

const CONFIG_DIRECTORY_NAME: &str = ".grimoire";
const CONFIG_FILE_NAME: &str = "config.toml";
const LOG_DIRECTORY_NAME: &str = "logs";
const LOG_FILE_NAME: &str = "log.txt";
const DB_FILE_NAME: &str = "grimoire.db";

#[derive(Debug)]
pub struct ProjectRoot {
    file_path: PathBuf,
}

impl ProjectRoot {
    pub fn current<T>(args: T) -> Option<Self>
    where
        T: Into<CurrentDirectoryArgs>,
    {
        let mut dir = args.into().current_dir.ok()?;
        loop {
            let config_dir = dir.join(CONFIG_DIRECTORY_NAME);
            match config_dir.exists() {
                true => break Some(Self { file_path: dir }),
                false => match dir.parent() {
                    Some(dir_parent) => dir = dir_parent.to_path_buf(),
                    None => break None,
                },
            }
        }
    }

    pub fn new<T>(args: T) -> crate::Result<Self>
    where
        T: Into<CurrentDirectoryArgs>,
    {
        let dir = args.into().current_dir?;
        Ok(Self { file_path: dir })
    }

    pub fn file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
    pub fn config_file_directory(&self) -> PathBuf {
        self.file_path.join(CONFIG_DIRECTORY_NAME)
    }

    pub fn config_file_path(&self) -> PathBuf {
        self.config_file_directory().join(CONFIG_FILE_NAME)
    }
    pub fn log_file_path(&self) -> PathBuf {
        self.log_file_directory().join(LOG_FILE_NAME)
    }

    pub fn log_file_directory(&self) -> PathBuf {
        self.config_file_directory().join(LOG_DIRECTORY_NAME)
    }

    pub fn db_file_path(&self) -> PathBuf {
        self.config_file_directory().join(DB_FILE_NAME)
    }
}

#[derive(Debug)]
pub struct CurrentDirectoryArgs {
    current_dir: crate::Result<PathBuf>,
}
impl Default for CurrentDirectoryArgs {
    fn default() -> Self {
        Self {
            current_dir: std::env::current_dir().map_err(|e| e.into()),
        }
    }
}
impl From<()> for CurrentDirectoryArgs {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<PathBuf> for CurrentDirectoryArgs {
    fn from(current_dir: PathBuf) -> Self {
        Self {
            current_dir: Ok(current_dir),
        }
    }
}

#[cfg(test)]
pub mod mocks {
    use super::*;
    use tempfile::Builder;

    impl ProjectRoot {
        pub fn mock() -> Self {
            let tmp_dir = Builder::new().prefix("grimoire").tempdir().unwrap();
            let path = tmp_dir.path().join("root");
            Self { file_path: path }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_directory_args_default() {
        assert!(CurrentDirectoryArgs::default().current_dir.is_ok());
    }

    #[test]
    fn test_current_directory_args_from_unit() {
        let current_directory_args: CurrentDirectoryArgs = ().into();
        assert!(current_directory_args.current_dir.is_ok());
    }

    #[test]
    fn test_current_directory_args_from_path_buf() -> crate::Result<()> {
        let path_buf = PathBuf::from("~");
        let current_directory_args: CurrentDirectoryArgs = path_buf.clone().into();
        assert_eq!(current_directory_args.current_dir?, path_buf);
        Ok(())
    }

    #[test]
    fn test_project_root_new_unit() {
        assert!(ProjectRoot::new(()).is_ok());
    }

    #[test]
    fn test_project_root_new_path_buf() {
        assert!(ProjectRoot::new(PathBuf::from("~")).is_ok());
    }

    #[test]
    fn test_project_root_file_path() -> crate::Result<()> {
        let path_buf = PathBuf::from("~");
        let project_root = ProjectRoot::new(path_buf)?;
        assert_eq!(project_root.file_path(), PathBuf::from("~"));
        Ok(())
    }

    #[test]
    fn test_project_root_config_file_directory() -> crate::Result<()> {
        let path_buf = PathBuf::from("~");
        let project_root = ProjectRoot::new(path_buf)?;
        assert_eq!(
            project_root.config_file_directory(),
            PathBuf::from("~/.grimoire")
        );
        Ok(())
    }

    #[test]
    fn test_project_root_config_file_path() -> crate::Result<()> {
        let path_buf = PathBuf::from("~");
        let project_root = ProjectRoot::new(path_buf)?;
        assert_eq!(
            project_root.config_file_path(),
            PathBuf::from("~/.grimoire/config.toml")
        );
        Ok(())
    }

    #[test]
    fn test_project_root_log_file_directory() -> crate::Result<()> {
        let path_buf = PathBuf::from("~");
        let project_root = ProjectRoot::new(path_buf)?;
        assert_eq!(
            project_root.log_file_directory(),
            PathBuf::from("~/.grimoire/logs")
        );
        Ok(())
    }

    #[test]
    fn test_project_root_log_file_path() -> crate::Result<()> {
        let path_buf = PathBuf::from("~");
        let project_root = ProjectRoot::new(path_buf)?;
        assert_eq!(
            project_root.log_file_path(),
            PathBuf::from("~/.grimoire/logs/log.txt")
        );
        Ok(())
    }

    #[test]
    fn test_project_root_db_file_path() -> crate::Result<()> {
        let path_buf = PathBuf::from("~");
        let project_root = ProjectRoot::new(path_buf)?;
        assert_eq!(
            project_root.db_file_path(),
            PathBuf::from("~/.grimoire/grimoire.db")
        );
        Ok(())
    }
}
