use std::fs;

use serde_derive::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub keys: Option<Keys>,
    pub user_prefs: Option<UserPrefs>,
}
impl Config {
    pub fn new(keys: Keys, user_prefs: UserPrefs) -> Config {
        Config {
            keys: Some(keys),
            user_prefs: Some(user_prefs),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Keys {
    pub account_sid: Option<String>,
    pub auth_token: Option<String>,
}
impl Keys {
    pub fn new(account_sid: String, auth_token: String) -> Keys {
        Keys {
            account_sid: Some(account_sid),
            auth_token: Some(auth_token),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserPrefs {
    /// Default number from which all texts are sent
    pub from: Option<String>,
}

pub fn read_config_from_file(path: &str) -> Result<Config, toml::de::Error> {
    let file_contents =
        fs::read_to_string(path).expect("Something went wrong with reading the config file!");
    toml::from_str(&file_contents)
}
