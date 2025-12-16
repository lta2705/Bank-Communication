use crate::app::{config, core, api, security};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let db_cfg = DatabaseCfg::new()
        .map_err(|e| format!("Database configuration error: {}"))?;
    
    let logger = setup_tracing()?;
    
    logger::info

    Ok(())
}