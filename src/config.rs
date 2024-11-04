use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ServerConfig {
    pub quic: Option<QuicConfig>,
    pub tcp: Option<TcpConfig>,
    pub websocket: Option<WebSocketConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct QuicConfig {
    pub address: String,
    pub port: u16,
    pub cert_path: String,
    pub key_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TcpConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WebSocketConfig {
    pub address: String,
    pub port: u16,
    pub path: String,
}

pub fn load_config(config_path: &str) -> Result<ServerConfig, config::ConfigError> {
    Config::builder()
        .add_source(File::with_name(config_path))
        .build()?
        .try_deserialize::<ServerConfig>()
}