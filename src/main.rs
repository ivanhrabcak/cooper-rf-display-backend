use std::io::{self, stdout, Write};

use config::Config;
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{config::EdupageConfiguration, dongle::Dongle};

pub mod config;
pub mod dongle;
pub mod information;
pub mod storage;

const CONFIG_PATH: &'static str = "./config.toml";

fn get_input() -> Result<Option<String>, String> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.replace("\n", "").replace("\r", "");
            if input.is_empty() {
                Ok(None)
            } else {
                Ok(Some(input))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tokio::main]
async fn main() {
    let mut config_file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("config.toml")
        .await
    {
        Ok(x) => x,
        Err(e) => panic!("Failed to open/create config! {}", e.to_string()),
    };

    let mut config_data = String::new();
    match config_file.read_to_string(&mut config_data).await {
        Ok(_) => (),
        Err(e) => panic!("Failed to read config! {}", e.to_string()),
    };

    let config: Config = match toml::from_str(&config_data) {
        Ok(x) => x,
        Err(e) => {
            if config_data.is_empty() {
                let dongle_serial_port: String;
                let edupage_username: String;
                let edupage_password: String;
                let save_directory: String;

                let mut config = Config::default();

                println!("You do not have a config file.");

                print!("Dongle serial port [{}]? ", &config.dongle_port);
                stdout().flush().unwrap();

                dongle_serial_port = get_input()
                    .unwrap()
                    .unwrap_or((&config.dongle_port).clone());

                print!("Data directory [{}]? ", &config.save_directory);
                stdout().flush().unwrap();

                save_directory = get_input()
                    .unwrap()
                    .unwrap_or((&config.save_directory).clone());

                print!("Edupage username [{}]? ", &config.edupage.username);
                stdout().flush().unwrap();

                edupage_username = get_input()
                    .unwrap()
                    .unwrap_or((&config.edupage.username).clone());

                print!("Edupage password [{}]? ", &config.edupage.password);
                stdout().flush().unwrap();

                edupage_password = get_input()
                    .unwrap()
                    .unwrap_or((&config.edupage.password).clone());

                {
                    config.dongle_port = dongle_serial_port;
                    config.edupage = EdupageConfiguration::new(edupage_username, edupage_password);
                    config.save_directory = save_directory;
                }

                config_file
                    .write(toml::to_string_pretty(&config).unwrap().as_bytes())
                    .await
                    .unwrap();
                config_file.flush().await.unwrap();

                config
            } else {
                panic!("Failed to deserialize config! {}", e.to_string())
            }
        }
    };

    println!("{:?}", &config);

    let dongle = Dongle::new(config.dongle_port);
}
