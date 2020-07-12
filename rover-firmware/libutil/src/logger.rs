use log4rs;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};

pub fn init_log(config_path: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = config_path {
        log4rs::init_file(path, Default::default())
            .map_err(|e| format!("Log initialization error: {}", e))?;
        Ok(())
    } else {
        let console_appender = ConsoleAppender::builder().build();
        let config = Config::builder()
            .appender(Appender::builder().build("console", Box::new(console_appender)))
            .build(
                Root::builder()
                    .appender("console")
                    .build(log::LevelFilter::Info),
            )?;
        log4rs::init_config(config).map_err(|e| format!("Log initialization error: {}", e))?;
        Ok(())
    }
}
