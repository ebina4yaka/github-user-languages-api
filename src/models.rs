use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ToDo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub done: bool,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LanguagePercentage {
    pub name: String,
    pub percentage: i32,
}

pub struct LanguageSize {
    pub name: String,
    pub size: i64,
}
