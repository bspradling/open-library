use crate::models::authors::Author;
use std::error::Error;

#[tokio::test]
pub async fn test_author_search_response_serde() -> Result<(), Box<dyn Error>> {
    let input = "{\"key\":\"OL23919A\",\"text\":[\"/authors/OL23919A\",\"J. K. Rowling\",\"Joanne Rowling\",\"Joanne K. Rowling\",\"J.K.Rowling\",\"Rowling, J.K.\",\"J. Rowling\",\"Rowling, Joanne K.\",\"Jo Murray\",\"J K Rowling\",\"Rowling J.K.\",\"J.K. Rowling (author)\",\"Rowling Joanne\",\"J.K. Rowling\",\"J.K. ROWLING\",\"Rowling J K\",\"J K ROWLING\",\"Newt Scamander\",\"JOANNE K. ROWLING\",\"Kennilworthy Whisp\",\"JK Rowling\",\"JK Rowlings\",\"jk rowling\",\"R.K Rowling\",\"J. K Rowling\",\"J.K Rowling\",\"Rowling J. K.\",\"J. K. Rowling (Auteur)\",\"J.k. Rowling\",\"Rowling, J. K.\",\"ROWLING J.K. -\",\"J. K. ROWLING\",\"Rowling,J.K.\",\"J. K. Rowling\",\"Robert Galbraith\",\"Robert Galbraith (J.K. Rowling)\"],\"type\":\"author\",\"name\":\"J. K. Rowling\",\"alternate_names\":[\"Joanne Rowling\",\"Joanne K. Rowling\",\"J.K.Rowling\",\"Rowling, J.K.\",\"J. Rowling\",\"Rowling, Joanne K.\",\"Jo Murray\",\"J K Rowling\",\"Rowling J.K.\",\"J.K. Rowling (author)\",\"Rowling Joanne\",\"J.K. Rowling\",\"J.K. ROWLING\",\"Rowling J K\",\"J K ROWLING\",\"Newt Scamander\",\"JOANNE K. ROWLING\",\"Kennilworthy Whisp\",\"JK Rowling\",\"JK Rowlings\",\"jk rowling\",\"R.K Rowling\",\"J. K Rowling\",\"J.K Rowling\",\"Rowling J. K.\",\"J. K. Rowling (Auteur)\",\"J.k. Rowling\",\"Rowling, J. K.\",\"ROWLING J.K. -\",\"J. K. ROWLING\",\"Rowling,J.K.\",\"J. K. Rowling\",\"Robert Galbraith\",\"Robert Galbraith (J.K. Rowling)\"],\"birth_date\":\"31 July 1965\",\"top_work\":\"Harry Potter and the Philosopher's Stone\",\"work_count\":224,\"top_subjects\":[\"Accessible book\",\"Protected DAISY\",\"Fiction\",\"Harry Potter\",\"Children's fiction\",\"Wizards, fiction\",\"England\",\"Wizards\",\"Magic\",\"Fantasy fiction\"],\"_version_\":1705097936033021952}";
    let actual = serde_json::from_str::<Author>(input)?;
    let output = serde_json::to_string(&actual)?;
    assert_eq!(input, output);
    Ok(())
}
