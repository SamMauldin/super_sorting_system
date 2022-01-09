use std::collections::HashMap;

use actix_web::web;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{Vec2, Vec3};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ComplexConfig {
    pub dimension: String,
    pub y_level: i32,
    pub bounds: (Vec2, Vec2),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthConfig {
    pub agent_api_keys: Vec<Uuid>,
    pub admin_api_keys: Vec<Uuid>,
    pub automation_api_keys: Vec<Uuid>,
    pub data_api_keys: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PathfindingPortal {
    pub location: Vec3,
    pub connects_to: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PathfindingNode {
    pub pretty_name: Option<String>,
    pub dimension: String,
    pub location: Vec3,
    pub connections: Vec<String>,
    pub portal: Option<PathfindingPortal>,
    pub chest: Option<Vec3>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PathfindingConfig {
    pub nodes: HashMap<String, PathfindingNode>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub complex: ComplexConfig,
    pub auth: AuthConfig,
    pub pathfinding: PathfindingConfig,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

pub type ConfigData = web::Data<Config>;

fn default_host() -> String {
    String::from("0.0.0.0")
}

fn default_port() -> u16 {
    6322
}

pub fn read_config() -> Result<Config, figment::Error> {
    Figment::new().merge(Toml::file("operator.toml")).extract()
}
