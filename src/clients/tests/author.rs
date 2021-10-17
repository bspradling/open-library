use crate::models::authors::AuthorResponse;
use crate::{OpenLibraryClient, OpenLibraryError};
use http::Method;
use reqwest::Url;
use std::error::Error;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_author_search_returns_success() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start().await;
    let client = OpenLibraryClient::builder()
        .with_host(Url::parse(server.uri().as_str())?)
        .build()?;

    let mock_response: AuthorResponse =
        serde_json::from_str(include_str!("resources/search_author.json"))?;

    let author_name = String::from("j.k. rowling");

    Mock::given(method(Method::GET.as_str()))
        .and(path("/search/authors.json"))
        .and(query_param("q", &author_name))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let actual = client.author.search(&author_name).await?;

    assert_eq!(actual.docs.len(), 1);
    Ok(())
}

#[tokio::test]
async fn test_author_search_returns_failure_when_request_fails() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start().await;
    let client = OpenLibraryClient::builder()
        .with_host(Url::parse(server.uri().as_str())?)
        .build()?;

    let author_name = String::from("brett spradling");

    Mock::given(method(Method::GET.as_str()))
        .and(path("/search/authors.json"))
        .and(query_param("q", &author_name))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let actual = client.author.search(&author_name).await;
    let error = actual.expect_err("Expected Author Search call to return an error but it didn't!");

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
