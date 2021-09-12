use crate::models::account::ReadingLogResponse;
use std::error::Error;

#[tokio::test]
pub async fn test_reading_log_response_serde() -> Result<(), Box<dyn Error>> {
    let input = include_str!("resources/account/all-fields.json");
    let actual = serde_json::from_str::<ReadingLogResponse>(input)?;
    let output = serde_json::to_string(&actual)?;
    // TODO figure out a better way to remove newlines from json instead removing all whitespace
    assert_eq!(
        input.split_whitespace().collect::<String>(),
        output.split_whitespace().collect::<String>()
    );
    Ok(())
}
