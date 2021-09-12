use crate::models::books::{BibliographyKey, Book};
use std::collections::HashMap;
use std::error::Error;
use test_case::test_case;

#[test_case(BibliographyKey::ISBN("1839485743".to_string()),
            "ISBN:1839485743";
            "isbn10")]
#[test_case(BibliographyKey::ISBN("1839485743123".to_string()),
            "ISBN:1839485743123";
            "isbn13")]
#[test_case(BibliographyKey::LCCN("62019420".to_string()),
            "LCCN:62019420";
            "lccn")]
#[test_case(BibliographyKey::OCLC("ocn123456789".to_string()),
            "OCLC:ocn123456789";
            "oclc")]
#[test_case(BibliographyKey::OLID("OL32770978M".to_string()),
            "OLID:OL32770978M";
            "open_library")]
#[tokio::test]
async fn test_bibliography_key_to_string(
    key: BibliographyKey,
    expected: &str,
) -> Result<(), Box<dyn Error>> {
    assert_eq!(key.to_string(), expected.to_string());
    Ok(())
}

#[test_case("ISBN:1839485743"; "isbn10")]
#[test_case("ISBN:1839485743123"; "isbn13")]
#[test_case("LCCN:62019420"; "lccn")]
#[test_case("OCLC:ocn123456789"; "oclc")]
#[test_case("OLID:OL32770978M"; "open_library")]
#[tokio::test]
async fn test_bibliography_key_serde(expected: &str) -> Result<(), Box<dyn Error>> {
    let key: BibliographyKey = serde_json::from_str(expected)?;
    let actual = serde_json::to_string(&key)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[tokio::test]
async fn test_serde_book_required_fields() -> Result<(), Box<dyn Error>> {
    let expected = include_str!("resources/books/required-fields.json");
    let book: HashMap<BibliographyKey, Book> = serde_json::from_str(expected)?;
    let _actual = serde_json::to_string(&book)?;

    // TODO need to refactor BookIdentifier before asserting equality
    // assert_eq!(
    //     // Terrible idea to remove all whitespace
    //     expected.split_whitespace().collect::<String>(),
    //     actual.split_whitespace().collect::<String>()
    // );
    Ok(())
}

#[tokio::test]
async fn test_serde_book_all_fields() -> Result<(), Box<dyn Error>> {
    let expected = include_str!("resources/books/all-fields.json");
    let book: HashMap<BibliographyKey, Book> = serde_json::from_str(expected)?;
    let _actual = serde_json::to_string(&book)?;

    // TODO need to refactor BookIdentifier before asserting equality
    // assert_eq!(
    //     // Terrible idea to remove all whitespace
    //     expected.split_whitespace().collect::<String>(),
    //     actual.split_whitespace().collect::<String>()
    // );
    Ok(())
}
