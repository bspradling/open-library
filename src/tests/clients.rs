use crate::clients::handle;
use crate::models::OpenLibraryModel;
use crate::OpenLibraryError;
use http::{Method, StatusCode};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct FakeResponse {
    message: String,
}

impl OpenLibraryModel for FakeResponse {}

#[tokio::test]
pub async fn test_get_returns_successfully() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start().await;
    let base_url = Url::parse(server.uri().as_str())?;
    let url_path = "/some/fake/path";
    let expected = FakeResponse {
        message: "Some API response message".to_string(),
    };

    Mock::given(method(Method::GET.as_str()))
        .and(path(url_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&server)
        .await;

    let actual: FakeResponse = handle(Client::new().get(base_url.join(url_path)?)).await?;

    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
pub async fn test_get_returns_error_when_receives_invalid_json() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start().await;
    let base_url = Url::parse(server.uri().as_str())?;
    let url_path = "/some/fake/path";

    Mock::given(method(Method::GET.as_str()))
        .and(path(url_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(&"{"))
        .mount(&server)
        .await;

    let response = handle::<FakeResponse>(Client::new().get(base_url.join(url_path)?)).await;
    let actual =
        response.expect_err("Expected call to return error but it completed successfully!");

    match &actual {
        OpenLibraryError::JsonParseError { source: _ } => Ok(()),
        _ => panic!("Expected to received an error regarding json parsing but didn't!"),
    }
}

#[tokio::test]
pub async fn test_get_returns_error_when_api_responds_with_error() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start().await;
    let base_url = Url::parse(server.uri().as_str())?;
    let url_path = "/some/fake/path";

    Mock::given(method(Method::GET.as_str()))
        .and(path(url_path))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let response = handle::<FakeResponse>(Client::new().get(base_url.join(url_path)?)).await;
    let actual =
        response.expect_err("Expected call to return error but it completed successfully!");

    match &actual {
        OpenLibraryError::ApiError {
            status_code: StatusCode::NOT_FOUND,
            error: _,
        } => Ok(()),
        _ => panic!("Expected to received an error regarding json parsing but didn't!"),
    }
}
