use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_yaml::{from_str, Value};
use std::fs;
use std::{collections::HashMap, sync::RwLock};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TiKVConfig {
    pdaddrs: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpConfig {
    port: u16,
    addr: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    tikv: TiKVConfig,
    http: HttpConfig,
}

impl TiKVConfig {
    pub fn default() -> Self {
        Self {
            pdaddrs: vec!["127.0.0.1:2379".to_string()],
        }
    }
}

impl HttpConfig {
    pub fn default() -> Self {
        Self {
            port: 3000,
            addr: "0.0.0.0".to_string(),
        }
    }
}

impl Config {
    pub fn default() -> Self {
        Self {
            tikv: TiKVConfig::default(),
            http: HttpConfig::default(),
        }
    }
}

pub fn generate_default_config(path: &str) -> Result<()> {
    let config = Config::default();
    let yml = serde_yaml::to_string(&config)?;
    fs::write(path, yml)?;
    Ok(())
}

lazy_static::lazy_static! {
    static ref SETTINGS: RwLock<config::Config> = RwLock::new({
        let settings = config::Config::default();
        settings
    });
    static ref CONFIG_FILE_PATH: RwLock<String> = RwLock::new({
        let path = "".to_string();
        path
    });
}

pub fn set_config(path: &str) {
    if path.is_empty() {
        SETTINGS
            .write()
            .unwrap()
            .merge(config::File::with_name("settings.toml"))
            .unwrap();
    } else {
        SETTINGS
            .write()
            .unwrap()
            .merge(config::File::with_name(path))
            .unwrap();
    }
}

pub fn set_config_file_path(path: String) {
    CONFIG_FILE_PATH.write().unwrap().clear();
    CONFIG_FILE_PATH.write().unwrap().push_str(path.as_str());
}

pub fn get_config_file_path() -> String {
    CONFIG_FILE_PATH.read().unwrap().clone()
}

pub fn get_config() -> Result<HashMap<String, String>, config::ConfigError> {
    SETTINGS
        .read()
        .unwrap()
        .clone()
        .try_into::<HashMap<String, String>>()
}
