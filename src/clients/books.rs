use crate::models::books::{BibliographyKey, Book};
use crate::models::identifiers::{Identifier, InternationalStandardBookNumber};
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

    pub async fn by_isbn(
        &self,
        isbn: InternationalStandardBookNumber,
    ) -> Result<Book, OpenLibraryError> {
        let path = format!("/isbn/{}.json", isbn.value());
        let response =
            self.client
                .get(self.host.join(path.as_str()).map_err(|error| {
                    OpenLibraryError::ParsingError {
                        reason: format!(
                            "Invalid URL from base url ({}) and path ({}): {}",
                            self.host, path, error
                        ),
                    }
                })?)
                .send()
                .await
                .map_err(|error| OpenLibraryError::RequestFailed { source: error })?;

        return match response.status() {
            StatusCode::OK => Ok(response
                .json::<Book>()
                .await
                .map_err(|error| OpenLibraryError::JsonParseError { source: error })?),
            _ => Err(OpenLibraryError::ApiError {
                status_code: response.status(),
                error: None,
            }),
        };
    }

    pub async fn search(
        &self,
        identifiers: &[BibliographyKey],
    ) -> Result<HashMap<BibliographyKey, Book>, OpenLibraryError> {
        // tracing::info!("Identifiers: {:?}", identifiers);
        let ids_filter = identifiers
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let path = "/api/books";
        let response = self
            .client
            .get(
                self.host
                    .join(path)
                    .map_err(|_error| OpenLibraryError::ParsingError {
                        reason: format!(
                            "Invalid URL from base url ({}) and path ({}).",
                            self.host, path
                        ),
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
