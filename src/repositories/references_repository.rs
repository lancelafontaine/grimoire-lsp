use crate::ctx::DatabaseTable;
use crate::models::Reference;

pub struct ReferencesRepository<'a> {
    table: &'a DatabaseTable,
}

impl<'a> ReferencesRepository<'a> {
    pub fn new(table: &'a DatabaseTable) -> Self {
        Self { table }
    }

    pub fn upsert_reference(&self, reference: &Reference) -> crate::Result<()> {
        let key = serde_json::to_vec(&reference.header())?;
        let value = serde_json::to_vec(reference)?;
        self.table.insert(&key, value)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::RepositoryBuilder;

    #[test]
    fn test_references_repositority_create_file_success() {
        let repository_builder = RepositoryBuilder::mock();
        let repository = repository_builder.references();
        let reference = Reference::mock();
        assert!(repository.upsert_reference(&reference).is_ok());
        assert!(repository.upsert_reference(&reference).is_ok());
    }
}
