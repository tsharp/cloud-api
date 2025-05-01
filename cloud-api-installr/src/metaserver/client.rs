use reqwest::Client;
use crate::config::ExtensionState;

use super::error::MetaServerError;
use super::model::MetadataResponse;

#[derive(Debug, Clone)]
pub struct MetaServerClient {
    endpoint: String,
    client: Client,
}

impl MetaServerClient {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            client: Client::new(),
        }
    }

    fn get_metadata_url(&self) -> String {
        format!("{}/cloud-api/v1/metadata", self.endpoint)
    }

    pub async fn get_metadata(&self) -> Result<MetadataResponse, MetaServerError> {
        let url = format!("{}", self.get_metadata_url());
        let res = self.client
            .get(&url)
            .header("Metadata", "true")
            .send()
            .await?
            .error_for_status()? // HTTP-level error
            .json::<MetadataResponse>()
            .await?;

        Ok(res)
    }

    pub async fn get_extensions(&self) -> Result<Vec<ExtensionState>, MetaServerError> {
        let url = format!("{}/extensions", self.get_metadata_url());
        let res = self.client
            .get(&url)
            .header("Metadata", "true")
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<ExtensionState>>()
            .await?;

        Ok(res)
    }
}
