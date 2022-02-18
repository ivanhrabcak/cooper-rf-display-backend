use chrono::{Local, NaiveDateTime, NaiveTime};
use rocket::{get, State};

use crate::config::Config;
use crate::edupage::edupage::{Edupage, EdupageError};
use crate::edupage::edupage_traits::{Login, Ringing, Substitution};
use tokio::task::spawn_blocking;

use super::data::NaiveDateForm;
use super::response::Response;

#[get("/api/edupage/substitution/<date>")]
pub async fn get_substitution(config: &State<Config>, date: NaiveDateForm) -> Response<String> {
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

        edupage.get_substitution_html(&date.0, &"gymlsba".to_string())
    })
    .await
    {
        Ok(x) => {
            if x.is_err() {
                Response::new("Failed to fetch substitution".to_string(), 500)
            } else {
                let x = x.unwrap();
                let x = match x.split("<span class=\"print-font-resizable\">").nth(1) {
                    Some(x) => x.split("</span>").nth(0).unwrap(),
                    None => return Response::new("Failed to fetch substitution".to_string(), 500),
                };
                Response::new(x.to_string(), 200)
            }
        }
        Err(_) => Response::new("Failed to fetch substitution".to_string(), 500),
    }
}

#[get("/api/edupage/nextlesson?<hours>&<minutes>")]
pub async fn get_next_lesson(config: &State<Config>, hours: u32, minutes: u32) -> Response<String> {
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

        let now = Local::now();
        let now_date = now.date().naive_local();

        match edupage.get_next_lesson_time(NaiveDateTime::new(
            now_date,
            NaiveTime::from_hms(hours, minutes, 0),
        )) {
            Some(x) => Ok(x.time().format("%H:%M").to_string()),
            None => {
                let now = Local::now();
                let day = now.date().format("%a").to_string();

                if day == "Fri" {
                    Ok("Weekend!".to_string())
                } else {
                    Err(EdupageError::ParseError(
                        "Error while parsing ringing times".to_string(),
                    ))
                }
            }
        }
    })
    .await
    {
        Ok(x) => match x {
            Ok(r) => Response::new(r, 200),
            Err(_) => Response::new("Server error!".to_string(), 500),
        },
        Err(_) => Response::new("Fatal server error!".to_string(), 500),
    }
}
