use crate::models::books::{BookIdentifier, BookIdentifierKey};
use crate::OpenLibraryClient;

#[tokio::test]
async fn verify_search_call() {
    //TODO: get wiremock working for unit tests
    let client = OpenLibraryClient::new().unwrap();
    let results = client
        .books
        .search(vec![
            &BookIdentifier::from(BookIdentifierKey::ISBN, "0451526538".to_string()),
            &BookIdentifier::from(BookIdentifierKey::GoodReads, "0451526538".to_string()),
        ])
        .await;
    assert_eq!(results.unwrap().len(), 1)
}
