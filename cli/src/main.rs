use std::{
    fs,
    io::Write as _,
    path::{Path, PathBuf},
};

use config::{AppConfig, RuntimeConfig};
use config_manager::ConfigInit as _;
use error::{Error, FerroResult};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

mod config;
mod error;

fn main() {
    setup_tracing();

    let config = match setup_app_config() {
        Ok(config) => config,
        Err(why) => shutdown(why),
    };

    if let Err(why) = setup_directory(config) {
        shutdown(why);
    }
}

fn setup_tracing() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into());

    let fmt_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_ansi(true)
        .with_level(true)
        .with_file(true)
        .with_line_number(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();
}

fn setup_app_config() -> FerroResult<AppConfig> {
    let config = AppConfig::parse().map_err(Error::ConfigError)?;
    tracing::info!(config = ?config, "setup_config");
    Ok(config)
}

fn setup_directory(app_config: AppConfig) -> FerroResult<()> {
    let dir_path = Path::new(&app_config.directory);

    let mut runtime_config_path = PathBuf::from(dir_path);
    runtime_config_path.push(config::RUNTIME_CONFIG_FILE);

    let dir_exists = fs::exists(dir_path).map_err(Error::IoError)?;
    let config_exists = fs::exists(&runtime_config_path).map_err(Error::IoError)?;

    // Directory already exists, doesn't have a ferrofabric.toml, and --force is not applied
    if dir_exists && !config_exists && !app_config.force {
        return Err(Error::SetupError("Directory already exists, but does not contain a ferrofabric.toml; use --force to override."));
    }

    // ...otherwise create directory
    fs::create_dir_all(&dir_path).map_err(Error::IoError)?;

    // ...and create default runtime config inside
    if !config_exists {
        let runtime_config =
            toml::to_string(&RuntimeConfig::default()).map_err(Error::TomlError)?;
        let mut runtime_config_file =
            fs::File::create(&runtime_config_path).map_err(Error::IoError)?;
        runtime_config_file
            .write_all(runtime_config.as_bytes())
            .map_err(Error::IoError)?;
    }

    tracing::info!(created_config = !config_exists, "setup_directory");

    Ok(())
}

fn shutdown(why: Error) -> ! {
    tracing::error!("{}", why);
    std::process::exit(1);
}
