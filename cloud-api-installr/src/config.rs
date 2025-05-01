
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallrConfig {
    pub package_endpoint: String,
    pub package_cache: String,
    pub extensions: Vec<ExtensionState>,
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
pub struct ExtensionState {
  pub uid: String,
  pub id: String,
  pub publisher: Option<String>,
  pub version: String,
  pub config: Option<String>,
  pub status: ExtensionStatus,
  pub modified_at: String,
}

impl  ExtensionState {
    pub fn new(uid: &str, id: &str, version: &str) -> Self {
        ExtensionState {
            uid: uid.to_string(),
            id: id.to_string(),
            publisher: None,
            version: version.to_string(),
            modified_at: Utc::now().to_rfc3339(),
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