use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MetadataResponse {
    pub instance_id: String,
    pub location: String,
    pub name: String,
    pub os_type: String,
    pub zone: Option<String>,
}