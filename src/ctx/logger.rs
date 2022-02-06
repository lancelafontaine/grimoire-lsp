use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;

use log4rs::config::{Appender, Config, Root};

use crate::ctx::ProjectRoot;

use log::LevelFilter;
use std::env;

const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::Warn;

pub trait Logger {
    fn initialize(&self, project_root: &ProjectRoot) -> crate::Result<()>;
}

pub struct StandardLogger {}

impl StandardLogger {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for StandardLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl Logger for StandardLogger {
    fn initialize(&self, project_root: &ProjectRoot) -> crate::Result<()> {
        let log = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
            .build(project_root.log_file_path())?;

        let log_config = Config::builder()
            .appender(Appender::builder().build("log", Box::new(log)))
            .build(
                Root::builder()
                    .appender("log")
                    .build(log_level(env_var_log_level())),
            )?;

        log4rs::init_config(log_config).unwrap();
        Ok(())
    }
}

fn log_level(env_var_log_level: crate::Result<String>) -> LevelFilter {
    match env_var_log_level {
        Ok(lvl) => match &*lvl {
            "off" => LevelFilter::Off,
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "info" => LevelFilter::Info,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => DEFAULT_LOG_LEVEL,
        },
        Err(_) => DEFAULT_LOG_LEVEL,
    }
}

fn env_var_log_level() -> crate::Result<String> {
    env::var("GRIMOIRE_LOG_LEVEL").map_err(|e| e.into())
}

#[cfg(test)]
pub mod mocks {
    use super::*;

    pub struct MockLogger {}

    impl MockLogger {
        pub fn mock() -> Self {
            Self {}
        }
    }

    impl Logger for MockLogger {
        fn initialize(&self, _project_root: &ProjectRoot) -> crate::Result<()> {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_default() {
        assert_eq!(
            LevelFilter::Warn,
            log_level(Err(crate::errors::mocks::mock_error()))
        );
    }

    #[test]
    fn test_log_level_unrecognized() {
        assert_eq!(LevelFilter::Warn, log_level(Ok("foo".into())));
    }

    #[test]
    fn test_log_level_off() {
        assert_eq!(LevelFilter::Off, log_level(Ok("off".into())));
    }

    #[test]
    fn test_log_level_error() {
        assert_eq!(LevelFilter::Error, log_level(Ok("error".into())));
    }

    #[test]
    fn test_log_level_warn() {
        assert_eq!(LevelFilter::Warn, log_level(Ok("warn".into())));
    }

    #[test]
    fn test_log_level_info() {
        assert_eq!(LevelFilter::Info, log_level(Ok("info".into())));
    }

    #[test]
    fn test_log_level_debug() {
        assert_eq!(LevelFilter::Debug, log_level(Ok("debug".into())));
    }

    #[test]
    fn test_log_level_trace() {
        assert_eq!(LevelFilter::Trace, log_level(Ok("trace".into())));
    }
}
