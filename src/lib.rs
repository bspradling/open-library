use crate::clients::account::AccountClient;
use crate::models::account::Session;
use clients::books::BooksClient;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::ClientBuilder;
use serde::Deserialize;
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

mod clients;
mod format;
pub mod models;
#[cfg(test)]
mod tests;

#[derive(Debug, Error)]
pub enum OpenLibraryError {
    #[error("Received an error response from the Open Library API: {}", response)]
    ApiError { response: ErrorResponse },
    #[error("Unable to build HTTP client: {}", source)]
    ClientBuildingError { source: reqwest::Error },
    #[error("An internal error occurred: {}", reason)]
    InternalError { reason: String },
    #[error("An error occurred while parsing json: {}", source)]
    JsonParseError { source: reqwest::Error },
    #[error("The operation ({}) requires authentication to be provided!", reason)]
    NotAuthenticated { reason: String },
    #[error("An error occurred while trying to parse a value: {}", reason)]
    ParsingError { reason: String },
    #[error("An error occurred while sending HTTP request: {}", source)]
    RequestFailed { source: reqwest::Error },
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

pub struct OpenLibraryAuthClient {
    account: AccountClient,
}

impl OpenLibraryAuthClient {
    pub fn new() -> Result<OpenLibraryAuthClient, OpenLibraryError> {
        let client = ClientBuilder::new()
            .build()
            .map_err(|error| OpenLibraryError::ClientBuildingError { source: error })?;

        Ok(Self {
            account: AccountClient { client },
        })
    }

    pub async fn login(
        self,
        username: String,
        password: String,
    ) -> Result<Session, OpenLibraryError> {
        self.account.login(username, password).await
    }
}

pub struct OpenLibraryClient {
    pub account: AccountClient,
    pub books: BooksClient,
}

impl OpenLibraryClient {
    pub fn builder() -> OpenLibraryClientBuilder {
        OpenLibraryClientBuilder::new()
    }
}

pub struct OpenLibraryClientBuilder {
    session: Option<Session>,
}

impl OpenLibraryClientBuilder {
    fn new() -> OpenLibraryClientBuilder {
        OpenLibraryClientBuilder { session: None }
    }

    pub fn with_session(self, session: Session) -> OpenLibraryClientBuilder {
        OpenLibraryClientBuilder {
            session: Some(session),
        }
    }

    pub fn build(self) -> Result<OpenLibraryClient, OpenLibraryError> {
        let default_headers = match self.session.clone() {
            Some(session) => {
                let mut headers = HeaderMap::new();
                headers.insert(
                    "Cookie",
                    HeaderValue::from_str(session.cookie.as_str()).map_err(|_error| {
                        OpenLibraryError::ParsingError {
                            reason: "Unable to parse session cookie into header value".to_string(),
                        }
                    })?,
                );
                headers
            }
            None => HeaderMap::new(),
        };

        let client = ClientBuilder::new()
            .default_headers(default_headers)
            .build()
            .map_err(|error| OpenLibraryError::ClientBuildingError { source: error })?;

        Ok(OpenLibraryClient {
            books: BooksClient::new(&client),
            account: AccountClient::new(&client),
        })
    }
}
