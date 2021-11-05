use crate::models::authors::{AuthorDetails, AuthorResponse};
use std::error::Error;

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
