use anyhow::anyhow;

use std::path::PathBuf;

const CONFIG_DIRECTORY_NAME: &str = ".grimoire";
const LOG_DIRECTORY_NAME: &str = "logs";
const LOG_FILE_NAME: &str = "log.txt";

#[derive(Debug)]
pub struct ProjectRoot {
    filepath: PathBuf,
}

impl ProjectRoot {
    pub fn current() -> anyhow::Result<Self> {
        // It seems as though the LSP client sets the current working directory
        // to be the root directory that it's been configured with?
        // I'm not sure if all clients would do this or only Neovim's LSP client.
        // In any case, I'm raising an error here if there's an exception to this rule.
        let dir = std::env::current_dir()?.join(CONFIG_DIRECTORY_NAME);
        if !dir.exists() {
            return Err(anyhow!(
                "This directory isn't a part of a Grimoire project."
            ));
        }

        Ok(Self { filepath: dir })
    }

    pub fn log_file_path(&self) -> anyhow::Result<PathBuf> {
        let log_directory = self.filepath.join(LOG_DIRECTORY_NAME);
        if !log_directory.exists() {
            std::fs::create_dir_all(&log_directory)?;
        }

        let log_file = log_directory.join(LOG_FILE_NAME);
        if !log_file.exists() {
            std::fs::File::create(&log_file)?;
        }

        Ok(log_file.canonicalize()?)
    }
}
