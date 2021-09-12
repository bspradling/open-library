use crate::models::{Identifier, OpenLibraryIdentifierKey, Resource};
use crate::OpenLibraryError;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Book {
    pub url: Url,
    pub key: Identifier<Resource>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    pub pagination: Option<String>,
    pub by_statement: Option<String>,
    pub notes: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<Entity>,
    #[serde(default)]
    pub identifiers: HashMap<BookIdentifier, Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Classifications::is_default")]
    pub classifications: Classifications,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<Entity>,
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub publishers: Vec<Entity>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub publish_places: Vec<Entity>,
    pub publish_date: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub excerpts: Vec<Excerpt>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<String>,
    #[serde(default)]
    #[serde(rename = "covers")]
    pub cover_images: CoverImages,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ebooks: Vec<ElectronicBook>,
    pub number_of_pages: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum BibliographyKey {
    ISBN(String),
    LCCN(String),
    OCLC(String),
    OLID(String),
}

impl BibliographyKey {
    pub fn from((key, value): (String, String)) -> Result<Self, OpenLibraryError> {
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

impl<'de> Deserialize<'de> for BibliographyKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = Deserialize::deserialize(deserializer).map_err(D::Error::custom)?;
        let chunks: Vec<&str> = value.split(':').collect();

        if chunks.len() != 2 {
            return Err(D::Error::custom("The specified value {} has improper form"));
        }

        let key = match chunks.get(0) {
            Some(string) => Ok(String::from(*string)),
            None => Err(D::Error::custom(format!(
                "Supplied identifier string has improper format {}",
                &value
            ))),
        }?;

        let value = match chunks.get(1) {
            Some(string) => Ok(String::from(*string)),
            None => Err(D::Error::custom(format!(
                "Supplied identifier string has improper format {}",
                &value
            ))),
        }?;

        BibliographyKey::from((key, value)).map_err(D::Error::custom)
    }
}

impl Serialize for BibliographyKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl From<Identifier<BookIdentifier>> for Result<BibliographyKey, OpenLibraryError> {
    fn from(identifier: Identifier<BookIdentifier>) -> Self {
        match identifier.resource {
            BookIdentifier::InternationalStandard10 => {
                Ok(BibliographyKey::ISBN(identifier.identifier))
            }
            BookIdentifier::InternationalStandard13 => {
                Ok(BibliographyKey::ISBN(identifier.identifier))
            }
            BookIdentifier::LibraryOfCongress => Ok(BibliographyKey::LCCN(identifier.identifier)),
            BookIdentifier::OhioCollegeLibraryCenter => {
                Ok(BibliographyKey::OCLC(identifier.identifier))
            }
            BookIdentifier::OpenLibrary => Ok(BibliographyKey::OLID(identifier.identifier)),
            _ => Err(OpenLibraryError::ParsingError {
                reason: format!(
                    "The identifier specified ({}) is not supported as a bibliogrpahy key!",
                    identifier.resource
                ),
            }),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum BookIdentifier {
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

impl OpenLibraryIdentifierKey for BookIdentifier {}

impl Display for BookIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BookIdentifier::InternationalStandard10 => Ok(write!(f, "isbn_10")?),
            BookIdentifier::InternationalStandard13 => Ok(write!(f, "isbn_13")?),
            BookIdentifier::LibraryOfCongress => Ok(write!(f, "lccn")?),
            BookIdentifier::OhioCollegeLibraryCenter => Ok(write!(f, "oclc")?),
            BookIdentifier::GoodReads => Ok(write!(f, "goodreads")?),
            BookIdentifier::OpenLibrary => Ok(write!(f, "openlibrary")?),
            BookIdentifier::LibraryThing => Ok(write!(f, "librarything")?),
            BookIdentifier::ProjectGutenberg => Ok(write!(f, "project_gutenberg")?),
            BookIdentifier::WikiData => Ok(write!(f, "wikidata")?),
        }
    }
}

impl FromStr for BookIdentifier {
    type Err = OpenLibraryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "isbn_10" => Ok(BookIdentifier::InternationalStandard10),
            "isbn_13" => Ok(BookIdentifier::InternationalStandard13),
            "lccn" => Ok(BookIdentifier::LibraryOfCongress),
            "oclc" => Ok(BookIdentifier::OhioCollegeLibraryCenter),
            "goodreads" => Ok(BookIdentifier::GoodReads),
            "openlibrary" => Ok(BookIdentifier::OpenLibrary),
            "librarything" => Ok(BookIdentifier::LibraryThing),
            "project_gutenberg" => Ok(BookIdentifier::ProjectGutenberg),
            "wikidata" => Ok(BookIdentifier::WikiData),
            _ => Err(OpenLibraryError::ParsingError {
                reason: format!(
                    "Unable to parse supplied value ({}) into a book identifier!",
                    s
                ),
            }),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Author {
    key: Identifier<Resource>,
    name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entity {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CoverImages {
    pub small: Option<Url>,
    pub medium: Option<Url>,
    pub large: Option<Url>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ElectronicBook {
    preview_url: String,
    availability: String, //Should be enum but don't know possible values ("restricted",...)
                          // formats: ?  //Don't know the form of the struct yet
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Excerpt {
    comment: String,
    text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Link {
    #[serde(skip_serializing_if = "String::is_empty")]
    url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    title: String,
}
