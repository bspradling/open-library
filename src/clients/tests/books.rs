use crate::models::books::{BibliographyKey, Book};
use crate::{OpenLibraryClient, OpenLibraryError};
use http::Method;
use reqwest::Url;
use std::collections::HashMap;
use std::error::Error;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod isbn;

#[tokio::test]
async fn test_book_search_returns_success() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start().await;
    let client = OpenLibraryClient::builder()
        .with_host(Url::parse(server.uri().as_str())?)
        .build()?;

    let mock_response: HashMap<BibliographyKey, Book> =
        serde_json::from_str(include_str!("resources/book.json"))?;

    let key = BibliographyKey::ISBN("0201558025".to_string());
    let identifiers = vec![key];
    let ids_filter = identifiers
        .clone()
        .into_iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(",");

    Mock::given(method(Method::GET.as_str()))
        .and(path("/api/books"))
        .and(query_param("bibkeys", &ids_filter))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let actual = client.books.search(&identifiers).await?;

    assert_eq!(actual.keys().len(), 3);
    Ok(())
}

#[tokio::test]
async fn test_book_search_returns_failure_when_request_fails() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start().await;
    let client = OpenLibraryClient::builder()
        .with_host(Url::parse(server.uri().as_str())?)
        .build()?;

    let key = BibliographyKey::ISBN("0201558025".to_string());
    let identifiers = vec![key];
    let ids_filter = identifiers
        .clone()
        .into_iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(",");

    Mock::given(method(Method::GET.as_str()))
        .and(path("/api/books"))
        .and(query_param("bibkeys", &ids_filter))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let actual = client.books.search(&identifiers).await;
    let error = actual.expect_err("Expected Book Search call to return an error but it didn't!");
    match &error {
        OpenLibraryError::ApiError {
            status_code: _,
            error: _,
        } => Ok(()),
        _ => panic!(
            "Expected to received an API error, but received {:?} instead!",
            error
        ),
    }
}
