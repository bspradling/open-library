extern crate open_library;
use open_library::models::books::BibliographyKey;
use open_library::{OpenLibraryAuthClient, OpenLibraryClient, OpenLibraryError};
use std::error::Error;

#[tokio::test]
async fn test_book_search() -> Result<(), Box<dyn Error>> {
    let client = OpenLibraryClient::builder().build()?;
    let identifier = BibliographyKey::ISBN("0374386137".to_string());
    let book_results = client.books.search(vec![&identifier]).await?;
    let book = book_results
        .get(&identifier)
        .ok_or(format!("No book found with identifier {}", identifier))?;

    assert_eq!(book.title, "A Wrinkle in Time");

    Ok(())
}

#[tokio::test]
async fn test_login() -> Result<(), Box<dyn Error>> {
    let auth_client = OpenLibraryAuthClient::new(None)?;
    let username = std::env::var("OPEN_LIBRARY_USERNAME").map_err(|_e| {
        OpenLibraryError::NotAuthenticated {
            reason: "Unable to find username in environment variables!".to_string(),
        }
    })?;
    let password = std::env::var("OPEN_LIBRARY_PASSWORD").map_err(|_e| {
        OpenLibraryError::NotAuthenticated {
            reason: "Unable to find password in environment variables!".to_string(),
        }
    })?;
    let session = auth_client.login(username, password).await?;
    let _client = OpenLibraryClient::builder()
        .with_session(&session)
        .build()?;

    Ok(())
}

#[tokio::test]
async fn test_want_to_read() -> Result<(), Box<dyn Error>> {
    let auth_client = OpenLibraryAuthClient::new(None)?;
    let username = std::env::var("OPEN_LIBRARY_USERNAME").map_err(|_e| {
        OpenLibraryError::NotAuthenticated {
            reason: "Unable to find username in environment variables!".to_string(),
        }
    })?;
    let password = std::env::var("OPEN_LIBRARY_PASSWORD").map_err(|_e| {
        OpenLibraryError::NotAuthenticated {
            reason: "Unable to find password in environment variables!".to_string(),
        }
    })?;
    let session = auth_client
        .login(username.clone(), password.clone())
        .await?;
    let client = OpenLibraryClient::builder()
        .with_session(&session)
        .build()?;

    let reading_log_entries = client.account.get_want_to_read(username).await?;

    assert_eq!(
        reading_log_entries.get(0).unwrap().work.title,
        "Atomic Habits"
    );
    Ok(())
}
