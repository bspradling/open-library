use crate::clients::handle;
use crate::models::books::{BibliographyKey, Book};
use crate::models::identifiers::{
    Identifier, InternationalStandardBookNumber, OpenLibraryIdentifier,
};
use crate::models::OpenLibraryModel;
use crate::OpenLibraryError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[derive(Clone)]
pub struct BooksClient {
    client: Client,
    host: Url,
}

type BookSearchResponse = HashMap<BibliographyKey, Book>;
impl OpenLibraryModel for BookSearchResponse {}

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
        let url = self
            .host
            .join(format!("/isbn/{}.json", isbn.value()).as_str())?;

        handle(self.client.get(url)).await
    }

    pub async fn get(&self, identifier: OpenLibraryIdentifier) -> Result<Book, OpenLibraryError> {
        let url = self
            .host
            .join(format!("/books/{}.json", identifier.value()).as_str())?;

        handle(self.client.get(url)).await
    }

    pub async fn search<'a, T: Into<&'a Vec<BibliographyKey>>>(
        &self,
        identifiers: T,
    ) -> Result<BookSearchResponse, OpenLibraryError> {
        let ids_filter = identifiers
            .into()
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        handle(self.client.get(self.host.join("/api/books")?).query(&[
            (QueryParameters::BibliographyKeys, &ids_filter),
            (QueryParameters::Format, &String::from("json")),
            (QueryParameters::JavascriptCommand, &String::from("data")),
        ]))
        .await
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
