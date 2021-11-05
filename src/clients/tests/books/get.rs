use crate::models::books::Book;
use crate::models::identifiers::{Identifier, OpenLibraryIdentifer};
use crate::OpenLibraryClient;
use http::Method;
use reqwest::Url;
use std::error::Error;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_returns_success() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start().await;
    let client = OpenLibraryClient::builder()
        .with_host(Url::parse(server.uri().as_str())?)
        .build()?;

    let mock_response: Book = serde_json::from_str(include_str!("resources/edition.json"))?;

    let olid = OpenLibraryIdentifer::from("OL7353617M")?;

    Mock::given(method(Method::GET.as_str()))
        .and(path(format!("/books/{}.json", olid.value()).as_str()))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let actual = client.books.get(olid).await?;
    assert_eq!(actual, mock_response);
    Ok(())
}
