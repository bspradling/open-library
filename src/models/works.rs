use crate::format::KeyedValue;
use crate::models::authors::{AuthorReference, AuthorType};
use crate::models::{OpenLibraryModel, OpenLibraryResource};
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize};

/// Represents a logical collection of similar Editions.
// The fields present per Work varies by instance so to better understand the distribution a key
// frequency distribution was created with 10 million records. This client won't support anything
// over 20% until a reason to do so presents itself. For a detailed view of field frequencies, view
// `models` directory README.
#[derive(Deserialize, Debug, Eq, PartialEq, Serialize)]
pub struct Work {
    pub title: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub covers: Vec<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_places: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<String>,
    pub key: OpenLibraryResource,
    pub authors: Vec<AuthorReference>,
    pub latest_revision: u32,
    pub revision: u32,
    // pub created: TODO need to support
    // pub last_modified TODO need to support
}

impl OpenLibraryModel for Work {}
