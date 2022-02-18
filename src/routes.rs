use rocket::{routes, Route};

use crate::api::dashboard::get_storage_info;
use crate::api::data::get_data_from_date;
use crate::api::data::get_data_points_for_date;
use crate::api::data::get_dates_with_data;
use crate::api::data::get_stations;
use crate::api::edupage::get_next_lesson;
use crate::api::edupage::get_substitution;

pub fn get_routes() -> Vec<Route> {
    return routes![
        get_data_points_for_date,
        get_dates_with_data,
        get_data_from_date,
        get_substitution,
        get_stations,
        get_next_lesson,
        get_storage_info
    ];
}
