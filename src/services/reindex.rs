use crate::ctx::{Context, ProjectRoot};
use crate::models::File;
use std::ffi::OsStr;
use std::path::Path;

use walkdir::WalkDir;

const MARKDOWN_FILE_EXTENSION: &str = "md";

pub fn reindex(context: &Context) -> crate::Result<()> {
    context.db().drop()?;
    context.db().execute(|repository| {
        for file_result in files_iter(context.project_root()) {
            let mut file = file_result?;
            for reference in file.references_mut() {
                reference.upsert(&repository.references())?;
            }
            file.create(&repository.files())?;
        }
        Ok(())
    })?;

    Ok(())
}

fn files_iter(project_root: &ProjectRoot) -> impl Iterator<Item = crate::Result<File>> {
    WalkDir::new(project_root.file_path())
        .into_iter()
        .filter_entry(|e| !os_str_is_hidden(e.file_name()))
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|p| path_has_markdown_extension(p))
        .map(File::parse_from_path)
}

fn path_has_markdown_extension(path: &Path) -> bool {
    path.extension()
        .filter(|e| &MARKDOWN_FILE_EXTENSION == e)
        .is_some()
}

fn os_str_is_hidden(os_str: &OsStr) -> bool {
    os_str.to_str().map(|s| s.starts_with('.')).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_reindex_service_path_has_markdown_extension() {
        let path = PathBuf::from("test.md");
        assert!(path_has_markdown_extension(&path));
    }

    #[test]
    fn test_reindex_service_path_does_not_have_markdown_extension() {
        let path = PathBuf::from("test.exe");
        assert!(!path_has_markdown_extension(&path));
    }

    #[test]
    fn test_reindex_service_os_str_is_not_hidden() {
        let os_str = OsStr::new("test.md");
        assert!(!os_str_is_hidden(os_str));
    }

    #[test]
    fn test_reindex_service_os_str_is_hidden() {
        let os_str = OsStr::new(".test.md");
        assert!(os_str_is_hidden(os_str));
    }
}
