use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguagePercentage {
    pub name: String,
    pub color: String,
    pub percentage: f64,
}

#[derive(Debug)]
pub struct LanguageSize {
    pub name: String,
    pub color: String,
    pub size: i64,
}
