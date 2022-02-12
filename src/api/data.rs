use std::collections::{HashMap, HashSet};

use chrono::{NaiveDate, NaiveDateTime, ParseError};
use rocket::{
    get,
    http::{ContentType, Status},
    request::FromParam,
    State,
};

use crate::{config::Config, information::Information, storage::Storage};

use super::response::Response;

#[derive(Debug, PartialEq)]
pub struct NaiveDateForm(pub NaiveDate);

impl<'a> FromParam<'a> for NaiveDateForm {
    type Error = ParseError;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match NaiveDate::parse_from_str(&param, "%Y-%m-%d") {
            Ok(date) => Ok(NaiveDateForm(date)),
            Err(e) => Err(e),
        }
    }
}

#[get("/api/data/points/<date>", rank = 2)]
pub async fn get_data_points_for_date(
    date: NaiveDateForm,
    config: &State<Config>,
    stations: &State<HashMap<String, String>>,
) -> Response<HashMap<String, Vec<String>>> {
    let mut data_points: HashMap<String, _> = HashMap::new();

    let storage = Storage::new((&config).save_directory.clone());

    for (station, _) in stations.iter() {
        let mut station_data_points = Vec::<String>::new();

        let files = match storage.get_files_for_station(&station).await {
            Ok(x) => x,
            Err(_) => continue,
        };

        for file in files {
            let file = file.replace(".json", "");
            let timestamp: i64 = file.parse().unwrap();

            let file_date = NaiveDateTime::from_timestamp(timestamp, 0);

            if NaiveDateForm(file_date.date()) != date {
                continue;
            }
            station_data_points.push(file_date.format("%Y-%m-%d %H:%M:%S").to_string());
        }

        data_points.insert((&station).to_string(), station_data_points);
    }

    Response::new(data_points, 200)
}

#[get("/api/data/points")]
pub async fn get_dates_with_data(
    config: &State<Config>,
    stations: &State<HashMap<String, String>>,
) -> Response<Vec<String>> {
    let storage = Storage::new((&config).save_directory.clone());

    let mut dates_with_data = HashSet::new();

    for (station, _) in stations.iter() {
        let files = match storage.get_files_for_station(&station).await {
            Ok(x) => x,
            Err(_) => continue,
        };

        for file in files {
            let file = file.replace(".json", "");
            let timestamp: i64 = file.parse().unwrap();

            let file_date = NaiveDateTime::from_timestamp(timestamp, 0);

            dates_with_data.insert(file_date.format("%Y-%m-%d").to_string());
        }
    }

    Response::new(dates_with_data.into_iter().collect(), 200)
}

#[get("/api/data/<date>/<format>", rank = 3)]
pub async fn get_data_from_date(
    date: NaiveDateForm,
    format: String,
    config: &State<Config>,
    stations: &State<HashMap<String, String>>,
) -> (Status, (ContentType, String)) {
    let mut data_points: HashMap<String, HashMap<String, Information>> = HashMap::new();

    let storage = Storage::new((&config).save_directory.clone());

    for (station, _) in stations.iter() {
        let mut station_data_points = HashMap::new();

        let files = match storage.get_files_for_station(&station).await {
            Ok(x) => x,
            Err(_) => continue,
        };

        for file in files {
            let file = file.replace(".json", "");
            let timestamp: i64 = file.parse().unwrap();

            let file_date = NaiveDateTime::from_timestamp(timestamp, 0);

            if NaiveDateForm(file_date.date()) != date {
                continue;
            }

            let information = match storage.read_information(station, timestamp).await {
                Ok(x) => x,
                Err(e) => {
                    println!("Error while reading information {}", e.to_string());
                    continue;
                }
            };

            station_data_points.insert(
                file_date.format("%Y-%m-%d %H:%M:%S").to_string(),
                information,
            );
        }

        data_points.insert((&station).to_string(), station_data_points);
    }

    let format: &str = &format;
    match format {
        "json" => match serde_json::to_string_pretty(&Response::new(data_points, 200)) {
            Ok(x) => (Status::Ok, (ContentType::JSON, x)),
            Err(_) => (
                Status::BadRequest,
                (
                    ContentType::JSON,
                    "{ \"status\": \"Failed to serialize data!\", \"status_code\": 400 }"
                        .to_string(),
                ),
            ),
        },
        "text" => {
            let mut output = String::new();
            for (station, data) in data_points {
                output += &(station + ":");
                output += "\n";

                for (date, information) in data {
                    output += &date;
                    output += "=";

                    let properties = vec![
                        information.altitude.to_string(),
                        information.co2_concentration.to_string(),
                        information.humidity.to_string(),
                        information.illuminance.to_string(),
                        information.motion_count.to_string(),
                        information.orientation.to_string(),
                        information.press_count.to_string(),
                        information.sound_level.to_string(),
                        information.temperature.to_string(),
                        information.voc_conc.to_string(),
                        information.voltage.to_string(),
                        information.pressure.to_string(),
                        information.uptime.to_string(),
                    ];

                    for (i, property) in properties.iter().enumerate() {
                        if i != 0 {
                            output += ",";
                        }
                        output += &property;
                    }

                    output += "\n";
                }
            }

            (Status::Ok, (ContentType::Text, output))
        }

        _ => (
            Status::BadRequest,
            (ContentType::Text, "Invalid format!".to_string()),
        ),
    }
}
