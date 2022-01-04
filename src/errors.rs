use anyhow::anyhow;

pub type Error = anyhow::Error;

pub fn project_uninitialized() -> Error {
    anyhow!("A Grimoire project hasn't been initialized yet.")
}

pub fn project_already_initialized() -> Error {
    anyhow!("A Grimoire project has already been initialized.")
}

#[cfg(test)]
pub mod test {
    use super::*;

    pub fn error() -> Error {
        anyhow!("A test error")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_uninitialized() {
        assert_eq!(
            project_uninitialized().to_string(),
            "A Grimoire project hasn't been initialized yet."
        );
    }

    #[test]
    fn test_project_already_initialized() {
        assert_eq!(
            project_already_initialized().to_string(),
            "A Grimoire project has already been initialized."
        );
    }
}
