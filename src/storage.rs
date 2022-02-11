use std::path::Path;

use tokio::fs::{create_dir, File};
use tokio::io::AsyncWriteExt;

pub struct Storage {
    pub directory: String,
}

impl Storage {
    pub fn new(directory: String) -> Self {
        Self { directory }
    }

    pub async fn write_file(
        &mut self,
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
}
