use open_library::models::books::BibliographyKey;
use open_library::models::identifiers::{InternationalStandardBookNumber, OpenLibraryIdentifer};
use open_library::{OpenLibraryAuthClient, OpenLibraryClient, OpenLibraryError};
use std::error::Error;
use std::str::FromStr;

#[tokio::test]
async fn test_author_get() -> Result<(), Box<dyn Error>> {
    let client = OpenLibraryClient::builder().build()?;
    let identifier = OpenLibraryIdentifer::from("OL4452558A")?;
    let author = client.author.get(identifier).await?;

    assert_eq!(author.name, "Gary Paulsen");
    Ok(())
}

#[tokio::test]
async fn test_author_search() -> Result<(), Box<dyn Error>> {
    let client = OpenLibraryClient::builder().build()?;
    let author = client.author.search("Markus Zusak").await?;

    assert!(author.docs.len() >= 1);
    Ok(())
}

#[tokio::test]
async fn test_book_by_isbn() -> Result<(), Box<dyn Error>> {
    let client = OpenLibraryClient::builder().build()?;
    let isbn = InternationalStandardBookNumber::from_str("0374386137")?;
    let book = client.books.by_isbn(isbn).await?;

    assert_eq!(book.title, "A Wrinkle in Time");
    Ok(())
}

#[tokio::test]
async fn test_book_get() -> Result<(), Box<dyn Error>> {
    let client = OpenLibraryClient::builder().build()?;
    let olid = OpenLibraryIdentifer::from_str("OL8458764M")?;
    let book = client.books.get(olid).await?;

    assert_eq!(book.title, "Hatchet");

    Ok(())
}

#[tokio::test]
async fn test_book_search() -> Result<(), Box<dyn Error>> {
    let client = OpenLibraryClient::builder().build()?;
    let identifier = BibliographyKey::ISBN("0374386137".to_string());
    let identifiers = vec![identifier.clone()];
    let book_results = client.books.search(identifiers).await?;
    let book = book_results
        .get(&identifier)
        .ok_or(format!("No book found with identifier {}", identifier))?;

    assert_eq!(book.title, "A Wrinkle in Time");

    Ok(())
}

#[tokio::test]
async fn test_works_get() -> Result<(), Box<dyn Error>> {
    let client = OpenLibraryClient::builder().build()?;
    let works_id = OpenLibraryIdentifer::from_str("OL7353617M")?;
    let work = client.works.get(&works_id).await?;

    assert_eq!(work.title, "Fantastic Mr. Fox");

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
