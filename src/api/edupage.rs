use chrono::NaiveDateTime;
use rocket::{get, State};

use crate::config::Config;
use crate::edupage::edupage::{Edupage, EdupageError};
use crate::edupage::edupage_traits::{Login, Substitution};
use tokio::task::spawn_blocking;

use super::response::Response;

#[get("/api/edupage/substitution")]
pub async fn get_substitution(config: &State<Config>) -> Response<String> {
    let username = (&config).edupage.username.clone();
    let password = (&config).edupage.password.clone();

    match spawn_blocking(move || {
        let mut edupage = Edupage::new();
        match edupage.login(&"gymlsba".to_string(), &username, &password) {
            Ok(_) => (),
            Err(_) => {
                return Err(EdupageError::NotLoggedIn);
            }
        };

        let now = chrono::offset::Local::now();
        let today_date = NaiveDateTime::from_timestamp(now.timestamp(), 0).date();

        edupage.get_substitution_html(&today_date, &"gymlsba".to_string())
    })
    .await
    {
        Ok(x) => {
            if x.is_err() {
                Response::new("Failed to fetch substitution".to_string(), 500)
            } else {
                Response::new(x.unwrap(), 200)
            }
        }
        Err(_) => Response::new("Failed to fetch substitution".to_string(), 500),
    }
}
