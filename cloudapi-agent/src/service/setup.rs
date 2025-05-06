use std::{fs, path::Path};

#[cfg(windows)]
use std::{ffi::OsStr, os::windows::ffi::OsStrExt};

#[cfg(windows)]
use windows::core::PCWSTR;

#[cfg(windows)]
use windows::Win32::Storage::FileSystem::{SetFileAttributesW, FILE_ATTRIBUTE_HIDDEN};

use crate::config::AgentConfig;

pub fn create_default_config_file_if_missing(path: &str) -> anyhow::Result<()> {
    let file_path = Path::new(path);

    if !file_path.exists() {
        let config = serde_json::to_string_pretty(&AgentConfig::default())?; // Serialize to pretty JSON

        fs::create_dir_all(file_path.parent().unwrap())?; // Ensure parent directories exist
        fs::write(file_path, config)?;
        tracing::info!("Created default AgentConfig at {}", path);
    }

    Ok(())
}

pub fn create_application_data_dir(root: &str) -> anyhow::Result<String> {
    let agent_dir = format!("{}", root);
    fs::create_dir_all(&agent_dir)?; // Create the directory if it doesn't exist

    let extensions_dir = format!("{}\\extensions", agent_dir);
    let package_cache = format!("{}\\package-cache", agent_dir);

    fs::create_dir_all(&extensions_dir)?;
    fs::create_dir_all(&package_cache)?;

    #[cfg(windows)]
    set_hidden_attribute_windows(&agent_dir)?; // Set the directory as hidden on Windows

    tracing::info!("Created application data directory at {}", agent_dir);

    Ok(agent_dir)
}

#[cfg(windows)]
fn set_hidden_attribute_windows(path: &str) -> std::io::Result<()> {
    let result = unsafe {
        SetFileAttributesW(str_to_pcwstr(path), FILE_ATTRIBUTE_HIDDEN)
    };

    if result.is_ok() {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

#[cfg(windows)]
fn str_to_pcwstr(s: &str) -> PCWSTR {
    let wide: Vec<u16> = OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0)) // null-terminate
        .collect();

    PCWSTR(wide.as_ptr())
}
