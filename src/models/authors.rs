use crate::models::OpenLibraryModel;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Author {
    pub key: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub text: Vec<String>,
    #[serde(rename(deserialize = "type"))]
    pub r#type: String,
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_names: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    pub top_work: String,
    pub work_count: i32,
    pub top_subjects: Vec<String>,
    pub _version_: u64,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuthorDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source_records: Vec<String>, //TODO parse records
    pub key: Identifier<Resource>,
    #[serde(default)]
    #[serde(with = "crate::format::value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    pub photos: Vec<i32>,
    // #[serde(with = "crate::format::date")]
    pub birth_date: String,
    pub personal_name: String,
    pub remote_ids: HashMap<String, String>,
    pub entity_type: Option<String>, //TODO: should be enum
    pub links: Vec<Link>,
    pub name: String,
    pub alternate_names: Vec<String>,
    pub wikipedia: Option<Url>,
    pub latest_revision: u16,
    pub revision: u16,
    #[serde(with = "crate::format::value")]
    pub created: Option<NaiveDateTime>,
    #[serde(with = "crate::format::value")]
    pub last_modified: Option<NaiveDateTime>,
}

impl OpenLibraryModel for AuthorDetails {}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorResponse {
    #[serde(rename = "numFound")]
    pub num_found: i32,
    pub start: i32,
    #[serde(rename = "numFoundExact")]
    pub num_found_exact: bool,
    pub docs: Vec<Author>,
}

impl OpenLibraryModel for AuthorResponse {}
