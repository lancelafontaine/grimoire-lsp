use crate::ctx::{Database, ProjectRoot};
use crate::ctx::{Logger, StandardLogger};

#[derive(Debug)]
pub struct Context {
    project_root: ProjectRoot,
    db: Database,
}

impl Context {
    pub fn new<T>(args: T) -> crate::Result<Self>
    where
        T: Into<ContextArgs>,
    {
        let args = args.into();
        let project_root = args.project_root?;
        args.logger.initialize(&project_root)?;
        let db = args.db?;
        Ok(Self { project_root, db })
    }

    pub fn project_root(&self) -> &ProjectRoot {
        &self.project_root
    }

    pub fn db(&self) -> &Database {
        &self.db
    }
}

pub struct ContextArgs {
    project_root: crate::Result<ProjectRoot>,
    logger: Box<dyn Logger>,
    db: crate::Result<Database>,
}

impl Default for ContextArgs {
    fn default() -> Self {
        let logger = Box::new(StandardLogger::new());
        let project_root =
            ProjectRoot::current(()).ok_or_else(crate::errors::project_uninitialized);
        let db = match project_root {
            Ok(ref pr) => Database::new(Ok(pr)),
            Err(_) => Err(crate::errors::project_uninitialized()),
        };
        Self {
            project_root,
            logger,
            db,
        }
    }
}

impl From<()> for ContextArgs {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

#[cfg(test)]
pub mod mocks {
    use super::*;

    impl Context {
        pub fn mock() -> Self {
            Self {
                project_root: ProjectRoot::mock(),
                db: Database::mock(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ctx::logger::mocks::MockLogger;
    use std::path::PathBuf;

    #[test]
    fn test_context_args_project_root() -> crate::Result<()> {
        let project_root = ProjectRoot::mock();
        let logger = Box::new(MockLogger::mock());
        let db = Ok(Database::mock());
        let context_args = ContextArgs {
            project_root: Ok(project_root),
            logger,
            db,
        };
        assert_ne!(context_args.project_root?.file_path(), PathBuf::from(""));
        Ok(())
    }

    #[test]
    fn test_context_args_logger() -> crate::Result<()> {
        let project_root = ProjectRoot::mock();
        let logger = Box::new(MockLogger::mock());
        let db = Ok(Database::mock());
        let context_args = ContextArgs {
            project_root: Ok(project_root),
            logger,
            db,
        };
        assert!(context_args
            .logger
            .initialize(&context_args.project_root?)
            .is_ok());
        Ok(())
    }

    #[test]
    fn test_context_args_db() -> crate::Result<()> {
        let project_root = ProjectRoot::mock();
        let logger = Box::new(MockLogger::mock());
        let db = Ok(Database::mock());
        let context_args = ContextArgs {
            project_root: Ok(project_root),
            logger,
            db,
        };
        assert!(context_args.db?.drop().is_ok());
        Ok(())
    }

    #[test]
    fn test_context_new_from_context_args() {
        let project_root = ProjectRoot::mock();
        let logger = Box::new(MockLogger::mock());
        let db = Ok(Database::mock());
        let context_args = ContextArgs {
            project_root: Ok(project_root),
            logger,
            db,
        };
        let context = Context::new(context_args);
        assert!(context.is_ok());
    }

    #[test]
    fn test_context_project_root() {
        let context = Context::mock();
        assert_ne!(context.project_root().file_path(), PathBuf::from(""));
    }

    #[test]
    fn test_context_db() {
        let context = Context::mock();
        assert!(context.db().drop().is_ok());
    }
}
