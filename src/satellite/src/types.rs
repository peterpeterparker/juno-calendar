use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EventsData {
    pub title: String,
    pub dates: Vec<u64>,
    pub count_answers: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct AnswersData {
    pub firstname: String,
    pub answers: Vec<AnswerData>,
}

#[derive(Serialize, Deserialize)]
pub struct AnswerData {
    pub date: u64,
    pub checked: bool,
}

#[derive(Serialize, Deserialize)]
pub struct EnvData {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct SettingData {
    pub email: String,
}
