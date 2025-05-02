
use cloudapi_sdk::model::extension::ExtensionState;
use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallrConfig {
    pub package_endpoint: String,
    pub package_cache: String,
    pub extensions: Vec<ExtensionState>,
}

impl InstallrConfig {
    pub fn default() -> Self {
        InstallrConfig {
            package_endpoint: format!("{}/packages", constants::CLOUD_METADATA_V1_ENDPOINT),
            package_cache: format!("{}\\package-cache", constants::DEFAULT_CLOUD_API_ROOT_DIR),
            extensions: vec![],
        }
    }

    pub fn get_package_endpoint(&self) -> &String {
        &self.package_endpoint
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