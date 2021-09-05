use crate::models::books::{BookIdentifier, BookIdentifierKey};
use crate::OpenLibraryClient;
use std::error::Error;

#[tokio::test]
async fn verify_search_call() -> Result<(), Box<dyn Error>> {
    //TODO: get wiremock working for unit tests
    let client = OpenLibraryClient::builder().build()?;
    let results = client
        .books
        .search(vec![
            &BookIdentifier::from(BookIdentifierKey::ISBN, "0451526538".to_string()),
            &BookIdentifier::from(BookIdentifierKey::GoodReads, "0451526538".to_string()),
        ])
        .await?;

    assert_eq!(results.len(), 1);
    Ok(())
}
