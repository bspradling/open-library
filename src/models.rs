use crate::models::identifiers::OpenLibraryIdentifier;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub mod account;
pub mod authors;
pub mod books;
pub mod identifiers;
pub mod works;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OpenLibraryResource {
    Author(String),
    Book(String),
    Work(String),
}

impl OpenLibraryResource {
    pub fn value(&self) -> String {
        match self {
            OpenLibraryResource::Author(value) => value,
            OpenLibraryResource::Book(value) => value,
            OpenLibraryResource::Work(value) => value,
        }
        .clone()
    }
}

impl Display for OpenLibraryResource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            OpenLibraryResource::Author(value) => format!("/authors/{}", value),
            OpenLibraryResource::Book(value) => format!("/books/{}", value),
            OpenLibraryResource::Work(value) => format!("/works/{}", value),
        };

        write!(f, "{}", x)?;
        Ok(())
    }
}

impl<'de> Deserialize<'de> for OpenLibraryResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = Deserialize::deserialize(deserializer).map_err(D::Error::custom)?;

        let chunks = value
            .split('/')
            .filter(|str| !str.is_empty())
            .collect::<Vec<&str>>();

        let resource = match chunks.get(0) {
            Some(string) => Ok(*string),
            None => Err(D::Error::custom(format!(
                "Supplied identifier string has improper format {}",
                &value
            ))),
        }?;

        let identifier = match chunks.get(1) {
            Some(string) => Ok(*string),
            None => Err(D::Error::custom(format!(
                "Supplied identifier string has improper format {}",
                &value
            ))),
        }?;

        match resource {
            "authors" => Ok(OpenLibraryResource::Author(identifier.to_string())),
            "books" => Ok(OpenLibraryResource::Book(identifier.to_string())),
            "works" => Ok(OpenLibraryResource::Work(identifier.to_string())),
            _ => Err(D::Error::custom("Could not parse into Resource")),
        }
    }
}

impl Serialize for OpenLibraryResource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl From<OpenLibraryResource> for OpenLibraryIdentifier {
    fn from(resource: OpenLibraryResource) -> Self {
        Self::from_str(&resource.value()).unwrap()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Link {
    #[serde(skip_serializing_if = "String::is_empty")]
    url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    title: String,
}

#[derive(Clone, Deserialize, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LinkName {
    Author,
    #[serde(rename = "self")]
    Itself,
    Next,
}

pub trait OpenLibraryModel {}
