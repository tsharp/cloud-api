mod config;
mod constants;
mod service;
mod extension;
use anyhow::Result;
use service::run_service;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let dependency = assert_command_installed("pwsh").await;

    if dependency.is_err() {
        tracing::error!("Dependency check failed: {}", dependency.unwrap_err());
        return Err(anyhow::anyhow!("Dependency check failed"));
    }

    run_service().await?;

    Ok(())
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