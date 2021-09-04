use clients::books::BooksClient;
use reqwest::header::HeaderMap;
use reqwest::ClientBuilder;
use std::fmt::Debug;
use thiserror::Error;

mod clients;
pub mod models;
#[cfg(test)]
mod tests;

#[derive(Debug, Error)]
pub enum OpenLibraryError {
    #[error("Unable to build HTTP client: {}", source)]
    ClientBuildingError { source: reqwest::Error },
    #[error("An error occurred while parsing json: {}", source)]
    JsonParseError { source: reqwest::Error },
    #[error("An error occurred while trying to parse a value: {}", reason)]
    ParsingError { reason: String },
    #[error("An error occurred while sending HTTP request: {}", source)]
    RequestFailed { source: reqwest::Error },
}

pub struct OpenLibraryClient {
    pub books: BooksClient,
}

impl OpenLibraryClient {
    pub fn new() -> Result<OpenLibraryClient, OpenLibraryError> {
        let client = ClientBuilder::new()
            .default_headers(HeaderMap::new())
            .build()
            .map_err(|error| OpenLibraryError::ClientBuildingError { source: error })?;

        Ok(Self {
            books: BooksClient::new(client),
        })
    }
}
