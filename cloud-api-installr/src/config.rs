
use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Serialize, Deserialize)]
pub struct ExtensionRunLog {
    pub executed_at: String,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallrConfig {
    pub package_endpoint: String,
    pub package_cache: String,
    pub extensions: Vec<ExtensionSpec>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionSpec {
    pub id: Option<String>,
    pub publisher: Option<String>,
    pub name: String,
    pub version: String,
    pub uninstall: Option<bool>,
    pub is_service: bool,
    pub is_enabled: bool,
    pub timestamp: String,
    pub config: Option<String>,
    pub description: Option<String>,
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

    pub fn get_extensions(&self) -> &Vec<ExtensionSpec> {
        &self.extensions
    }
}