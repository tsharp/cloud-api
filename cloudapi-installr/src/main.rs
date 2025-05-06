mod config;
mod constants;
mod service;
mod extension;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    if !check_system_configuration() {
        tracing::warn!("Service configuration not found.");
        tracing::info!("Please run the installer to set up the service.");
        tracing::info!("Press Ctrl-C to exit.");
        
        service::wait_for_shutdown_signal().await;
        tracing::info!("Shutdown signal received. Exiting...");

        return Ok(());
    }

    let dependency = assert_command_installed("pwsh").await;

    if dependency.is_err() {
        tracing::error!("Dependency check failed: {}", dependency.unwrap_err());
        return Err(anyhow::anyhow!("Dependency check failed"));
    }

    service::run_service().await?;

    Ok(())
}

fn check_system_configuration() -> bool {
    let config_file = format!("{}/installr.config.json", constants::DEFAULT_CLOUD_API_ROOT_DIR);
    tracing::info!("Checking for service configuration at: {}", config_file);

    if std::path::Path::new(&config_file).exists() {
        tracing::info!("Service configuration found.");
        return true;
    }

    tracing::warn!("Service configuration not found.");

    return false;
}

async fn assert_command_installed(cmd: &str) -> Result<()> {
    tracing::info!("Checking if required command is present: {}", cmd);

    let output = tokio::process::Command::new(cmd)
        .arg("--version")
        .output()
        .await;

    if output.is_ok() && output.unwrap().status.success() {
        tracing::info!("Command is installed: {}", cmd);
    } else {
        tracing::error!("Command {} is not installed", cmd);
        return Err(anyhow::anyhow!("Command {} is not installed", cmd));
    }

    Ok(())
}