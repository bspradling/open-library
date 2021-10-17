use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    pub key: String,
    pub text: Vec<String>,
    #[serde(rename(deserialize = "type"))]
    pub r#type: String,
    pub name: String,
    pub alternate_names: Vec<String>,
    #[serde(with = "crate::format::date")]
    pub birth_date: NaiveDate,
    pub top_work: String,
    pub work_count: i32,
    pub top_subjects: Vec<String>,
    pub _version_: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorResponse {
    #[serde(rename = "numFound")]
    pub num_found: i32,
    pub start: i32,
    #[serde(rename = "numFoundExact")]
    pub num_found_exact: bool,
    pub docs: Vec<Author>,
}
