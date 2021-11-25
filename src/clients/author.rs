use crate::clients::handle;
use crate::models::authors::{AuthorDetails, AuthorResponse};
use crate::models::identifiers::OpenLibraryIdentifer;
use crate::OpenLibraryError;
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

    pub async fn get(
        &self,
        identifier: OpenLibraryIdentifer,
    ) -> Result<AuthorDetails, OpenLibraryError> {
        let url = self
            .host
            .join(format!("/authors/{}.json", identifier).as_str())?;

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
