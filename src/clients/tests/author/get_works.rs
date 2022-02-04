use crate::models::authors::AuthorWorksResponse;
use crate::models::identifiers::OpenLibraryIdentifer;
use crate::OpenLibraryClient;
use http::Method;
use reqwest::Url;
use std::error::Error;
use std::str::FromStr;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_author_get_works_returns_success() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start().await;
    let client = OpenLibraryClient::builder()
        .with_host(Url::parse(server.uri().as_str())?)
        .build()?;

    let expected: AuthorWorksResponse =
        serde_json::from_str(include_str!("resources/author_works.json"))?;
    let olid = OpenLibraryIdentifer::from_str("OL23919A")?;

    Mock::given(method(Method::GET.as_str()))
        .and(path(format!("/authors/{}/works.json", olid).as_str()))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
        .mount(&server)
        .await;

    let actual = client.author.get_works(olid, None).await?;

    assert_eq!(actual, expected);
    assert_eq!(actual.entries)
    Ok(())
}
