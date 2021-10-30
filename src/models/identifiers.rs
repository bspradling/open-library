use crate::OpenLibraryError;
use std::fmt::Display;

pub trait Identifier: Display {
    fn value(&self) -> &str;
}

macro_rules! identifer {
    ($name:ident) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn from(identifier: &str) -> Result<$name, OpenLibraryError> {
                return Ok($name(identifier.to_string()));
            }
        }

        impl Identifier for $name {
            fn value(&self) -> &str {
                &self.0.as_str()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
    ($name:ident, $validation: expr) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn from(identifier: &str) -> Result<$name, OpenLibraryError> {
                return match $validation {
                    true => Ok($name(identifier.to_string())),
                    false => Err(OpenLibraryError::ParsingError {
                        reason: format!("{} is not a valid {}", identifier, $name),
                    }),
                };
            }
        }
        impl Identifier for $name {
            fn value(&self) -> &str {
                &self.0.as_str()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

identifer!(InternationalStandardBookNumber);

pub trait BibliographyKeyTrait: Identifier {}

impl BibliographyKeyTrait for InternationalStandardBookNumber {}
