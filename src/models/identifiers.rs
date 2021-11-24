use crate::OpenLibraryError;
use serde;
use std::fmt::Display;
use std::str::FromStr;

pub trait Identifier: Display {
    fn acronym(&self) -> &'static str;
    fn value(&self) -> &str;
}

macro_rules! identifier {
    ($name:ident, $acy: literal) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
        #[serde(transparent)]
        pub struct $name(String);

        impl Identifier for $name {
            fn acronym(&self) -> &'static str {
                $acy
            }

            fn value(&self) -> &str {
                &self.0.as_str()
            }
        }

        impl FromStr for $name {
            type Err = OpenLibraryError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok($name(s.to_string()))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

identifier!(InternationalStandardBookNumber, "ISBN");
identifier!(OpenLibraryIdentifer, "OLID");
