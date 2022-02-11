use crate::clients::handle;
use crate::models::identifiers::{Identifier, OpenLibraryIdentifier};
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

    pub async fn get<'a, T: Into<&'a OpenLibraryIdentifier>>(
        &self,
        identifier: T,
    ) -> Result<Work, OpenLibraryError> {
        let url = self
            .host
            .join(format!("/works/{}.json", identifier.into().value()).as_str())?;

        handle(self.client.get(url)).await
    }
}
