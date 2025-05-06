// #[cfg(windows)]
// pub mod windows;

// #[cfg(unix)]
// pub mod unix;
use anyhow::{Context, Result};
use chrono::Utc;
use cloudapi_sdk::client::CloudApiClient;
use cloudapi_sdk::model::extension::{ExtensionState, ExtensionStatus};
use sha2::{Digest, Sha256};
use tokio::{select, signal, process::Command};
use tokio_util::sync::CancellationToken;
use std::{fs, path::Path, path::PathBuf};
use std::fs::File;
use std::io::BufReader;
use crate::config::AgentConfig;
use crate::constants;
use crate::extension::ExtensionRunLog;
use zip::ZipArchive;

mod setup;

pub async fn run_service() -> Result<()> {
    // Setup cancellation token
    let cancel_token = CancellationToken::new();
    let shutdown_token = cancel_token.clone();

    // Create application data directory
    setup::create_application_data_dir(constants::DEFAULT_CLOUD_API_ROOT_DIR)?;

    // Example config file path
    let config_file = format!("{}/agent.config.json", constants::DEFAULT_CLOUD_API_ROOT_DIR);

    // Ensure config file exists
    setup::create_default_config_file_if_missing(config_file.as_str())?;

    // Spawn main polling task
    let poll_task = tokio::spawn(async move {
        poll_and_reconcile_config(config_file.as_str(), 15, cancel_token).await
    });

    // Spawn signal handler
    let shutdown_task = tokio::spawn(async move {
        wait_for_shutdown_signal().await?;
        println!("[shutdown] Signal received. Triggering shutdown...");
        shutdown_token.cancel();
        Ok::<_, anyhow::Error>(())
    });

    // Wait for either task to complete
    select! {
        res = poll_task => {
            println!("[service] Polling task completed: {:?}", res);
        },
        res = shutdown_task => {
            println!("[service] Shutdown handler completed: {:?}", res);
        }
    }

    Ok(())
}

pub async fn wait_for_shutdown_signal() -> Result<()> {
    #[cfg(unix)]
    {
        let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())?;
        let mut sigint = signal::unix::signal(signal::unix::SignalKind::interrupt())?;

        select! {
            _ = sigterm.recv() => {
                println!("[signal] Received SIGTERM");
            },
            _ = sigint.recv() => {
                println!("[signal] Received SIGINT (Ctrl+C)");
            },
        }
    }

    #[cfg(windows)]
    {
        signal::ctrl_c().await?;
        println!("[signal] Received Ctrl+C (Windows)");
    }

    Ok(())
}

async fn pull_latest_extension_states(config: &AgentConfig) -> Result<Vec<ExtensionState>> {
    CloudApiClient::new(config.get_cloudapi_endpoint())
        .get_extensions()
        .await
        .context("Failed to pull latest extension data")
}

async fn poll_and_reconcile_config(path: &str, interval_secs: u64, cancellation_token: CancellationToken) -> Result<()> {
    let path = Path::new(path).to_path_buf();
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(interval_secs));

    loop {
        if cancellation_token.is_cancelled() {
            tracing::info!("Cancellation token triggered. Exiting poll loop.");
            return Ok(());
        }

        interval.tick().await;

        match std::fs::read_to_string(&path) {
            Ok(contents) => {
                match serde_json::from_str::<AgentConfig>(&contents) {
                    Ok(config) => {
                        tracing::info!("Reloaded config. Starting reconciliation...");
                        let mut config = config;

                        let extension_states = pull_latest_extension_states(&config)
                            .await;

                        if extension_states.is_ok() {
                            tracing::info!("Updating config with latest extension states...");
                            
                            for state in extension_states.unwrap() {
                                config.remove_extension(state.uid.as_str());
                                config.add_extension(state);
                            }

                            // Save updated config
                            let updated_config = serde_json::to_string_pretty(&config)?;
                            fs::write(&path, updated_config)?;
                            tracing::info!("Updated config file with latest extension states.");
                        }
                        else
                        {
                            tracing::warn!("Failed to pull latest extension states: {:?}", extension_states);
                        }

                        reconcile_extensions(&config).await?;
                    }
                    Err(e) => {
                        tracing::error!("Failed to parse config file: {:?}", e);
                    }
                }
            }
            Err(e) => {
                tracing::error!("Failed to read config file: {:?}", e);
            }
        }
    }
}

async fn reconcile_extensions(config: &AgentConfig) -> Result<()> {
    for extension in config.get_extensions() {
        tracing::info!(">> Reconciling extension: {} (version: {})", extension.get_package_id(), extension.version);

        if needs_update(extension).await? {
            tracing::info!("Extension {} needs update or install.", extension.get_package_id());
            let result = install_or_update_extension(extension, config.get_package_endpoint().as_str(), config.get_package_cache()).await;
            
            if let Err(e) = result {
                tracing::error!("Failed to install/update extension {}: {:?}", extension.get_package_id(), e);
            } else {
                tracing::info!("Extension {} installed/updated successfully.", extension.get_package_id());
            }
        }
    }

    crate::extension::uninstall::uninstall_extensions(config).await?;

    tracing::info!("Reconciliation complete.");

    Ok(())
}



async fn needs_update(extension: &ExtensionState) -> Result<bool> {   
    match extension.status {
        ExtensionStatus::Uninstalling => {
            tracing::info!("Extension {} is currently uninstalling.", extension.get_package_id());
            return Ok(false);
        }
        ExtensionStatus::Uninstalled => {
            tracing::info!("Extension {} is currently uninstalled.", extension.get_package_id());
            return Ok(false);
        }
        ExtensionStatus::Disabled => {
            tracing::info!("Extension {} is currently disabled.", extension.get_package_id());
            return Ok(false);
        }
        _ => {}
    }

    let extension_dir = format!("C:\\cloud-api\\extensions\\{}", extension.get_package_id());
    let version_file = PathBuf::from(&extension_dir).join("VERSION");

    if !version_file.exists() {
        tracing::info!("Extension {} not installed.", extension.get_package_id());
        return Ok(true);
    }

    let current_version_hash = fs::read_to_string(version_file)?.trim().to_string();
    let version_hash = hash_extension_state(extension)?;

    if current_version_hash != version_hash {
        tracing::info!("Extension {} version mismatch: current {}, desired {}", extension.get_package_id(), current_version_hash, version_hash);
        return Ok(true);
    }

    Ok(false)
}

pub async fn install_or_update_extension(extension: &ExtensionState, endpoint: &str, cache_dir: &str) -> Result<()> {
    let extension_pkg = format!("{}-{}.extpkg", extension.get_package_id(), extension.version);
    let package_path = download_package(endpoint, &extension_pkg, cache_dir).await?;

    let target_dir = format!(
        "{}\\extensions\\{}\\v{}",
        constants::DEFAULT_CLOUD_API_ROOT_DIR,
        extension.get_package_id(),
        extension.version
    );

    extract_package(&package_path, &target_dir).await?;

    // === One-off PowerShell execution ===
    let ps_script = Path::new(&target_dir).join("install.ps1");
    let ran_marker = Path::new(&target_dir).join("ran.lock");

    if ps_script.exists() && !ran_marker.exists() {
        let output = Command::new("pwsh")
            .arg("-NoProfile")
            .arg("-ExecutionPolicy").arg("Bypass")
            .arg("-File").arg(ps_script)
            .output()
            .await
            .context("Failed to execute PowerShell script")?;

        let log = ExtensionRunLog {
            executed_at: Utc::now().to_rfc3339(),
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        };

        // Save run-log.json
        let log_path = Path::new(&target_dir).join("run-log.json");
        fs::write(&log_path, serde_json::to_vec_pretty(&log)?)?;

        // Mark script as executed
        fs::write(&ran_marker, log.executed_at.as_bytes())?;

        // Optional: error if script failed
        if log.exit_code != 0 {
            return Err(anyhow::anyhow!("PowerShell script failed with code {}", log.exit_code));
        }
    }

    // Write version marker
    let version_path = PathBuf::from(format!(
        "{}\\extensions\\{}",
        constants::DEFAULT_CLOUD_API_ROOT_DIR,
        extension.get_package_id()
    )).join("VERSION");

    let version_hash = hash_extension_state(extension)?;

    fs::write(&version_path, version_hash)?;

    Ok(())
}

async fn download_package(endpoint: &str, package_name: &str, cache_dir: &str) -> Result<PathBuf> {
    let url = format!("{}/{}", endpoint.trim_end_matches('/'), package_name);
    let dest_path = PathBuf::from(cache_dir).join(package_name);

    tracing::info!("Downloading package from: {:?}", dest_path);

    if dest_path.exists() {
        tracing::info!("Package already downloaded: {:?}", dest_path);
        return Ok(dest_path);
    }

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let bytes = response.bytes().await?;

    fs::create_dir_all(cache_dir)?;
    fs::write(&dest_path, &bytes)?;

    Ok(dest_path)
}

async fn extract_package(zip_path: &PathBuf, dest_dir: &str) -> Result<()> {
    let file = File::open(zip_path)?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = PathBuf::from(dest_dir).join(file.name());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

fn hash_extension_state(spec: &ExtensionState) -> Result<String> {
    // Canonical JSON serialization
    let json = serde_json::to_string(spec)?;

    // Hash it
    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    let hash = hasher.finalize();

    // Return as lowercase hex string
    Ok(format!("{:x}", hash))
}