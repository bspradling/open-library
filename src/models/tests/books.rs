use crate::models::books::{BookIdentifier, BookIdentifierKey};
use std::error::Error;
use test_case::test_case;

#[test_case(BookIdentifier::from(BookIdentifierKey::ISBN, "1839485743".to_string()),
            "ISBN:1839485743";
            "isbn10")]
#[test_case(BookIdentifier::from(BookIdentifierKey::ISBN, "1839485743123".to_string()),
            "ISBN:1839485743123";
            "isbn13")]
#[test_case(BookIdentifier::from(BookIdentifierKey::LCCN, "62019420".to_string()),
            "LCCN:62019420";
            "lccn")]
#[test_case(BookIdentifier::from(BookIdentifierKey::OCLC, "ocn123456789".to_string()),
            "OCLC:ocn123456789";
            "oclc")]
#[test_case(BookIdentifier::from(BookIdentifierKey::OpenLibrary, "OL32770978M".to_string()),
            "OLID:OL32770978M";
            "open_library")]
#[test_case(BookIdentifier::from(BookIdentifierKey::GoodReads, "24583".to_string()),
            "GRID:24583";
            "good_reads")]
#[tokio::test]
async fn test_book_identifier_to_string(
    identifier: BookIdentifier,
    expected: &str,
) -> Result<(), Box<dyn Error>> {
    assert_eq!(identifier.to_string(), expected.to_string());
    Ok(())
}

#[tokio::test]
async fn test_book_identifier_key_serde() -> Result<(), Box<dyn Error>> {
    assert_eq!(BookIdentifierKey::ISBN.to_string(), "ISBN");
    Ok(())
}
