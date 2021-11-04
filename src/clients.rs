use crate::models::OpenLibraryModel;
use crate::OpenLibraryError;
use http::StatusCode;
use reqwest::Client;
use serde::Deserialize;
use url::Url;

pub mod account;
pub mod author;
pub mod books;
pub mod works;

#[cfg(test)]
mod tests;

pub async fn get<T>(client: Client, url: Url) -> Result<T, OpenLibraryError>
where
    T: for<'de> Deserialize<'de> + OpenLibraryModel,
{
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|error| OpenLibraryError::RequestFailed { source: error })?;

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
