use crate::clients::get;
use crate::models::books::{BibliographyKey, Book};
use crate::models::identifiers::{
    Identifier, InternationalStandardBookNumber, OpenLibraryIdentifer,
};
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
        let response = self
            .client
            .get(self.host.join(path.as_str())?)
            .send()
            .await?;

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

    pub async fn get(&self, identifier: OpenLibraryIdentifer) -> Result<Book, OpenLibraryError> {
        let url = self
            .host
            .join(format!("/books/{}.json", identifier.value()).as_str())?;

        get(&self.client, url).await
    }

    pub async fn search<T>(
        &self,
        identifiers: T,
    ) -> Result<HashMap<BibliographyKey, Book>, OpenLibraryError>
    where
        T: Into<Vec<BibliographyKey>>,
    {
        let ids_filter = identifiers
            .into()
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let response = self
            .client
            .get(self.host.join("/api/books")?)
            .query(&[
                (QueryParameters::BibliographyKeys, &ids_filter),
                (QueryParameters::Format, &String::from("json")),
                (QueryParameters::JavascriptCommand, &String::from("data")),
            ])
            .send()
            .await?;

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
