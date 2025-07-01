
use rmcp::transport::sse_server::SseServer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use rmcp::{ServiceExt, transport::stdio};

use tracing_subscriber::{self};
use crayon_mcp::CrayonMcpServer;


#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
    .with_writer(std::io::stderr)
    .with_ansi(false)
    .init();

tracing::info!("Starting MCP server");

// Create an instance of our counter router
let service = CrayonMcpServer::new().serve(stdio()).await.inspect_err(|e| {
    tracing::error!("serving error: {:?}", e);
})?;

service.waiting().await?;
Ok(())
}
