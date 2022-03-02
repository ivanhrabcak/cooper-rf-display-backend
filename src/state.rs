use std::io::Write;
use std::{collections::HashMap, path::Path, fs::File, io::Read};
use std::fs::remove_file;

use chrono::{NaiveDateTime, Local};
use serde::{Serialize, Deserialize};

use crate::questions::{Answer, Question};

#[derive(Serialize, Deserialize)]
pub struct State {
    pub start_date: NaiveDateTime,
    pub answers: HashMap<Question, Vec<Answer>>
}

impl Default for State {
    fn default() -> Self {
        let current_date = Local::now().naive_local();
        Self { start_date: current_date, answers: HashMap::new() }
    }
}

pub fn read_state(path: String) -> Result<State, String> {
        if !Path::new(&path).exists() {
            return Err(format!("The path {path} doesn't exist!"));
        }

        let file = match File::open(path) {
            Ok(x) => x,
            Err(e) => return Err(e.to_string())
        };

        let mut json = String::new();
        file.read_to_string(&mut json);

        let state: State = match serde_json::from_str(&json) {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        };

        Ok(state)
}

pub fn write_state(path: String, state: State) -> Result<(), String> {
    if Path::new(&path).exists() {
        match remove_file(path) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string())
        };
    }

    let file = match File::create(path) {
        Ok(x) => x,
        Err(e) => return Err(e.to_string())
    };

    let json = match serde_json::to_string_pretty(&state) {
        Ok(x) => x,
        Err(e) => return Err(e.to_string())
    };

    match file.write(json.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}
