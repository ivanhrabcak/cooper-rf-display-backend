use std::fs::read_dir;
use std::path::{Path, PathBuf};

use tokio::fs::{create_dir, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::information::Information;

pub struct Storage {
    pub directory: String,
}

impl Storage {
    pub fn new(directory: String) -> Self {
        Self { directory }
    }

    pub async fn write_file(
        &self,
        device_id: String,
        filename: String,
        contents: String,
    ) -> Result<(), String> {
        if !Path::new(&self.directory).exists() {
            match create_dir(&self.directory).await {
                Ok(_) => (),
                Err(e) => {
                    return Err(format!(
                        "Failed to create directory {}! {}",
                        self.directory,
                        e.to_string()
                    ))
                }
            };
        }

        let id_path = Path::new(&self.directory).join(Path::new(&device_id));
        if !id_path.exists() {
            match create_dir(&id_path).await {
                Ok(_) => (),
                Err(e) => {
                    return Err(format!(
                        "Failed to create directory {}! {}",
                        id_path.display().to_string(),
                        e.to_string()
                    ))
                }
            };
        }

        let new_file_path = Path::new(&id_path).join(Path::new(&filename));

        let file = File::create(new_file_path).await;
        if file.is_err() {
            return Err(format!("Failed to create file {}", file.unwrap_err()));
        }

        let mut file = file.unwrap();

        match file.write(contents.as_bytes()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn get_files_for_station(&self, station_id: &String) -> Result<Vec<String>, String> {
        let station_files_path = Path::new(&self.directory).join(Path::new(&station_id));

        if !station_files_path.exists() {
            return Err(format!("No data for station with id {}", station_id));
        }

        let paths = match read_dir(&station_files_path) {
            Ok(x) => x,
            Err(e) => {
                return Err(format!(
                    "Error listing directory {}! {}",
                    station_files_path.to_str().unwrap(),
                    e.to_string()
                ))
            }
        };

        let mut files = Vec::new();

        for path in paths {
            files.push(path.unwrap().file_name().to_str().unwrap().to_string());
        }

        Ok(files)
    }

    pub async fn read_information(
        &self,
        station_id: &String,
        timestamp: i64,
    ) -> Result<Information, String> {
        let timestamp = timestamp.to_string();

        let station_directory = Path::new(&self.directory).join(station_id);
        if !station_directory.exists() {
            return Err(format!(
                "{} doesn't exist!",
                station_directory.to_str().unwrap()
            ));
        }

        let paths = match read_dir(&station_directory) {
            Ok(x) => x,
            Err(e) => {
                return Err(format!(
                    "Error listing directory {}! {}",
                    station_directory.to_str().unwrap(),
                    e.to_string()
                ))
            }
        };

        let searching_for_path = format!("{timestamp}.json");

        let mut path: Option<PathBuf> = None;
        for path_entry in paths {
            let path_entry = path_entry.unwrap();
            if path_entry.file_name().to_str().unwrap() == searching_for_path {
                path = Some(path_entry.path());
                break;
            }
        }

        if path.is_none() {
            return Err("Missing data!".to_string());
        }

        let path = path.unwrap();
        let mut file = match File::open(path).await {
            Ok(x) => x,
            Err(e) => return Err(format!("Failed to open file! {}", e.to_string())),
        };

        let mut json = String::new();

        match file.read_to_string(&mut json).await {
            Ok(_) => (),
            Err(e) => return Err(format!("Failed to read file! {}", e.to_string())),
        };

        let information: Information = match serde_json::from_str(&json) {
            Ok(x) => x,
            Err(e) => {
                return Err(format!(
                    "Failed to deserialize information! {}",
                    e.to_string()
                ))
            }
        };

        Ok(information)
    }
}
