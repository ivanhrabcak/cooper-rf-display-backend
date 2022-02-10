use config::Config;
use tokio::{fs::File, io::AsyncReadExt};

use crate::dongle::Dongle;

pub mod config;
pub mod dongle;
pub mod information;
pub mod storage;

const CONFIG_PATH: &'static str = "./config.toml";

#[tokio::main]
async fn main() {
    let mut config_file = match File::open(CONFIG_PATH).await {
        Ok(x) => x,
        Err(_) => File::create(CONFIG_PATH).await.unwrap(),
    };

    let mut config_data = String::new();
    match config_file.read_to_string(&mut config_data).await {
        Ok(_) => (),
        Err(e) => panic!("Failed to read config! {}", e.to_string()),
    };

    let config: Config = match toml::from_str(&config_data) {
        Ok(x) => x,
        Err(e) => panic!("Failed to deserialize config! {}", e.to_string()),
    };

    let dongle = Dongle::new(config.dongle_port);
}
