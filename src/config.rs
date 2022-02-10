use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub dongle_port: String,
    pub save_directory: String,
}
