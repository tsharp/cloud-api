pub mod uninstall;
pub mod install;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionRunLog {
    pub executed_at: String,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionSpec {
    pub id: String,
    pub publisher: String,
    pub version: String,
    pub description: Option<String>,
    pub install_script: Option<String>,
    pub uninstall_script: Option<String>,
    pub config_schema: Option<String>,
    pub one_time_script: Option<String>,
}