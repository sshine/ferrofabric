use thiserror::Error;

pub type FerroResult<T> = Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    // #[error("Database error: {0}")]
    // DatabaseError(rusqlite::Error),

    // #[error("Template error: {0}")]
    // TemplateError(minijinja::Error),

    // #[error("Lock poisoned: {0}")]
    // LockError(String),

    // #[error("HTTP error: {0}")]
    // HttpError(http::Error),

    // #[error("Validation error: {0}")]
    // ValidationError(&'static str),
    #[error("Config error: {0}")]
    ConfigError(config_manager::Error),

    #[error("I/O error: {0}")]
    IoError(std::io::Error),

    #[error("TOML serialization error: {0}")]
    TomlError(toml::ser::Error),

    #[error("Setup error: {0}")]
    SetupError(&'static str),
}
