use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Answer {
    firstname: String,
    answers: Vec<AnswerData>,
}

#[derive(Serialize, Deserialize)]
struct AnswerData {
    date: u64,
    checked: bool,
}