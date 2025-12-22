use crate::app::error::AppError;
use crate::app::utils::database::establish_db_conn;
use crate::app::utils::connection_initializer::{
    TcpServer, ConnectionMode,
};
use crate::app::config::connection_config::ConnAttr;
use tracing::{info, debug, error, warn};

pub async fn run() -> Result<(), AppError> {
    
    // 1. Init DB pool
    let db_pool = establish_db_conn()
        .await
        .map_err(AppError::Database)?;

    info!("Database connection established");

    let db_pool = Arc::new(db_pool);
    
    let conn_cfg = ConnAttr::load_env()
        .map_err(AppError::Config)?;

    let tcp_address = format!("{}:{}", conn_cfg.host, conn_cfg.port);

    let tcp_server = TcpServer::new(
        tcp_address,
        ConnectionMode::Plain,
    );

    let tcp_handle = tokio::spawn(async move {
        tcp_server.start().await
    });

    info!("Application started");

    tokio::select! {
        res = tcp_handle => {
            res
                .map_err(AppError::TaskJoin)?
                .map_err(AppError::Io)?;
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Shutdown signal received");
        }
    }

    Ok(())
}
