use crate::models::File;

pub struct FilesRepository<'a> {
    table: &'a crate::ctx::DatabaseTable,
}

impl<'a> FilesRepository<'a> {
    pub fn new(table: &'a crate::ctx::DatabaseTable) -> Self {
        Self { table }
    }

    pub fn create_file(&self, file: &File) -> crate::Result<()> {
        let key = serde_json::to_vec(&file.header())?;
        let value = serde_json::to_vec(file)?;

        if let Some(old_value) = self.table.get(&key)? {
            let old_value: &[u8] = &old_value;
            let old_file: File = serde_json::from_slice(old_value)?;
            return Err(crate::errors::file_with_duplicate_header_created(
                &old_file, file,
            ));
        }
        self.table.insert(&key, value)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::RepositoryBuilder;

    #[test]
    fn test_files_repositority_create_file_success() {
        let repository_builder = RepositoryBuilder::mock();
        let repository = repository_builder.files();
        let file = File::mock(None);
        assert!(repository.create_file(&file).is_ok());
    }

    #[test]
    fn test_files_repositority_create_file_failure_duplicate_header() {
        let repository_builder = RepositoryBuilder::mock();
        let repository = repository_builder.files();
        let file = File::mock(None);
        assert!(repository.create_file(&file).is_ok());
        assert!(!repository.create_file(&file).is_ok());
    }
}
