use crate::models::File;

pub struct FilesRepository<'a> {
    table: &'a crate::ctx::DatabaseTable,
}

impl<'a> FilesRepository<'a> {
    pub fn new(table: &'a crate::ctx::DatabaseTable) -> Self {
        Self { table }
    }

    pub fn create_file(&self, file: &File) -> crate::Result<()> {
        let header = &file.header();
        let key = serde_json::to_vec(header)?;
        let value = serde_json::to_vec(file)?;

        if let Some(old_file) = self.find(header)? {
            return Err(crate::errors::file_with_duplicate_header_created(
                &old_file, file,
            ));
        }
        self.table.insert(&key, value)?;
        Ok(())
    }

    pub fn find(&self, header: &str) -> crate::Result<Option<File>> {
        let key = serde_json::to_vec(&header)?;
        if let Some(value) = self.table.get(&key)? {
            let value: &[u8] = &value;
            let file: File = serde_json::from_slice(value)?;
            return Ok(Some(file));
        }
        Ok(None)
    }

    pub fn find_all(&self) -> crate::Result<Vec<File>> {
        let mut files = Vec::new();
        for entry_result in self.table.iter() {
            let entry = entry_result?;
            let (_, value) = entry;
            let value: &[u8] = &value;
            let file: File = serde_json::from_slice(value)?;
            files.push(file);
        }
        Ok(files)
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
        assert!(repository.create_file(&file).is_err());
    }

    #[test]
    fn test_files_repositority_find() {
        let repository_builder = RepositoryBuilder::mock();
        let repository = repository_builder.files();
        let file = File::mock(None);
        assert!(repository.find(&file.header()).unwrap().is_none());
        assert!(repository.create_file(&file).is_ok());
        assert!(repository.find(&file.header()).unwrap().is_some());
    }

    #[test]
    fn test_files_repositority_find_all() {
        let repository_builder = RepositoryBuilder::mock();
        let repository = repository_builder.files();
        assert!(repository.find_all().unwrap().is_empty());
        let file1 = File::mock(None);
        assert!(repository.create_file(&file1).is_ok());
        assert_eq!(repository.find_all().unwrap().len(), 1);
    }
}
