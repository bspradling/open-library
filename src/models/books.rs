use crate::models::{OpenLibraryIdentifierKey, OpenLibraryModel, Resource};
use crate::OpenLibraryError;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use url::Url;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Book {
    #[serde(default)]
    #[serde(deserialize_with = "url_or_list")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub url: Vec<Url>,
    pub key: Resource,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    pub pagination: Option<String>,
    pub by_statement: Option<String>,
    pub notes: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<Author>,
    #[serde(default)]
    pub identifiers: HashMap<BookIdentifierKey, Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Classifications::is_default")]
    pub classifications: Classifications,
    #[serde(default)]
    #[serde(deserialize_with = "strings_or_entities")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_places: Vec<Entity>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_people: Vec<Entity>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_times: Vec<Entity>,
    #[serde(default)]
    #[serde(deserialize_with = "strings_or_entities")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub publishers: Vec<String>,
    #[serde(default)]
    #[serde(deserialize_with = "strings_or_entities")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub publish_places: Vec<String>,
    pub publish_date: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub excerpts: Vec<Excerpt>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    #[serde(default)]
    #[serde(rename = "covers")]
    pub cover_images: Vec<u32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ebooks: Vec<ElectronicBook>,
    pub number_of_pages: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
}

impl OpenLibraryModel for Book {}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum BibliographyKey {
    ISBN(String),
    LCCN(String),
    OCLC(String),
    OLID(String),
}

impl BibliographyKey {
    pub fn from_tuple((key, value): (String, String)) -> Result<Self, OpenLibraryError> {
        match key.as_str() {
            "ISBN" => Ok(BibliographyKey::ISBN(value)),
            "LCCN" => Ok(BibliographyKey::LCCN(value)),
            "OCLC" => Ok(BibliographyKey::OCLC(value)),
            "OLID" => Ok(BibliographyKey::OLID(value)),
            _ => Err(OpenLibraryError::ParsingError {
                reason: format!(
                    "Unable to determine bibliography key from specific value ({})",
                    value
                ),
            }),
        }
    }
}

impl Display for BibliographyKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BibliographyKey::ISBN(value) => write!(f, "ISBN:{}", value)?,
            BibliographyKey::LCCN(value) => write!(f, "LCCN:{}", value)?,
            BibliographyKey::OCLC(value) => write!(f, "OCLC:{}", value)?,
            BibliographyKey::OLID(value) => write!(f, "OLID:{}", value)?,
        }
        Ok(())
    }
}

impl<'de> serde::Deserialize<'de> for BibliographyKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String =
            serde::Deserialize::deserialize(deserializer).map_err(D::Error::custom)?;
        let chunks: Vec<&str> = value.split(':').collect();

        if chunks.len() != 2 {
            return Err(D::Error::custom("The specified value {} has improper form"));
        }

        let key = match chunks.get(0) {
            Some(value) => Ok(*value),
            None => Err(D::Error::custom(format!(
                "Supplied identifier string has improper format {}",
                &value
            ))),
        }?
        .to_string();

        let value = match chunks.get(1) {
            Some(string) => Ok(*string),
            None => Err(D::Error::custom(format!(
                "Supplied identifier string has improper format {}",
                &value
            ))),
        }?
        .to_string();

        BibliographyKey::from_tuple((key, value)).map_err(|error| D::Error::custom(error))
    }
}

impl serde::Serialize for BibliographyKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(format!("{}", self).as_str())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum BookIdentifierKey {
    #[serde(alias = "isbn_10")]
    InternationalStandard10, // International Standard Book Number - 10 Digits
    #[serde(alias = "isbn_13")]
    InternationalStandard13, // International Standard Book Number - 13 Digits
    #[serde(alias = "lccn")]
    LibraryOfCongress, // Library of Congress Control Number
    #[serde(alias = "oclc")]
    OhioCollegeLibraryCenter, // Ohio College Library Center https://en.wikipedia.org/wiki/OCLC
    #[serde(alias = "goodreads")]
    GoodReads,
    #[serde(alias = "openlibrary")]
    OpenLibrary,
    #[serde(alias = "librarything")]
    LibraryThing,
    #[serde(alias = "project_gutenberg")]
    ProjectGutenberg,
    #[serde(alias = "wikidata")]
    WikiData,
}

impl BookIdentifierKey {
    pub fn from_isbn(value: &str) -> Result<Self, OpenLibraryError> {
        match value.len() {
            10 => Ok(BookIdentifierKey::InternationalStandard10),
            13 => Ok(BookIdentifierKey::InternationalStandard13),
            _ => Err(OpenLibraryError::ParsingError {
                reason: format!("Invalid length ({}) for ISBN", value.len()),
            }),
        }
    }
}

impl OpenLibraryIdentifierKey for BookIdentifierKey {}

impl Display for BookIdentifierKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BookIdentifierKey::InternationalStandard10 => Ok(write!(f, "isbn_10")?),
            BookIdentifierKey::InternationalStandard13 => Ok(write!(f, "isbn_13")?),
            BookIdentifierKey::LibraryOfCongress => Ok(write!(f, "lccn")?),
            BookIdentifierKey::OhioCollegeLibraryCenter => Ok(write!(f, "oclc")?),
            BookIdentifierKey::GoodReads => Ok(write!(f, "goodreads")?),
            BookIdentifierKey::OpenLibrary => Ok(write!(f, "openlibrary")?),
            BookIdentifierKey::LibraryThing => Ok(write!(f, "librarything")?),
            BookIdentifierKey::ProjectGutenberg => Ok(write!(f, "project_gutenberg")?),
            BookIdentifierKey::WikiData => Ok(write!(f, "wikidata")?),
        }
    }
}

impl FromStr for BookIdentifierKey {
    type Err = OpenLibraryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "isbn_10" => Ok(BookIdentifierKey::InternationalStandard10),
            "isbn_13" => Ok(BookIdentifierKey::InternationalStandard13),
            "lccn" => Ok(BookIdentifierKey::LibraryOfCongress),
            "oclc" => Ok(BookIdentifierKey::OhioCollegeLibraryCenter),
            "goodreads" => Ok(BookIdentifierKey::GoodReads),
            "openlibrary" => Ok(BookIdentifierKey::OpenLibrary),
            "librarything" => Ok(BookIdentifierKey::LibraryThing),
            "project_gutenberg" => Ok(BookIdentifierKey::ProjectGutenberg),
            "wikidata" => Ok(BookIdentifierKey::WikiData),
            _ => Err(OpenLibraryError::ParsingError {
                reason: format!(
                    "Unable to parse supplied value ({}) into a book identifier!",
                    s
                ),
            }),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Author {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<Resource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Entity {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Classifications {
    #[serde(default)]
    #[serde(rename(deserialize = "dewey_decimal_class"))]
    pub dewey_decimal: Vec<String>,
    #[serde(default)]
    #[serde(rename(deserialize = "lc_classifications"))]
    pub library_of_congress: Vec<String>,
}

impl Classifications {
    pub fn is_default(&self) -> bool {
        self.dewey_decimal.is_empty() && self.library_of_congress.is_empty()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ElectronicBook {
    preview_url: String,
    availability: String, //Should be enum but don't know possible values ("restricted",...)
                          // formats: ?  //Don't know the form of the struct yet
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Excerpt {
    comment: String,
    text: String,
}

// Necessary since the `publishers` field can either be Vec<String> or Vec<Entity> based on the endpoint
// Book Search:
//    "publishers": [
//       {
//         "name": "Anchor Books"
//       }
//     ]
// By ISBN:
//    "publishers": [
//      "Addison-Wesley"
//     ]
fn strings_or_entities<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StrOrEntity {
        Str(Vec<String>),
        Entity(Vec<Entity>),
    }

    Ok(match StrOrEntity::deserialize(deserializer)? {
        StrOrEntity::Str(v) => v,
        StrOrEntity::Entity(v) => v.into_iter().map(|x| x.name).collect(),
    })
}

// Necessary since the `url` field can either be a String or Vec<Url> based on the endpoint
// Book Search:
//    "publishers": [
//       {
//         "name": "Anchor Books"
//       }
//     ]
// By ISBN:
//    "publishers": [
//      "Addison-Wesley"
//     ]
fn url_or_list<'de, D>(deserializer: D) -> Result<Vec<Url>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum UrlOrVector {
        Url(Url),
        Vector(Vec<Url>),
    }

    Ok(match UrlOrVector::deserialize(deserializer)? {
        UrlOrVector::Url(value) => vec![value],
        UrlOrVector::Vector(values) => values,
    })
}
