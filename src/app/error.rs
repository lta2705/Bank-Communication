use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database configuration error")]
    Database(sqlx::Error),
    
    #[error("Kafka error: {0}")]
    Kafka(#[from] rdkafka::error::KafkaError),
    
    #[error("I/O error")]
    Io(std::io::Error),
    
    #[error("Task join error")]
    TaskJoin(tokio::task::JoinError),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Kafka configuration error: {0}")]
    KafkaConfig(#[from] anyhow::Error),
    
    #[error("logging initialization failed")]
    Logging(#[from] tracing::subscriber::SetGlobalDefaultError),
}