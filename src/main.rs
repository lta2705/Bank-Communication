mod app;
mod models;
use app::builder::builder::run;
use app::utils::logging::setup_tracing;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_tracing()?;
    run().await?;
    tracing::info!("Application started");
    Ok(())
}

