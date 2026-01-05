mod app;
mod models;
mod dto;

use actix_web::{App, HttpServer};
use app::builder::builder::run;
use app::utils::logging::setup_tracing;
use crate::app::handlers::vietqr_handler::{create_qr, index};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_tracing().expect("Failed to setup tracing");

    // Run core service in background
    let core_handle = tokio::spawn(async {
        if let Err(e) = run().await {
            tracing::error!("Core application crashed: {:?}", e);
        }
    });

    // Run HTTP server
    let http_server = HttpServer::new(|| {
        App::new()
            .route("/health", actix_web::web::get().to(|| async { "OK" }))
            .service(create_qr)
            .service(index)
    })
    .bind(("127.0.0.1", 8081))?
    .run();

    tracing::info!("HTTP server started on 127.0.0.1:8080");

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
