use chrono::Local;

use config::read_config;

use cors::CORS;
use rocket::{launch, routes};
use storage::Storage;
use tokio::task::spawn_blocking;

use crate::api::data::get_data_from_date;
use crate::api::data::get_data_points_for_date;
use crate::api::data::get_dates_with_data;
use crate::api::data::get_stations;
use crate::api::edupage::get_next_lesson;
use crate::api::edupage::get_substitution;
use crate::dongle::Dongle;
use crate::edupage::edupage::Edupage;
use crate::edupage::edupage_traits::Login;

pub mod api;
pub mod config;
pub mod cors;
pub mod dongle;
pub mod edupage;
pub mod information;
pub mod storage;

#[launch]
async fn rocket() -> _ {
    let config = read_config().await.unwrap();

    let username = (&config).edupage.username.clone();
    let password = (&config).edupage.password.clone();

    spawn_blocking(move || {
        let mut edupage = Edupage::new();
        edupage
            .login(&"gymlsba".to_string(), &username, &password)
            .unwrap();
    })
    .await
    .unwrap();

    let mut dongle = Dongle::new((&config).dongle_port.clone());

    let storage_directory = (&config).save_directory.clone();

    let stations = dongle.get_stations().unwrap();

    tokio::spawn(async move {
        let storage = Storage::new(storage_directory.to_string());
        loop {
            let information = dongle.wait_for_information();

            if information.is_err() {
                println!("{:?}", information);
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
        }
    });

    rocket::build()
        .mount(
            "/",
            routes![
                get_data_points_for_date,
                get_dates_with_data,
                get_data_from_date,
                get_substitution,
                get_stations,
                get_next_lesson
            ],
        )
        .manage(stations)
        .manage(config)
        .attach(CORS)
}
