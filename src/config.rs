use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub dongle_port: String,
    pub save_directory: String,
    pub edupage: EdupageConfiguration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dongle_port: "COM1".to_string(),
            save_directory: "./data".to_string(),
            edupage: EdupageConfiguration::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EdupageConfiguration {
    pub username: String,
    pub password: String,
}

impl EdupageConfiguration {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

impl Default for EdupageConfiguration {
    fn default() -> Self {
        Self {
            username: "username".to_string(),
            password: "password".to_string(),
        }
    }
}
