use crate::clients::account::AccountClient;
use crate::models::account::Session;
use clients::books::BooksClient;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{ClientBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;
use url::Url;

mod clients;
mod format;
pub mod models;
#[cfg(test)]
mod tests;

#[derive(Debug, Error)]
pub enum OpenLibraryError {
    #[error(
        "Received an {:?} response from the Open Library API: {:?}",
        status_code,
        error
    )]
    ApiError {
        status_code: StatusCode,
        error: Option<OpenLibraryErrorResponse>,
    },
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

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenLibraryErrorResponse {
    pub error: String,
}

pub struct OpenLibraryAuthClient {
    account: AccountClient,
}

impl OpenLibraryAuthClient {
    pub fn new(host: Option<Url>) -> Result<OpenLibraryAuthClient, OpenLibraryError> {
        let client = ClientBuilder::new()
            .build()
            .map_err(|error| OpenLibraryError::ClientBuildingError { source: error })?;

        let host_url = match host {
            Some(value) => value,
            None => Url::parse("https://openlibrary.org/").unwrap(),
        };

        Ok(Self {
            account: AccountClient {
                client,
                host: host_url,
            },
        })
    }

    pub async fn login(
        &self,
        username: String,
        password: String,
    ) -> Result<Session, OpenLibraryError> {
        self.account.login(username, password).await
    }
}

#[derive(Clone)]
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
    host: Url,
    session: Option<Session>,
}

impl OpenLibraryClientBuilder {
    fn new() -> OpenLibraryClientBuilder {
        OpenLibraryClientBuilder {
            host: Url::parse("https://openlibrary.org/").unwrap(),
            session: None,
        }
    }

    pub fn with_host(self, host: Url) -> OpenLibraryClientBuilder {
        OpenLibraryClientBuilder {
            host,
            session: self.session,
        }
    }

    pub fn with_session(self, session: &Session) -> OpenLibraryClientBuilder {
        OpenLibraryClientBuilder {
            host: self.host,
            session: Some(session.clone()),
        }
    }

    pub fn build(self) -> Result<OpenLibraryClient, OpenLibraryError> {
        let default_headers = match self.session {
            Some(session) => {
                let mut headers = HeaderMap::new();
                headers.insert(
                    "Cookie",
                    HeaderValue::from_str(session.cookie().as_str()).map_err(|_error| {
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
            books: BooksClient::new(&client, &self.host),
            account: AccountClient::new(&client, &self.host),
        })
    }
}
