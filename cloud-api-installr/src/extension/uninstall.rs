use anyhow::{Context, Result};
use tokio::process::Command;
use std::path::PathBuf;
use std::{fs, path::Path};
use crate::config::{ExtensionState, ExtensionStatus, InstallrConfig};
use crate::constants;

pub async fn uninstall_extensions(config: &InstallrConfig) -> Result<()> {
    for state in config.get_extensions() 
    {
        if state.status == ExtensionStatus::Uninstalling 
        {
            uninstall_extension(state).await?;
        }
    }

    clean_extension_dir(config)?;

    Ok(())
}

fn get_extension_uninstall_script_path(state: &ExtensionState) -> Result<Option<PathBuf>> {
    let versioned_ext_dir = format!("{}\\extensions\\{}\\v{}", constants::DEFAULT_CLOUD_API_ROOT_DIR, state.get_package_id(), state.version);
    let extension_spec_path = Path::new(&versioned_ext_dir).join("extension.spec");

    tracing::info!("Looking for extension spec file: {}", extension_spec_path.to_string_lossy());
    let spec_contents = std::fs::read_to_string(&extension_spec_path);

    if spec_contents.is_err() {
        tracing::warn!("Extension spec file not found: {}", extension_spec_path.to_string_lossy());
        return Ok(Option::None);
    }

    tracing::info!("Found extension spec file: {}", extension_spec_path.to_string_lossy());
    let extension_spec: crate::extension::ExtensionSpec = serde_json::from_str(&spec_contents.unwrap())
        .context(format!("Failed to parse extension spec file: {}", extension_spec_path.to_string_lossy()))?;
   
    let ext_uninstall_script = extension_spec.uninstall_script.clone();

    tracing::info!("Parsed extension spec: ...");
    if ext_uninstall_script.is_some() && !ext_uninstall_script.as_ref().unwrap().is_empty() {
        let spec_uninstall_script = Path::new(&versioned_ext_dir).join(ext_uninstall_script.clone().unwrap());

        if spec_uninstall_script.exists() {
            return Ok(Some(spec_uninstall_script));
        } else {
            tracing::warn!("Extension defined an uninstall script that was not found: {}", ext_uninstall_script.clone().unwrap());
        }
    }

    let implicit_uninstall_script = Path::new(&versioned_ext_dir).join("uninstall.ps1");

    if implicit_uninstall_script.exists() {
        tracing::info!("Found implicit uninstall script: {}", implicit_uninstall_script.to_string_lossy());
        return Ok(Some(implicit_uninstall_script));
    }

    tracing::warn!("No uninstall script found for extension: {}", state.get_package_id());
    Ok(Option::None)
}

async fn uninstall_extension(state: &ExtensionState) -> Result<()> {
    let ext_dir = format!("{}\\extensions\\{}", constants::DEFAULT_CLOUD_API_ROOT_DIR, state.get_package_id());
    tracing::info!("Uninstalling extension: {}", ext_dir);

    if !Path::new(&ext_dir).exists() {
        tracing::warn!("Extension {} not found for uninstallation.", state.get_package_id());
        return Ok(());
    }

    // === One-off PowerShell execution ===
    tracing::info!("Looking for uninstall script for extension: {}", state.get_package_id());
    let ps_script = get_extension_uninstall_script_path(state)?;

    if ps_script.is_some() {
        tracing::info!("Running uninstall script for extension: {}", state.get_package_id());
        let output = Command::new("pwsh")
            .arg("-NoProfile")
            .arg("-ExecutionPolicy").arg("Bypass")
            .arg("-File").arg(ps_script.unwrap())
            .output()
            .await;

        if output.is_err() {
            tracing::error!("Failed to execute uninstall script: {:?}", output.err().unwrap());
            return Err(anyhow::anyhow!("Failed to execute uninstall script"));
        }

        tracing::info!("Uninstall script output: {:?}", output);
    }
    else {
        tracing::info!("No uninstall script found for extension: {}", state.get_package_id());
    }

    if fs::remove_dir_all(&ext_dir).is_err() {
        tracing::error!("Failed to remove extension directory: {}", ext_dir);
    } else {
        tracing::info!("Extension {} uninstalled successfully.", state.get_package_id());
    }

    Ok(())
}

/**
 * Removes any extensions that are not in the config but are present in the extensions directory.
 * This is useful for cleaning up old extensions that are no longer needed.
 * 
 * Note this function does not call the uninstall script for the extensions, it simply removes the directory.
 */
fn clean_extension_dir(config: &InstallrConfig) -> Result<()> {
    // Check for any extensions that are not in the config but are installed
    let installed_extensions: Vec<String> = fs::read_dir(format!("{}\\extensions", constants::DEFAULT_CLOUD_API_ROOT_DIR))?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();

    let config_extensions: Vec<String> = config.get_extensions().iter()
        .map(|ext| ext.get_package_id())
        .collect();

    let extensions_to_remove: Vec<String> = installed_extensions.into_iter()
        .filter(|ext| !config_extensions.contains(ext))
        .collect();

    for ext in extensions_to_remove {
        let ext_dir = format!("{}\\extensions\\{}", constants::DEFAULT_CLOUD_API_ROOT_DIR, ext);
        tracing::info!("Removing extension: {}", ext_dir);

        if fs::remove_dir_all(&ext_dir).is_err() {
            tracing::error!("Failed to remove extension directory: {}", ext_dir);
        } else {
            tracing::info!("Extension {} removed successfully.", ext);
        }
    }

    Ok(())
}