use crate::models::account::{LoginRequest, ReadingLogResponse, Session};
use crate::{OpenLibraryAuthClient, OpenLibraryClient, OpenLibraryError, OpenLibraryErrorResponse};
use http::Method;
use std::error::Error;
use url::Url;
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_login_returns_success() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start().await;
    let client = OpenLibraryAuthClient::new(Some(Url::parse(server.uri().as_str())?))?;

    let expected = Session::from("mock_session_cookie".to_string(), "mock_user".to_string());

    Mock::given(method(Method::POST.as_str()))
        .and(path("/account/login"))
        .and(body_json(LoginRequest {
            username: expected.username().to_string(),
            password: "mock_password".to_string(),
        }))
        .respond_with(ResponseTemplate::new(200).append_header(
            http::header::SET_COOKIE.as_str(),
            expected.cookie().as_str(),
        ))
        .mount(&server)
        .await;

    let actual = client
        .login("mock_user".to_string(), "mock_password".to_string())
        .await?;

    assert_eq!(expected, actual);
    Ok(())
}

#[tokio::test]
async fn test_login_returns_error_on_failure() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start().await;
    let client = OpenLibraryAuthClient::new(Some(Url::parse(server.uri().as_str())?))?;

    let expected = Session::from("mock_session_cookie".to_string(), "mock_user".to_string());

    Mock::given(method(Method::POST.as_str()))
        .and(path("/account/login"))
        .and(body_json(LoginRequest {
            username: expected.username().to_string(),
            password: "mock_password".to_string(),
        }))
        .respond_with(ResponseTemplate::new(400))
        .mount(&server)
        .await;

    let actual = client
        .login("mock_user".to_string(), "mock_password".to_string())
        .await;

    let error = actual.expect_err("Expected login to return an error!");

    match error {
        OpenLibraryError::ApiError {
            status_code: _,
            error: _,
        } => Ok(()),
        _ => panic!(
            "Expected to receive an Api Error but received {:?} instead!",
            &error
        ),
    }
}

#[tokio::test]
async fn test_want_to_read_returns_success() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start().await;

    let mock_session = Session::from("mock_session_cookie".to_string(), "mock_user".to_string());

    let client = OpenLibraryClient::builder()
        .with_session(&mock_session)
        .with_host(Url::parse(server.uri().as_str())?)
        .build()?;

    let mock_response: ReadingLogResponse =
        serde_json::from_str(include_str!("resources/want-to-read.json"))?;

    Mock::given(method(Method::GET.as_str()))
        .and(path("/people/mock_user/books/want-to-read.json"))
        .and(header(
            http::header::COOKIE.as_str(),
            mock_session.cookie().as_str(),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&server)
        .await;

    let actual = client
        .account
        .get_want_to_read("mock_user".to_string())
        .await?;

    assert_eq!(actual.len(), 1);
    Ok(())
}

#[tokio::test]
async fn test_want_to_read_returns_failure_when_error_is_returned() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start().await;

    let mock_session = Session::from("mock_session_cookie".to_string(), "mock_user".to_string());

    let client = OpenLibraryClient::builder()
        .with_session(&mock_session)
        .with_host(Url::parse(server.uri().as_str())?)
        .build()?;

    Mock::given(method(Method::GET.as_str()))
        .and(path("/people/mock_user/books/want-to-read.json"))
        .and(header(
            http::header::COOKIE.as_str(),
            mock_session.cookie().as_str(),
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(OpenLibraryErrorResponse {
                error: "Shelf want-to-read not found or not accessible".to_string(),
            }),
        )
        .mount(&server)
        .await;

    let actual = client
        .account
        .get_want_to_read("mock_user".to_string())
        .await;

    let error = actual.err().unwrap();
    match error {
        OpenLibraryError::ApiError {
            status_code: _,
            error: _,
        } => Ok(()),
        _ => panic!(
            "Expected call to return a Api Error but returned {:?} instead",
            error
        ),
    }
}
