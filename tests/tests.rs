extern crate open_library;
use open_library::models::books::{BookIdentifier, BookIdentifierKey};
use open_library::OpenLibraryClient;

#[tokio::test]
async fn verify_identifier_values() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenLibraryClient::new()?;
    let identifier = BookIdentifier::from(BookIdentifierKey::ISBN, "0374386137".to_string());
    let book_results = client.books.search(vec![&identifier]).await?;
    let book = book_results
        .get(&identifier)
        .ok_or(format!("No book found with identifier {}", identifier))?;

    assert_eq!(book.title, "A Wrinkle in Time");

    Ok(())
}
