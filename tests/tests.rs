extern crate open_library;
use open_library::models::books::{BookIdentifier, BookIdentifierKey};
use open_library::{OpenLibraryAuthClient, OpenLibraryClient};
use std::error::Error;

#[tokio::test]
async fn verify_identifier_values() -> Result<(), Box<dyn Error>> {
    let client = OpenLibraryClient::builder().build()?;
    let identifier = BookIdentifier::from(BookIdentifierKey::ISBN, "0374386137".to_string());
    let book_results = client.books.search(vec![&identifier]).await?;
    let book = book_results
        .get(&identifier)
        .ok_or(format!("No book found with identifier {}", identifier))?;

    assert_eq!(book.title, "A Wrinkle in Time");

    Ok(())
}

#[tokio::test]
async fn verify_authed_endpoint() -> Result<(), Box<dyn Error>> {
    let auth_client = OpenLibraryAuthClient::new()?;
    let username = std::env::var("OPEN_LIBRARY_USERNAME")?;
    let password = std::env::var("OPEN_LIBRARY_PASSWORD")?;
    let session = auth_client.login(username, password).await?;
    let _client = OpenLibraryClient::builder().with_session(session).build()?;

    Ok(())
}

#[tokio::test]
async fn verify_want_to_read() -> Result<(), Box<dyn Error>> {
    let auth_client = OpenLibraryAuthClient::new()?;
    let username = std::env::var("OPEN_LIBRARY_USERNAME")?;
    let password = std::env::var("OPEN_LIBRARY_PASSWORD")?;
    let session = auth_client
        .login(username.clone(), password.clone())
        .await?;
    let client = OpenLibraryClient::builder().with_session(session).build()?;

    let reading_log_entries = client.account.get_want_to_read(username).await?;

    assert_eq!(
        reading_log_entries.get(0).unwrap().work.title,
        "Atomic Habits"
    );
    Ok(())
}
