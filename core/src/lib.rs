pub mod config;
pub mod protocols;
pub mod router;
pub mod tun;
pub mod xray;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RayCrateError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Xray core error: {0}")]
    XrayError(String),
    #[error("TUN device error: {0}")]
    TunError(String),
    #[error("Network I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("URL parse error: {0}")]
    UrlError(#[from] url::ParseError),
    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, RayCrateError>;
