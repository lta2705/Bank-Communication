use crate::app::utils::{connection_handler,kafka_consumer,kafka_producer,logging};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let db_cfg = DatabaseCfg::new()
        .map_err(|e| format!("Database configuration error: {}"))?;
    
    let logger = setup_tracing()?;
    
    Ok(())
}