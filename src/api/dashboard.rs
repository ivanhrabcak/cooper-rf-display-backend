use fs_extra::dir::get_size;
use rocket::{get, State};

use crate::config::Config;

use super::response::Response;

#[get("/api/dashboard/storage")]
pub async fn get_storage_info(config: &State<Config>) -> Response<u64> {
    let storage_path = (&config).save_directory.clone();

    match get_size(storage_path) {
        Ok(x) => Response::new(x, 200),
        Err(_) => Response::new(0, 500),
    }
}
