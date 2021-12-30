use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Handle;

use log4rs::config::{Appender, Config, Root};

use log::LevelFilter;
use std::env;

const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::Warn;

pub fn initialize_logger() -> anyhow::Result<Handle> {
    let project_root = crate::project_root::ProjectRoot::current()?;

    let log = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build(project_root.log_file_path()?)?;

    let log_config = Config::builder()
        .appender(Appender::builder().build("log", Box::new(log)))
        .build(Root::builder().appender("log").build(log_level()))?;

    let handle = log4rs::init_config(log_config).unwrap();
    Ok(handle)
}

fn log_level() -> LevelFilter {
    match env::var("GRIMOIRE_LOG_LEVEL") {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_level_default() {
        assert_eq!(LevelFilter::Warn, log_level());
    }
}
