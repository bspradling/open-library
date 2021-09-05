use crate::OpenLibraryError;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Deserialize)]
pub struct Session {
    pub cookie: String,
}

impl TryFrom<Response> for Session {
    type Error = OpenLibraryError;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        let cookie = response
            .headers()
            .get(http::header::SET_COOKIE)
            .ok_or(OpenLibraryError::ParsingError {
                reason: "The API response from Open Library did not include a Set-Cookie header"
                    .to_string(),
            })?
            .to_str()
            .map_err(|_e| OpenLibraryError::ParsingError {
                reason: "Unable to parse Set-Cookie Header Value into String".to_string(),
            })?
            .to_string();

        Ok(Self { cookie })
    }
}
