
use cloudapi_sdk::model::extension::ExtensionState;
use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallrConfig {
    pub cloudapi_endpoint: String,
    pub package_cache: String,
    pub extensions: Vec<ExtensionState>,
}

impl InstallrConfig {
    pub fn default() -> Self {
        InstallrConfig {
            cloudapi_endpoint: constants::CLOUD_METADATA_V1_ENDPOINT.to_string(),
            package_cache: format!("{}\\package-cache", constants::DEFAULT_CLOUD_API_ROOT_DIR),
            extensions: vec![],
        }
    }

    pub fn get_cloudapi_endpoint(&self) -> &String {
        &self.cloudapi_endpoint
    }

    pub fn get_package_endpoint(&self) -> String {
        format!(
            "{}/{}",
            self.cloudapi_endpoint,
            "package"
        )
    }

    pub fn get_package_cache(&self) -> &String {
        &self.package_cache
    }

    pub fn get_extensions(&self) -> &Vec<ExtensionState> {
        &self.extensions
    }

    pub fn add_extension(&mut self, extension: ExtensionState) {
        self.extensions.push(extension);
    }

    pub fn remove_extension(&mut self, uid: &str) {
        self.extensions.retain(|ext| ext.uid != uid);
    }
}