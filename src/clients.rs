use crate::models::OpenLibraryModel;
use crate::OpenLibraryError;
use http::StatusCode;
use reqwest::RequestBuilder;
use serde::Deserialize;

pub mod account;
pub mod author;
pub mod books;
pub mod works;

#[cfg(test)]
mod tests;

pub async fn handle<T>(request: RequestBuilder) -> Result<T, OpenLibraryError>
where
    T: for<'de> Deserialize<'de> + OpenLibraryModel,
{
    let response = request.send().await?;

    return match response.status() {
        StatusCode::OK => Ok(response
            .json::<T>()
            .await
            .map_err(|error| OpenLibraryError::JsonParseError { source: error })?),
        _ => Err(OpenLibraryError::ApiError {
            status_code: response.status(),
            error: None,
        }),
    };
}
