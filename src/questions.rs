use std::{path::Path, fs::{read_dir, File}, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Question {
    pub text: String,
    pub answers: Vec<Answer>
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Answer {
    pub text: String,
    pub is_correct: bool
}

pub fn load_questions(directory: String) -> Result<Vec<Question>, String> {
    if !Path::new(&directory).exists() {
        return Err(format!("The path {directory} doesn't exist!"));
    }
    
    let paths = match read_dir(directory) {
        Ok(x) => x,
        Err(e) => return Err(e.to_string()),
    };

    let mut questions = vec![];

    for path in paths {
        let path = match path {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        }.path();

        let mut file = match File::open(path) {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        };

        let mut json = String::new();
        match file.read_to_string(&mut json) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string())
        };

        let question: Question = match serde_json::from_str(&json) {
            Ok(x) => x,
            Err(e) => return Err(e.to_string())
        };

        questions.push(question);
    }

    Ok(questions)
}