use crate::models::books::{Author, BookIdentifierKey, Classifications};
use crate::models::identifiers::InternationalStandardBookNumber;
use crate::models::{OpenLibraryModel, OpenLibraryResource};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug, Eq, PartialEq, Serialize)]
pub struct Work {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub publishers: Vec<String>,
    pub number_of_pages: u32,
    #[serde(rename = "isbn_10")]
    pub isbns_10: Vec<InternationalStandardBookNumber>,
    pub covers: Vec<u32>, //TODO Cover Id
    pub key: OpenLibraryResource,
    pub authors: Vec<Author>,
    pub ocaid: String,
    pub contributions: Vec<String>,
    #[serde(with = "crate::format::keyed_list")]
    pub languages: Vec<String>,
    pub classifications: Classifications,
    pub source_records: Vec<String>, //TODO Parse these?
    pub title: String,
    #[serde(default)]
    pub identifiers: HashMap<BookIdentifierKey, Vec<String>>,
    #[serde(rename = "isbn_13")]
    pub isbns_13: Vec<InternationalStandardBookNumber>,
    pub local_id: Vec<String>, //TODO Parse?
    #[serde(with = "crate::format::date_m_dd_yyyy")]
    pub publish_date: NaiveDate,
    #[serde(with = "crate::format::keyed_list")]
    pub works: Vec<OpenLibraryResource>,
    #[serde(rename = "type")]
    #[serde(with = "crate::format::keyed_value")]
    pub works_type: String,
    pub first_sentence: FirstSentence,
    pub latest_revision: u32,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Serialize)]
pub struct FirstSentence {
    #[serde(rename = "type")]
    pub sentence_type: String,
    pub value: String,
}

impl OpenLibraryModel for Work {}
