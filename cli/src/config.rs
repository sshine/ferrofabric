use config_manager::config;
use serde::{Deserialize, Serialize};

/// The runtime configuration filename
pub const RUNTIME_CONFIG_FILE: &str = "ferrofabric.toml";

/// Ferrofabric
#[derive(Debug, Clone, Serialize, Deserialize)]
#[config(clap(version, author, long_about), env_prefix = "ferrofabric")]
pub struct AppConfig {
    #[source(
        env,
        clap(
            long,
            short = 'd',
            help = "The ferrofabric root directory (non-existent or containing a ferrofabric.toml)"
        ),
        default = "."
    )]
    pub directory: String,

    #[source(
        env,
        clap(
            long,
            short = 'f',
            flag,
            help = "Use root directory even if it already exists and does not contain a ferrofabric.toml",
        ),
        default = false
    )]
    pub force: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[config()]
pub struct RuntimeConfig {}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {}
    }
}
