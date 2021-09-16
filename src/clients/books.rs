use crate::models::books::{BibliographyKey, Book};
use crate::OpenLibraryError;
use http::StatusCode;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[derive(Clone)]
pub struct BooksClient {
    client: Client,
    host: Url,
}

impl BooksClient {
    pub fn new(client: &Client, host: &Url) -> Self {
        Self {
            client: client.clone(),
            host: host.clone(),
        }
    }

    pub async fn search(
        &self,
        identifiers: &Vec<BibliographyKey>,
    ) -> Result<HashMap<BibliographyKey, Book>, OpenLibraryError> {
        // tracing::info!("Identifiers: {:?}", identifiers);
        let ids_filter = identifiers
            .into_iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let response = self
            .client
            .get(
                self.host
                    .join("/api/books")
                    .map_err(|_e| OpenLibraryError::ParsingError {
                        reason: "Unable to parse into valid URL".to_string(),
                    })?,
            )
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
    #[serde(rename = "bibkeys")]
    BibliographyKeys,
    #[serde(rename = "format")]
    Format,
    #[serde(rename = "jscmd")]
    JavascriptCommand,
}
