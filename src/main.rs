mod app;
mod dto;
mod models;

use std::sync::Arc;

use crate::app::{
    handlers::pay_os_qr_handler::create_qr, service::pay_os_service::VietQrService,
};
use actix_web::{App, HttpServer, web};
use app::builder::builder::run;
use app::utils::logging::setup_tracing;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_tracing().expect("Failed to setup tracing");

    // Run core service in background
    let core_handle = tokio::spawn(async {
        if let Err(e) = run().await {
            tracing::error!("Core application crashed: {:?}", e);
        }
    });

    let qr_service = Arc::new(VietQrService::new());

    // Run HTTP server
    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(qr_service.clone()))
            .service(create_qr)
    })
    .bind(("0.0.0.0", 8081))?
    .run();

    tracing::info!("HTTP server started on 0.0.0.0:8081");

    // 3. Orchestrate lifecycle
    tokio::select! {
        res = http_server => {
            res?;
        }

        _ = core_handle => {
            tracing::error!("Core service stopped unexpectedly");
        }

        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Shutdown signal received");
        }
    }

    tracing::info!("Application shutdown complete");
    Ok(())
}
