use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};

pub mod account;
pub mod authors;
pub mod books;
pub mod identifiers;
pub mod works;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Resource {
    Author(String),
    Book(String),
    Work(String),
}
//
pub trait OpenLibraryModel {}
pub trait OpenLibraryIdentifierKey {}
impl OpenLibraryIdentifierKey for Resource {}

impl Display for Resource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            Resource::Author(value) => format!("/authors/{}", value),
            Resource::Book(value) => format!("/books/{}", value),
            Resource::Work(value) => format!("/works/{}", value),
        };

        write!(f, "{}", x)?;
        Ok(())
    }
}

impl<'de> Deserialize<'de> for Resource {
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
            "authors" => Ok(Resource::Author(identifier.to_string())),
            "books" => Ok(Resource::Book(identifier.to_string())),
            "works" => Ok(Resource::Work(identifier.to_string())),
            _ => Err(D::Error::custom("Could not parse into Resource")),
        }
    }
}

impl Serialize for Resource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Link {
    #[serde(skip_serializing_if = "String::is_empty")]
    url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    title: String,
}
