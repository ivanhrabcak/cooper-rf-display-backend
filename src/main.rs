use std::time::Duration;

use chrono::Local;
use config::read_config;

use rocket::{launch, routes};
use storage::Storage;

use crate::dongle::Dongle;

pub mod config;
pub mod dongle;
pub mod information;
pub mod storage;

#[launch]
async fn rocket() -> _ {
    let config = read_config().await.unwrap();

    let mut dongle = Dongle::new((&config).dongle_port.clone());
    let storage_directory = (&config).save_directory.clone();

    tokio::spawn(async move {
        let mut storage = Storage::new(storage_directory.to_string());
        loop {
            let information = dongle.wait_for_information();

            if information.is_err() {
                continue;
            }

            let information = information.unwrap();

            let now = Local::now();
            let filename = format!("{}.json", now.timestamp().to_string());

            let data = match serde_json::to_string_pretty(&information) {
                Ok(x) => x,
                Err(e) => {
                    println!("Failed to deserialize! {}", e.to_string());
                    continue;
                }
            };

            match storage.write_file(information.id, filename, data).await {
                Ok(_) => (),
                Err(e) => println!("Failed to write information to file! {}", e.to_string()),
            }

            tokio::time::sleep(Duration::from_millis(5000)).await;
        }
    });

    rocket::build().mount("/", routes![])
}
