use crate::models::books::{BibliographyKey, Book};
use crate::OpenLibraryError;
use http::StatusCode;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct BooksClient {
    client: Client,
    host: String,
}

impl BooksClient {
    pub fn new(client: &Client, host: &String) -> Self {
        Self {
            client: client.clone(),
            host: host.clone(),
        }
    }

    pub async fn search(
        self,
        identifiers: Vec<&BibliographyKey>,
    ) -> Result<HashMap<BibliographyKey, Book>, OpenLibraryError> {
        // tracing::info!("Identifiers: {:?}", identifiers);
        let ids_filter = identifiers
            .into_iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let response = self
            .client
            .get(format!("{}/api/books", self.host))
            .query(&[
                (QueryParameters::BibliographyKeys, &ids_filter),
                (QueryParameters::Format, &String::from("json")),
                (QueryParameters::JavascriptCommand, &String::from("data")),
            ])
            .send()
            .await
            .map_err(|error| OpenLibraryError::RequestFailed { source: error })?;

        let results: HashMap<BibliographyKey, Book> = match response.status() {
            StatusCode::OK => Ok(response
                .json::<HashMap<BibliographyKey, Book>>()
                .await
                .map_err(|error| OpenLibraryError::JsonParseError { source: error })?),
            _ => Err(OpenLibraryError::ApiError { response }),
        }?;

        Ok(results)
    }
}

#[derive(Deserialize, Serialize)]
enum QueryParameters {
    #[serde(rename = "bibkeys")]
    BibliographyKeys,
    #[serde(rename = "format")]
    Format,
    #[serde(rename = "jscmd")]
    JavascriptCommand,
}
