use crate::models::books::{Book, BookIdentifier, BookIdentifierKey};
use crate::OpenLibraryError;
use itertools::{Either, Itertools};
use reqwest::Client;
use std::collections::HashMap;

pub struct BooksClient {
    client: Client,
}

impl BooksClient {
    const BASE_URL: &'static str = "https://openlibrary.org/api/books";

    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub async fn search(
        self,
        identifiers: Vec<&BookIdentifier>,
    ) -> Result<HashMap<BookIdentifier, Book>, OpenLibraryError> {
        let unsupported_identifier_keys = vec![
            BookIdentifierKey::GoodReads,
            BookIdentifierKey::LibraryThing,
            BookIdentifierKey::ProjectGutenberg,
        ];
        let (supported_ids, unsupported_ids): (Vec<BookIdentifier>, Vec<BookIdentifier>) =
            identifiers.into_iter().partition_map(|id| {
                match unsupported_identifier_keys.contains(&id.clone().key) {
                    true => Either::Right(id.clone()),
                    false => Either::Left(id.clone()),
                }
            });

        if !unsupported_ids.is_empty() {
            println!("[WARNING] Some specified identifiers {:?} are not supported via Book Search, they will be ignored!",
                     unsupported_ids);
            tracing::warn!(
                "Some specified identifiers {:?} are not supported via Book Search, they will be ignored!",
                unsupported_ids
            );
        }

        tracing::info!("Supported id: {:?}", &supported_ids);

        let ids_filter = supported_ids
            .into_iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let response = self
            .client
            .get(BooksClient::BASE_URL)
            .query(&[
                ("bibkeys", &ids_filter),
                ("format", &"json".to_string()),
                ("jscmd", &"data".to_string()),
            ])
            .send()
            .await
            .map_err(|error| OpenLibraryError::RequestFailed { source: error })?;

        let results: HashMap<BookIdentifier, Book> = response
            .json::<HashMap<BookIdentifier, Book>>()
            .await
            .map_err(|error| OpenLibraryError::JsonParseError { source: error })?;

        Ok(results)
    }
}
