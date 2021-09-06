use crate::models::Identifier;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Deserialize)]
pub struct Session {
    pub cookie: String,
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct ReadingLogResponse {
    pub page: i16,
    pub reading_log_entries: Vec<ReadingLogEntry>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ReadingLogEntry {
    pub work: ReadingLogWork,
    pub logged_edition: Identifier,
    #[serde(with = "crate::format")]
    pub logged_date: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ReadingLogWork {
    pub title: String,
    pub key: Identifier,
    pub author_keys: Vec<Identifier>,
    pub author_names: Vec<String>,
    pub first_publish_year: Option<i32>,
    pub lending_edition_s: Option<String>,
    pub edition_key: Vec<String>,
    pub cover_id: Option<i32>,
    pub cover_edition_key: String,
}
