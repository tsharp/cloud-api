mod config;
mod constants;
mod service;

use anyhow::Result;
use service::run_service;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    run_service().await?;

    Ok(())
}
