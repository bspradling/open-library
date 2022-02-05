use crate::models::identifiers::OpenLibraryIdentifer;
use crate::models::{Link, OpenLibraryModel, Resource};
use crate::OpenLibraryError;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;
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
    pub key: Resource,
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

#[derive(Deserialize, Debug, Eq, PartialEq, Serialize)]
pub struct AuthorWorksRequest {
    pub identifier: OpenLibraryIdentifer,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl TryFrom<OpenLibraryIdentifer> for AuthorWorksRequest {
    type Error = OpenLibraryError;

    fn try_from(identifier: OpenLibraryIdentifer) -> Result<Self, OpenLibraryError> {
        Ok(Self {
            identifier,
            limit: None,
            offset: None,
        })
    }
}

impl TryFrom<Url> for AuthorWorksRequest {
    type Error = OpenLibraryError;

    fn try_from(value: Url) -> Result<Self, Self::Error> {
        let path_segments = value
            .path_segments()
            .ok_or(OpenLibraryError::ParsingError {
                reason: "Invalid URL supplied, no path segments found".to_string(),
            })?
            .collect::<Vec<&str>>();

        let path_index = path_segments.iter().position(|x| *x == "authors").ok_or(
            OpenLibraryError::ParsingError {
                reason: "Invalid URL supplied, unable to determine author identifier".to_string(),
            },
        )?;

        let query_parameters = value
            .query_pairs()
            .collect::<HashMap<Cow<'_, str>, Cow<'_, str>>>();

        let result = *path_segments
            .get(path_index + 1)
            .ok_or(OpenLibraryError::ParsingError {
                reason: "Unable to find an author identifier within the URL path".to_string(),
            })?;

        let limit = match query_parameters.get("limit") {
            Some(x) => Some(x.clone().into_owned().parse::<u32>().map_err(|e| {
                OpenLibraryError::ParsingError {
                    reason: e.to_string(),
                }
            })?),
            None => None,
        };

        let offset = match query_parameters.get("offset") {
            Some(z) => Some(z.clone().into_owned().parse::<u32>().map_err(|e| {
                OpenLibraryError::ParsingError {
                    reason: e.to_string(),
                }
            })?),
            None => None,
        };

        Ok(Self {
            identifier: OpenLibraryIdentifer::from_str(result)?,
            limit: limit,
            offset: offset,
        })
    }
}

#[derive(Deserialize, Debug, Eq, PartialEq, Serialize)]
pub struct AuthorWorksResponse {
    //TODO: add links dictionary
    pub size: u32,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<AuthorWorks>,
}

impl OpenLibraryModel for AuthorWorksResponse {}

#[derive(Deserialize, Debug, Eq, PartialEq, Serialize)]
pub struct AuthorWorks {
    // pub description: Option<String>, TODO: this can be a map or a string
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
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_people: Vec<String>,
    // pub key: Resource,
    // pub authors:
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_times: Vec<String>,
    pub latest_revision: u32,
    pub revision: u32,
    // pub created:
    // pub last_modified
}

impl OpenLibraryModel for AuthorWorks {}

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
