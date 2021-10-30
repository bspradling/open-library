use crate::models::books::Book;
use crate::models::identifiers::{Identifier, InternationalStandardBookNumber};
use crate::{OpenLibraryClient, OpenLibraryError};
use http::Method;
use reqwest::Url;
use std::error::Error;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_isbn_returns_success() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start().await;
    let client = OpenLibraryClient::builder()
        .with_host(Url::parse(server.uri().as_str())?)
        .build()?;

    let mock_response: Book = serde_json::from_str(include_str!("resources/isbn.json"))?;

    let isbn = InternationalStandardBookNumber::from("0201558025")?;

    Mock::given(method(Method::GET.as_str()))
        .and(path(format!("/isbn/{}.json", isbn.value()).as_str()))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let actual = client.books.by_isbn(isbn).await?;
    assert_eq!(actual, mock_response);
    Ok(())
}

#[tokio::test]
async fn test_get_isbn_returns_error_when_invalid_json_returned() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start().await;
    let client = OpenLibraryClient::builder()
        .with_host(Url::parse(server.uri().as_str())?)
        .build()?;

    let isbn = InternationalStandardBookNumber::from("0201558025")?;

    Mock::given(method(Method::GET.as_str()))
        .and(path(format!("/isbn/{}.json", isbn.value()).as_str()))
        .respond_with(ResponseTemplate::new(200).set_body_json("{"))
        .mount(&server)
        .await;

    let actual = client.books.by_isbn(isbn).await;
    let error =
        actual.expect_err("Expected by_isbn call to return an error but returned successfully!");

    match &error {
        OpenLibraryError::JsonParseError { source: _ } => Ok(()),
        _ => panic!(
            "Expected to received an API Not Found error, but received {:?} instead!",
            error
        ),
    }
}

#[tokio::test]
async fn test_get_isbn_returns_error_when_book_does_not_exist() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start().await;
    let client = OpenLibraryClient::builder()
        .with_host(Url::parse(server.uri().as_str())?)
        .build()?;

    let isbn = InternationalStandardBookNumber::from("_doesnotexist")?;

    Mock::given(method(Method::GET.as_str()))
        .and(path(format!("/isbn/{}.json", isbn.value()).as_str()))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let actual = client.books.by_isbn(isbn).await;
    let error =
        actual.expect_err("Expected by_isbn call to return an error but returned successfully!");

    match &error {
        OpenLibraryError::ApiError {
            status_code: http::status::StatusCode::NOT_FOUND,
            error: _,
        } => Ok(()),
        _ => panic!(
            "Expected to received an API Not Found error, but received {:?} instead!",
            error
        ),
    }
}
