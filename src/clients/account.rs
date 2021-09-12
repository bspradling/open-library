use crate::models::account::{LoginRequest, ReadingLogEntry, ReadingLogResponse, Session};
use crate::OpenLibraryError;
use reqwest::{Client, StatusCode};
use url::Url;

pub struct AccountClient {
    pub client: Client,
    pub host: String,
}

impl AccountClient {
    pub fn new(client: &Client, host: &String) -> Self {
        Self {
            client: client.clone(),
            host: host.clone(),
        }
    }

    pub async fn login(
        self,
        username: String,
        password: String,
    ) -> Result<Session, OpenLibraryError> {
        let response = self
            .client
            .post(
                Url::parse(format!("{}/account/login", self.host).as_str()).map_err(|error| {
                    OpenLibraryError::InternalError {
                        reason: error.to_string(),
                    }
                })?,
            )
            .json(&LoginRequest {
                username: username.clone(),
                password: password.clone(),
            })
            .send()
            .await
            .map_err(|error| OpenLibraryError::RequestFailed { source: error })?;

        match response.status() {
            StatusCode::OK => {
                let cookie = response
                    .headers()
                    .get(http::header::SET_COOKIE)
                    .ok_or(OpenLibraryError::ParsingError {
                        reason:
                            "The API response from Open Library did not include a Set-Cookie header"
                                .to_string(),
                    })?
                    .to_str()
                    .map_err(|_e| OpenLibraryError::ParsingError {
                        reason: "Unable to parse Set-Cookie Header Value into String".to_string(),
                    })?
                    .to_string();

                Ok(Session { cookie, username })
            }
            _ => Err(OpenLibraryError::ApiError { response }),
        }
    }

    pub async fn get_want_to_read(
        self,
        username: String,
    ) -> Result<Vec<ReadingLogEntry>, OpenLibraryError> {
        let response = self
            .client
            .get(
                Url::parse(
                    format!("{}/people/{}/books/want-to-read.json", self.host, username).as_str(),
                )
                .map_err(|_e| OpenLibraryError::ParsingError {
                    reason: "Unable to parse into valid URL".to_string(),
                })?,
            )
            .send()
            .await
            .map_err(|error| OpenLibraryError::RequestFailed { source: error })?
            .json::<ReadingLogResponse>()
            .await
            .map_err(|error| OpenLibraryError::JsonParseError { source: error })?;

        Ok(response.reading_log_entries)
    }
}
