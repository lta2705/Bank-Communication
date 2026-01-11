use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

/// Application-wide error type
#[derive(Debug, Error)]
pub enum AppError {
    // ========================
    // Infrastructure / Config
    // ========================

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Logging initialization failed: {0}")]
    Logging(#[from] tracing::subscriber::SetGlobalDefaultError),

    #[error("Kafka configuration error: {0}")]
    KafkaConfig(#[from] anyhow::Error),

    // ========================
    // Network / External
    // ========================

    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Network error: {0}")]
    Network(String),

    #[error("External service error: {0}")]
    ExternalService(String),

    // ========================
    // Runtime / Async
    // ========================

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Task join error: {0}")]
    TaskJoin(#[from] tokio::task::JoinError),

    // ========================
    // Business / Validation
    // ========================

    #[error("Validation error: {0}")]
    Validation(String),

    // ========================
    // Fallback
    // ========================

    #[error("Internal error: {0}")]
    Internal(String),
}
#[derive(Serialize)]
struct ErrorResponse {
    code: String,
    message: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, code, message) = match self {
            // Client errors
            AppError::Validation(msg) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR",
                msg.clone(),
            ),

            // External / Network
            AppError::Http(_) | AppError::Network(_) | AppError::ExternalService(_) => (
                actix_web::http::StatusCode::BAD_GATEWAY,
                "EXTERNAL_SERVICE_ERROR",
                self.to_string(),
            ),
            
            AppError::KafkaConfig(_) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "KAFKA_CONFIG_ERROR",
                self.to_string(),
            ),

            // Infra / Config
            AppError::Config(_) | AppError::Database(_) | AppError::Logging(_) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "CONFIG_ERROR",
                "Server configuration error".to_string(),
            ),

            // Runtime
            AppError::Io(_) | AppError::TaskJoin(_) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "RUNTIME_ERROR",
                self.to_string(),
            ),

            // Fallback
            AppError::Internal(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                msg.clone(),
            ),
        };

        HttpResponse::build(status).json(ErrorResponse {
            code: code.to_string(),
            message,
        })
    }
}
