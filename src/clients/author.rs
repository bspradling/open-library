use crate::models::authors::AuthorResponse;
use crate::OpenLibraryError;
use http::StatusCode;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone)]
pub struct AuthorClient {
    client: Client,
    host: Url,
}

impl AuthorClient {
    pub fn new(client: &Client, host: &Url) -> Self {
        Self {
            client: client.clone(),
            host: host.clone(),
        }
    }

    pub async fn search(&self, author_name: &str) -> Result<AuthorResponse, OpenLibraryError> {
        let response = self
            .client
            .get(self.host.join("search/authors.json").map_err(|_e| {
                OpenLibraryError::ParsingError {
                    reason: "Unable to parse into valid URL".to_string(),
                }
            })?)
            .query(&[(QueryParameters::AuthorQuery, author_name)])
            .send()
            .await
            .map_err(|error| OpenLibraryError::RequestFailed { source: error })?;

        let results: AuthorResponse = match response.status() {
            StatusCode::OK => Ok(response
                .json::<AuthorResponse>()
                .await
                .map_err(|error| OpenLibraryError::JsonParseError { source: error })?),
            _ => Err(OpenLibraryError::ApiError {
                status_code: response.status(),
                error: None,
            }),
        }?;

        Ok(results)
    }
}

#[derive(Deserialize, Serialize)]
enum QueryParameters {
    #[serde(rename = "q")]
    AuthorQuery,
}
