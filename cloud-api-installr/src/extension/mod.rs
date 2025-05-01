use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExtensionRunLog {
    pub executed_at: String,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExtensionPackageManifest {
    pub id: String,
    pub publisher: String,
    pub version: String,
    pub description: Option<String>,
    pub install_script: String,
    pub uninstall_script: String,
    pub config_schema: Option<String>
}