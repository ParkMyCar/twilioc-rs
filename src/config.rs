use std::fs;

use serde_derive::{Deserialize, Serialize};
use toml

#[derive(Debug, Deserialize)]
pub struct Config {
    pub keys: Option<Keys>,
}
impl Config {
    pub fn new(keys: Keys) -> Config {
        keys: Some(keys)
    }
}

#[derive(Debug, Deserialize)]
pub struct Keys {
    pub account_sid: Option<String>,
    pub auth_token: Option<String>,
}
impl Keys {
    pub fn new(account_sid: String, auth_token: String) -> {
        Keys {
            account_sid: Some(account_sid),
            auth_token: Some(auth_token),
        }
    }
}

pub fn read_config_from_file(path: String) -> Result<Config, toml::de::Error> {
    let file_contents = fs::read_to_string(config_filename)
        .expect("Something went wrong with reading the config file!");
    toml::from_str(&file_contents)
}
