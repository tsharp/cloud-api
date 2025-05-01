
use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallrConfig {
    pub package_endpoint: String,
    pub package_cache: String,
    pub extensions: Vec<ExtensionSpec>,
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionStatus {
    NotInstalled,
    Installing,
    Installed,
    Uninstalling,
    Uninstalled,
    Failed,
    Disabled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionSpec {
  pub id: String,
  pub publisher: Option<String>,
  pub version: String,
  pub timestamp: String,
  pub config: Option<String>,
  pub status: ExtensionStatus,
}

impl  ExtensionSpec {
    pub fn new(id: &str, version: &str, timestamp: &str) -> Self {
        ExtensionSpec {
            id: id.to_string(),
            publisher: None,
            version: version.to_string(),
            timestamp: timestamp.to_string(),
            config: None,
            status: ExtensionStatus::NotInstalled,
        }
    }

    pub fn set_publisher(&mut self, publisher: &str) {
        self.publisher = Some(publisher.to_string());
    }

    pub fn set_config(&mut self, config: &str) {
        self.config = Some(config.to_string());
    }

    pub fn set_status(&mut self, status: ExtensionStatus) {
        self.status = status;
    }

    pub fn get_package_id(&self) -> String {
        let package_id: String = format!("{}-{}", self.publisher.as_ref().unwrap_or(&"none".to_string()), self.id);

        return package_id;
    }
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