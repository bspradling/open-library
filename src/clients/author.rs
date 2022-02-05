use crate::clients::handle;
use crate::models::authors::{
    AuthorDetails, AuthorResponse, AuthorWorksRequest, AuthorWorksResponse,
};
use crate::models::identifiers::OpenLibraryIdentifier;
use crate::OpenLibraryError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
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

    pub async fn get(
        &self,
        identifier: OpenLibraryIdentifier,
    ) -> Result<AuthorDetails, OpenLibraryError> {
        let url = self
            .host
            .join(format!("/authors/{}.json", identifier).as_str())?;

        handle(self.client.get(url)).await
    }

    pub async fn get_works<T>(&self, request: T) -> Result<AuthorWorksResponse, OpenLibraryError>
    where
        T: TryInto<AuthorWorksRequest>,
    {
        let parameters: AuthorWorksRequest =
            request
                .try_into()
                .map_err(|_e| OpenLibraryError::ParsingError {
                    reason: format!("Unable to parse supplied object into a proper request object")
                        .to_string(),
                })?;
        let limit = parameters.limit.unwrap_or(50);
        let offset = parameters.offset.unwrap_or(0);
        let url = self.host.join(
            format!(
                "/authors/{}/works.json?limit={}&offset={}",
                parameters.identifier, limit, offset
            )
            .as_str(),
        )?;

        handle(self.client.get(url)).await
    }

    pub async fn search(&self, author_name: &str) -> Result<AuthorResponse, OpenLibraryError> {
        let url = self.host.join("search/authors.json")?;

        handle(
            self.client
                .get(url)
                .query(&[(QueryParameters::AuthorQuery, author_name)]),
        )
        .await
    }
}

#[derive(Deserialize, Serialize)]
enum QueryParameters {
    #[serde(rename = "q")]
    AuthorQuery,
}
