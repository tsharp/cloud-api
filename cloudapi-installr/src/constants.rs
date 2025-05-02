#[cfg(windows)]
pub const DEFAULT_CLOUD_API_ROOT_DIR: &str = "C:\\cloud-api";

#[cfg(unix)]
pub const DEFAULT_CLOUD_API_ROOT_DIR: &str = "/var/lib/cloud-api";

pub const CLOUD_METADATA_V1_ENDPOINT: &str = "http://169.254.169.254";

#[allow(dead_code)]
pub const CLOUD_METADATA_V2_ENDPOINT: &str = "http://168.63.129.16";