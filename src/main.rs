mod app;
mod dto;
mod models;

use std::env;
use std::sync::Arc;

use crate::app::{handlers::pay_os_qr_handler::create_qr, service::pay_os_service::PayOsQrService};
use actix_web::{App, HttpServer, web};
use app::builder::builder::run;
use app::utils::logging::setup_tracing;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_tracing().expect("Failed to setup tracing");
    
    dotenvy::dotenv().ok();
    let host = env::var("APP_HOST")
        .unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("APP_PORT")
        .unwrap_or_else(|_| "8081".to_string())
        .parse()
        .expect("APP_PORT must be a valid u16");
    

    // Run core service in background
    let core_handle = tokio::spawn(async {
        if let Err(e) = run().await {
            tracing::error!("Core application crashed: {:?}", e);
        }
    });

    let qr_service = Arc::new(PayOsQrService::new());

    // Run HTTP server
    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(qr_service.clone()))
            .service(create_qr)
    })
    .bind((host.as_str(), port))?
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
