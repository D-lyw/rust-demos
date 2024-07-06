use anyhow::Result;
use tokio::net::TcpListener;
use url_shortenr::{get_router, SERVICE_ADDR};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let listener = TcpListener::bind(SERVICE_ADDR).await?;
    let app_router = get_router().await?;
    axum::serve(listener, app_router).await?;

    Ok(())
}
