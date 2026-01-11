mod app;
mod dto;
mod models;
mod repository;

use std::env;
use std::sync::Arc;

use crate::app::handlers::pay_os_qr_handler::index;
use crate::app::{handlers::pay_os_qr_handler::create_qr, service::pay_os_service::PayOsQrService};
use actix_web::{App, HttpServer, web};
use app::builder::builder::run;
use app::utils::logging::setup_tracing;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_tracing().expect("Failed to setup tracing");
    dotenvy::dotenv().ok();

    let host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("APP_PORT")
        .unwrap_or_else(|_| "8081".to_string())
        .parse()
        .expect("APP_PORT must be a valid u16");

    let core_handle = tokio::spawn(async {
        if let Err(e) = run().await {
            tracing::error!("Core application crashed: {:?}", e);
        }
    });

    // 1. Khởi tạo service
    let qr_service = PayOsQrService::new();
    let qr_service_data = web::Data::new(qr_service);

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(qr_service_data.clone())
            .service(create_qr)
            .route("/", web::get().to(index))
    })
    .bind((host.as_str(), port))?
    .run();

    tracing::info!("HTTP server started on {}:{}", host, port);

    tokio::select! {
        res = http_server => { res?; }
        _ = core_handle => { tracing::error!("Core service stopped unexpectedly"); }
        _ = tokio::signal::ctrl_c() => { tracing::info!("Shutdown signal received"); }
    }

    Ok(())
}
