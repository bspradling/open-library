use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};

pub mod account;
pub mod authors;
pub mod books;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub struct Identifier<T: OpenLibraryIdentifierKey> {
    pub resource: T,
    pub identifier: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Resource {
    #[serde(alias = "authors")]
    Author,
    #[serde(alias = "books")]
    Book,
    #[serde(alias = "works")]
    Work,
}

pub trait OpenLibraryIdentifierKey {}
impl OpenLibraryIdentifierKey for Resource {}

impl From<&str> for Resource {
    fn from(value: &str) -> Self {
        match value {
            "authors" => Resource::Author,
            "books" => Resource::Book,
            "works" => Resource::Work,
            _ => panic!("for now: {}", value),
        }
    }
}

impl Display for Resource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            Resource::Author => "authors",
            Resource::Book => "books",
            Resource::Work => "works",
        };

        write!(f, "{}", x)?;
        Ok(())
    }
}

impl<'de> Deserialize<'de> for Identifier<Resource> {
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

        Ok(Identifier {
            resource: Resource::from(resource),
            identifier: identifier.to_string(),
        })
    }
}

impl Serialize for Identifier<Resource> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string = format!("/{}/{}", self.resource.to_string(), self.identifier);
        serializer.serialize_str(string.as_str())
    }
}
