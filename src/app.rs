mod adapters;
mod api;
mod builder;
mod config;
mod core;
mod security;
mod state;
mod utils;

use utils::logging;

pub async fn run() -> anyhow::Result<()> {
    logging::setup_logging()?;
}