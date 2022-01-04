use std::path::PathBuf;

const CONFIG_DIRECTORY_NAME: &str = ".grimoire";
const CONFIG_FILE_NAME: &str = "config.toml";
const LOG_DIRECTORY_NAME: &str = "logs";
const LOG_FILE_NAME: &str = "log.txt";

#[derive(Debug)]
pub struct ProjectRoot {
    filepath: PathBuf,
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
                true => {
                    break Some(Self {
                        filepath: config_dir,
                    })
                }
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
        Ok(Self {
            filepath: dir.join(CONFIG_DIRECTORY_NAME),
        })
    }

    pub fn config_file_path(&self) -> PathBuf {
        self.filepath.join(CONFIG_FILE_NAME)
    }
    pub fn log_file_path(&self) -> PathBuf {
        self.log_file_directory().join(LOG_FILE_NAME)
    }

    pub fn log_file_directory(&self) -> PathBuf {
        self.filepath.join(LOG_DIRECTORY_NAME)
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
    fn test_current_directory_args_from_path_buf() {
        let path_buf = PathBuf::from("~");
        let current_directory_args: CurrentDirectoryArgs = path_buf.clone().into();
        assert_eq!(current_directory_args.current_dir.unwrap(), path_buf);
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
    fn test_project_root_config_file_path() {
        let path_buf = PathBuf::from("~");
        let project_root = ProjectRoot::new(path_buf).unwrap();
        assert_eq!(
            project_root.config_file_path(),
            PathBuf::from("~/.grimoire/config.toml")
        );
    }

    #[test]
    fn test_project_root_log_file_directory() {
        let path_buf = PathBuf::from("~");
        let project_root = ProjectRoot::new(path_buf).unwrap();
        assert_eq!(
            project_root.log_file_directory(),
            PathBuf::from("~/.grimoire/logs")
        );
    }

    #[test]
    fn test_project_root_log_file_path() {
        let path_buf = PathBuf::from("~");
        let project_root = ProjectRoot::new(path_buf).unwrap();
        assert_eq!(
            project_root.log_file_path(),
            PathBuf::from("~/.grimoire/logs/log.txt")
        );
    }
}
