use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthConfig {
    pub api_keys: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub auth: AuthConfig,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_host() -> String {
    String::from("0.0.0.0")
}

fn default_port() -> u16 {
    6322
}

pub fn read_config() -> Result<Config, figment::Error> {
    Figment::new().merge(Toml::file("operator.toml")).extract()
}
