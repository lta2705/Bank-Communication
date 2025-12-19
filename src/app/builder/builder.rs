use crate::app::error::AppError;
use crate::app::utils::connection_initializer::ConnectionMode;
use crate::app::utils::database::establish_conn;
use crate::app::utils::{connection_initializer::run_server, connection_handler::Connection};
use crate::app::config::connection_config::ConnAttr;
use tracing::{info, debug, error, warn};

pub async fn run() -> Result<(), AppError> {
    establish_conn().await?;
    
    let conn_cfg = ConnAttr::load_env()
        .map_err(AppError::Config)?;
    
    info!("Connection configured: {}, {}", conn_cfg.host, conn_cfg.port);
    
    let tcp_address = format!("{}:{}", conn_cfg.host, conn_cfg.port);
    info!("{}", tcp_address);
    
    let tls_address = format!("{}:{}", conn_cfg.host, conn_cfg.tls_port);
    
    let tcp_server = run_server(tcp_address.as_str(), ConnectionMode::Plain);
    
    // let tls_server = run_server(tls_address.as_str(), ConnectionMode::Tls(()));
    
    return Ok(());
}