use crate::models::account::{LoginRequest, Session};
use crate::{ErrorResponse, OpenLibraryError};
use reqwest::{Client, StatusCode};
use std::convert::TryFrom;

pub struct AccountClient {
    pub client: Client,
}

impl AccountClient {
    const BASE_URL: &'static str = "https://openlibrary.org/account/login";

    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub async fn login(
        self,
        username: String,
        password: String,
    ) -> Result<Session, OpenLibraryError> {
        let response = self
            .client
            .post(AccountClient::BASE_URL)
            .json(&LoginRequest { username, password })
            .send()
            .await
            .map_err(|error| OpenLibraryError::RequestFailed { source: error })?;

        match response.status() {
            StatusCode::OK => Ok(Session::try_from(response))?,
            _ => Err(OpenLibraryError::ApiError {
                response: response
                    .json::<ErrorResponse>()
                    .await
                    .map_err(|err| OpenLibraryError::JsonParseError { source: err })?,
            }),
        }
    }
}
