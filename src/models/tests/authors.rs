use crate::models::authors::{
    AuthorDetails, AuthorResponse, AuthorWorksRequest, AuthorWorksResponse,
};
use crate::models::identifiers::OpenLibraryIdentifer;
use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;
use url::Url;

#[tokio::test]
pub async fn test_author_search_response_serde() -> Result<(), Box<dyn Error>> {
    let input = include_str!("resources/author/search.json");
    let actual = serde_json::from_str::<AuthorResponse>(input)?;
    let _output = serde_json::to_string(&actual)?;

    //TODO figure out actual equality
    Ok(())
}

#[tokio::test]
pub async fn test_author_details_response_serde() -> Result<(), Box<dyn Error>> {
    let input = include_str!("resources/author/get.json");
    let actual = serde_json::from_str::<AuthorDetails>(input)?;
    let _output = serde_json::to_string(&actual)?;

    //TODO figure out actual equality
    Ok(())
}

#[tokio::test]
pub async fn test_author_works_response_serde() -> Result<(), Box<dyn Error>> {
    let input = include_str!("resources/author/get_works.json");
    let actual = serde_json::from_str::<AuthorWorksResponse>(input)?;
    let _output = serde_json::to_string(&actual)?;

    //TODO figure out actual equality
    Ok(())
}

#[test]
pub fn test_author_works_from_identifier() -> Result<(), Box<dyn Error>> {
    let expected_identifier = OpenLibraryIdentifer::from_str("OL4452558A")?;
    let expected = AuthorWorksRequest {
        identifier: expected_identifier.clone(),
        limit: None,
        offset: None,
    };
    let actual = AuthorWorksRequest::try_from(expected_identifier)?;

    assert_eq!(actual, expected);
    Ok(())
}

#[test]
pub fn test_author_works_from_url() -> Result<(), Box<dyn Error>> {
    let result = Url::parse("https://www.openlibrary.org/authors/OL4452558A/works.json?limit=75")?;
    let request_builder = AuthorWorksRequest::try_from(result)?;

    assert_eq!(
        request_builder,
        AuthorWorksRequest {
            identifier: OpenLibraryIdentifer::from_str("OL4452558A")?,
            limit: Some(75),
            offset: None
        }
    );

    Ok(())
}
