use crate::format::KeyedValue;
use crate::models::identifiers::OpenLibraryIdentifier;
use crate::models::works::Work;
use crate::models::{Link, LinkName, OpenLibraryModel, OpenLibraryResource};
use crate::OpenLibraryError;
use chrono::NaiveDateTime;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Display;
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
pub struct AuthorReference {
    #[serde(rename = "type")]
    #[serde(deserialize_with = "deserialize_author_type")]
    pub author_type: KeyedValue<AuthorType>,
    #[serde(rename = "author")]
    pub identifier: KeyedValue<OpenLibraryResource>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum AuthorType {
    AuthorRole,
}

impl Display for AuthorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuthorType::AuthorRole => write!(f, "/type/author_role"),
        }
    }
}

impl FromStr for AuthorType {
    type Err = OpenLibraryError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "author_role" => Ok(Self::AuthorRole),
            _ => Err(OpenLibraryError::ParsingError {
                reason: format!("Unable to parse string ({}) into an Author Type", &value),
            }),
        }
    }
}

impl<'de> Deserialize<'de> for AuthorType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = Deserialize::deserialize(deserializer).map_err(D::Error::custom)?;

        let chunks = value
            .split('/')
            .filter(|str| !str.is_empty())
            .collect::<Vec<&str>>();

        match chunks.get(0) {
            Some(&"type") => match chunks.get(1) {
                Some(value) => Ok(AuthorType::from_str(*value).map_err(D::Error::custom)?),
                None => Err(D::Error::custom("No Author Type was provided!")),
            },
            _ => Err(D::Error::custom(format!(
                "Invalid format for Author Type: {}",
                &value
            ))),
        }
    }
}

impl Serialize for AuthorType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuthorDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source_records: Vec<String>, //TODO parse records
    pub key: OpenLibraryResource,
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
    pub identifier: OpenLibraryIdentifier,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl TryFrom<OpenLibraryIdentifier> for AuthorWorksRequest {
    type Error = OpenLibraryError;

    fn try_from(identifier: OpenLibraryIdentifier) -> Result<Self, OpenLibraryError> {
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
            identifier: OpenLibraryIdentifier::from_str(result)?,
            limit: limit,
            offset: offset,
        })
    }
}

#[derive(Deserialize, Debug, Eq, PartialEq, Serialize)]
pub struct AuthorWorksResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub links: HashMap<LinkName, String>,
    pub size: u32,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<Work>,
}

impl OpenLibraryModel for AuthorWorksResponse {}

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

fn deserialize_author_type<'de, D>(deserializer: D) -> Result<KeyedValue<AuthorType>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrKeyedValue {
        String(AuthorType),
        KeyedValue(KeyedValue<AuthorType>),
    }

    Ok(match StringOrKeyedValue::deserialize(deserializer)? {
        StringOrKeyedValue::String(v) => KeyedValue { key: v },
        StringOrKeyedValue::KeyedValue(v) => v,
    })
}
