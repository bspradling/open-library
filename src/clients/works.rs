use crate::clients::get;
use crate::models::identifiers::{Identifier, OpenLibraryIdentifer};
use crate::models::works::Work;
use crate::OpenLibraryError;
use reqwest::{Client, Url};

#[derive(Clone)]
pub struct WorksClient {
    client: Client,
    host: Url,
}

impl WorksClient {
    pub fn new(client: &Client, host: &Url) -> Self {
        Self {
            client: client.clone(),
            host: host.clone(),
        }
    }

    pub async fn get(&self, identifier: &OpenLibraryIdentifer) -> Result<Work, OpenLibraryError> {
        let path = format!("/works/{}.json", identifier.value());
        let url =
            self.host
                .join(path.as_str())
                .map_err(|error| OpenLibraryError::ParsingError {
                    reason: format!(
                        "Invalid URL from base url ({}) and path ({}): {}",
                        self.host, path, error
                    ),
                })?;

        get(self.client.clone(), url).await
    }
}
