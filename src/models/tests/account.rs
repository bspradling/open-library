use crate::models::account::ReadingLogResponse;
use std::error::Error;

#[tokio::test]
pub async fn test_reading_log_response_serde() -> Result<(), Box<dyn Error>> {
    let input = "{\"page\":1,\"reading_log_entries\":[{\"work\":{\"title\":\"AtomicHabits\",\"key\":\"/works/OL17930368W\",\"author_keys\":[\"/authors/OL7422948A\"],\"author_names\":[\"JamesClear\"],\"first_publish_year\":2018,\"lending_edition_s\":null,\"edition_key\":[\"OL32336498M\",\"OL27918581M\",\"OL26502528M\",\"OL31936190M\",\"OL30515574M\",\"OL30515575M\",\"OL31844060M\"],\"cover_id\":null,\"cover_edition_key\":\"OL32336498M\"},\"logged_edition\":\"/books/OL32336498M\",\"logged_date\":\"2021/09/06,00:44:47\"}]}";
    let actual = serde_json::from_str::<ReadingLogResponse>(input)?;
    let output = serde_json::to_string(&actual)?;
    assert_eq!(input, output);
    Ok(())
}
