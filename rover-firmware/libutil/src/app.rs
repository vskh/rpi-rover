use std::borrow::Cow;

use config::Config;
use log::debug;

use crate::logger::init_log;
use crate::sys::normalize_path;

pub fn bootstrap(config_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let self_path =
        std::env::current_exe().map_err(|_| "Could not get current executable path.")?;
    let self_name = self_path
        .file_name()
        .map(|s| s.to_string_lossy())
        .unwrap_or(Cow::from("<unknown>"));

    debug!(
        "Bootstrapping {} with configuration from {}.",
        self_name, config_path
    );

    let current_dir =
        std::env::current_dir().map_err(|_| "Could not get current working directory.")?;

    // load settings
    let config_path = normalize_path(config_path, &current_dir);

    let mut builder = Config::builder().add_source(config::File::with_name(&config_path));
    let settings = builder.build()?;

    // initialize logging
    init_log(
        settings
            .get_string("log_config")
            .map(|r| normalize_path(&r, &current_dir))
            .ok(),
    )?;

    Ok(settings)
}
