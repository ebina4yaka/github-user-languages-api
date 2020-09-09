use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguagePercentage {
    pub name: String,
    pub percentage: f64,
}

#[derive(Debug)]
pub struct LanguageSize {
    pub name: String,
    pub size: i64,
}
