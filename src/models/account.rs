use crate::models::{Identifier, Resource};
use crate::OpenLibraryErrorResponse;
use chrono::{DateTime, Utc};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub enum ReadingLogResponseWrapper {
    Success(ReadingLogResponse),
    Err(OpenLibraryErrorResponse),
}

impl<'de> Deserialize<'de> for ReadingLogResponseWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json: serde_json::Value = Deserialize::deserialize(deserializer)?;

        match json.get("error") {
            None => Ok(ReadingLogResponseWrapper::Success(
                serde_json::from_value(json).map_err(D::Error::custom)?,
            )),
            Some(_) => Ok(ReadingLogResponseWrapper::Err(
                serde_json::from_value(json).map_err(D::Error::custom)?,
            )),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Session {
    cookie: String,
    username: String,
}

impl Session {
    pub fn from(cookie: String, username: String) -> Self {
        Session { cookie, username }
    }

    pub fn cookie(&self) -> &String {
        &self.cookie
    }

    pub fn username(&self) -> &String {
        &self.username
    }
}

#[derive(Deserialize, Serialize)]
pub struct ReadingLogResponse {
    pub page: i16,
    pub reading_log_entries: Vec<ReadingLogEntry>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ReadingLogEntry {
    pub work: ReadingLogWork,
    pub logged_edition: Identifier<Resource>,
    #[serde(with = "crate::format")]
    pub logged_date: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ReadingLogWork {
    pub title: String,
    pub key: Identifier<Resource>,
    pub author_keys: Vec<Identifier<Resource>>,
    pub author_names: Vec<String>,
    pub first_publish_year: Option<i32>,
    pub lending_edition_s: Option<String>,
    pub edition_key: Vec<String>,
    pub cover_id: Option<i32>,
    pub cover_edition_key: Option<String>,
}
