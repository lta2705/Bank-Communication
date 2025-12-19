use thiserror::Error;

use crate::app::error;

#[derive(Debug,Error)]
pub enum AppError {
    #[error("database configuration error")]
    DatabaseConfig(#[from] DatabaseCfgError),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("logging initialization failed")]
    Logging(#[from] tracing::subscriber::SetGlobalDefaultError),
}