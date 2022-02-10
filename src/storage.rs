use std::env::join_paths;
use std::path::Path;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct Storage {
    pub directory: String,
}

impl Storage {
    pub fn new(directory: String) -> Self {
        Self { directory }
    }

    pub async fn write_file(&mut self, filename: String, contents: String) -> Result<(), String> {
        let new_file_path = match join_paths([Path::new(&self.directory), Path::new(&filename)]) {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        };

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
