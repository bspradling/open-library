use crate::models::identifiers::{Identifier, OpenLibraryIdentifer};
use crate::models::works::Work;
use crate::OpenLibraryClient;
use reqwest::{Method, Url};
use std::error::Error;
use std::str::FromStr;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_works_get_returns_success() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start().await;
    let client = OpenLibraryClient::builder()
        .with_host(Url::parse(server.uri().as_str())?)
        .build()?;

    let mock_work_identifier = OpenLibraryIdentifer::from_str("OL92304270")?;
    let mock_response: Work = serde_json::from_str(include_str!("resources/work.json"))?;

    Mock::given(method(Method::GET.as_str()))
        .and(path(format!(
            "/works/{}.json",
            mock_work_identifier.value()
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let actual = client.works.get(&mock_work_identifier).await?;

    assert_eq!(actual, mock_response);
    Ok(())
}
