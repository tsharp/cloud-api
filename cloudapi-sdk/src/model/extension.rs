use chrono::Utc;
use serde::{Deserialize, Serialize};

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