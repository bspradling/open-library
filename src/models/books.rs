use crate::OpenLibraryError;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Book {
    pub url: Url,
    pub title: String,
    pub subtitle: Option<String>,
    #[serde(default)]
    pub authors: Vec<Entity>,
    pub identifiers: HashMap<BookIdentifierKey, Vec<String>>,
    pub classifications: Classifications,
    #[serde(default)]
    pub subjects: Vec<Entity>,
    #[serde(default)]
    pub subject_places: Vec<Entity>,
    #[serde(default)]
    pub subject_people: Vec<Entity>,
    #[serde(default)]
    pub subject_times: Vec<Entity>,
    pub publishers: Vec<Entity>,
    #[serde(default)]
    pub publish_places: Vec<Entity>,
    pub publish_date: String,
    #[serde(default)]
    pub excerpts: Vec<Excerpt>,
    #[serde(default)]
    pub links: Vec<String>,
    #[serde(rename(deserialize = "cover"))]
    pub cover_images: CoverImages,
    #[serde(default)]
    pub ebooks: Vec<ElectronicBook>,
    pub number_of_pages: i32,
    pub weight: Option<String>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BookIdentifier {
    pub key: BookIdentifierKey,
    pub identifier: String,
}

impl BookIdentifier {
    pub fn from(key: BookIdentifierKey, value: String) -> Self {
        Self {
            key,
            identifier: value,
        }
    }
}

impl<'de> Deserialize<'de> for BookIdentifier {
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
            Some(string) => Ok(*string),
            None => Err(D::Error::custom(format!(
                "Supplied identifier string has improper format {}",
                &value
            ))),
        }?;

        let value = match chunks.get(1) {
            Some(string) => Ok(*string),
            None => Err(D::Error::custom(format!(
                "Supplied identifier string has improper format {}",
                &value
            ))),
        }?;

        let identifier = BookIdentifier {
            key: BookIdentifierKey::try_from(key).map_err(D::Error::custom)?,
            identifier: value.to_string(),
        };

        Ok(identifier)
    }
}

impl Display for BookIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.key.to_string(), self.identifier)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum BookIdentifierKey {
    #[serde(alias = "isbn_10")]
    #[serde(alias = "isbn_13")]
    ISBN, // International Standard Book Number
    #[serde(alias = "lccn")]
    LCCN, // Library of Congress Control Number
    #[serde(alias = "oclc")]
    OCLC, // Ohio College Library Center https://en.wikipedia.org/wiki/OCLC
    #[serde(alias = "goodreads")]
    GoodReads,
    #[serde(alias = "openlibrary")]
    OpenLibrary,
    #[serde(alias = "librarything")]
    LibraryThing, //changes necessary here
    #[serde(alias = "project_gutenberg")]
    ProjectGutenberg,
}

impl Display for BookIdentifierKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BookIdentifierKey::ISBN => write!(f, "ISBN")?,
            BookIdentifierKey::LCCN => write!(f, "LCCN")?,
            BookIdentifierKey::OCLC => write!(f, "OCLC")?,
            BookIdentifierKey::GoodReads => write!(f, "GRID")?,
            BookIdentifierKey::OpenLibrary => write!(f, "OLID")?,
            BookIdentifierKey::LibraryThing => write!(f, "LTID")?,
            BookIdentifierKey::ProjectGutenberg => write!(f, "PGID")?,
        }
        Ok(())
    }
}

impl TryFrom<&str> for BookIdentifierKey {
    type Error = OpenLibraryError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ISBN" | "isbn_10" | "isbn_13" => Ok(BookIdentifierKey::ISBN),
            "LCCN" => Ok(BookIdentifierKey::LCCN),
            "OCLC" => Ok(BookIdentifierKey::OCLC),
            "OLID" => Ok(BookIdentifierKey::OpenLibrary),
            "GRID" => Ok(BookIdentifierKey::GoodReads),
            "LTID" => Ok(BookIdentifierKey::LibraryThing),
            "PGID" => Ok(BookIdentifierKey::ProjectGutenberg),
            _ => Err(OpenLibraryError::ParsingError {
                reason: format!(
                    "Unable to determine Book Identifier Key from specified value {}",
                    value
                ),
            }),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Entity {
    name: String,
    url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CoverImages {
    small: Url,
    medium: Url,
    large: Url,
}

#[derive(Debug, Deserialize)]
pub struct Classifications {
    #[serde(default)]
    #[serde(rename(deserialize = "dewey_decimal_class"))]
    dewey_decimal: Vec<String>,
    #[serde(default)]
    #[serde(rename(deserialize = "lc_classifications"))]
    library_of_congress: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ElectronicBook {
    preview_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Excerpt {
    comment: String,
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct Link {
    url: String,
    title: String,
}
