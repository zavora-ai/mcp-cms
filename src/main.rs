mod client;
mod server;

use client::CmsBackend;
use rmcp::{ServiceExt, transport::stdio};
use server::CmsServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).init();
    let backend = CmsBackend::from_env()?;
    let service = CmsServer { backend }.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
