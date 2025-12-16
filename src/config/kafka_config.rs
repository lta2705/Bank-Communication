use std::env;
use dotenvy::from_filename;
use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct KafkaConfig {
    // ===== Common =====
    pub bootstrap_servers: String,

    // ===== Producer =====
    pub producer_topic: String,
    pub compression_type: String,
    pub acks: String,
    pub retries: i32,
    pub linger_ms: i32,
    pub max_in_flight: i32,
    pub enable_idempotence: bool,
    pub request_timeout_ms: i32,
    pub delivery_timeout_ms: i32,

    // ===== Consumer =====
    pub consumer_topic: String,
    pub group_id: String,
    pub enable_auto_commit: bool,
    pub max_poll_interval_ms: i32,
    pub session_timeout_ms: i32,
    pub heartbeat_interval_ms: i32,
    pub isolation_level: String,
    pub auto_offset_reset: String,
}

impl KafkaConfig {
    pub fn from_env() -> Result<Self> {
        from_filename("../kafka_cfg.env")
            .context("Failed to load kafka_cfg.env")?;

        Ok(Self {
            // ===== Common =====
            bootstrap_servers: env::var("KAFKA_BOOTSTRAP_SERVERS")
                .context("KAFKA_BOOTSTRAP_SERVERS is required")?,

            // ===== Producer =====
            producer_topic: env::var("KAFKA_PRODUCER_TOPIC")
                .context("KAFKA_PRODUCER_TOPIC is required")?,
            compression_type: env::var("KAFKA_PRODUCER_COMPRESSION")?,
            acks: env::var("KAFKA_PRODUCER_ACKS")?,
            retries: env::var("KAFKA_PRODUCER_RETRIES")?
                .parse()
                .context("KAFKA_PRODUCER_RETRIES must be i32")?,
            linger_ms: env::var("KAFKA_PRODUCER_LINGER_MS")?
                .parse()
                .context("KAFKA_PRODUCER_LINGER_MS must be i32")?,
            max_in_flight: env::var("KAFKA_MAX_IN_FLIGHT")?
                .parse()
                .context("KAFKA_MAX_IN_FLIGHT must be i32")?,
            enable_idempotence: env::var("KAFKA_ENABLE_IDEMPOTENCE")?
                .parse()
                .context("KAFKA_ENABLE_IDEMPOTENCE must be bool")?,
            request_timeout_ms: env::var("KAFKA_REQUEST_TIMEOUT_MS")?
                .parse()
                .context("KAFKA_REQUEST_TIMEOUT_MS must be i32")?,
            delivery_timeout_ms: env::var("KAFKA_DELIVERY_TIMEOUT_MS")?
                .parse()
                .context("KAFKA_DELIVERY_TIMEOUT_MS must be i32")?,

            // ===== Consumer =====
            consumer_topic: env::var("KAFKA_CONSUMER_TOPIC")
                .context("KAFKA_CONSUMER_TOPIC is required")?,
            group_id: env::var("KAFKA_CONSUMER_GROUP_ID")?,
            enable_auto_commit: env::var("KAFKA_ENABLE_AUTO_COMMIT")?
                .parse()
                .context("KAFKA_ENABLE_AUTO_COMMIT must be bool")?,
            max_poll_interval_ms: env::var("KAFKA_MAX_POLL_INTERVAL_MS")?
                .parse()
                .context("KAFKA_MAX_POLL_INTERVAL_MS must be i32")?,
            session_timeout_ms: env::var("KAFKA_SESSION_TIMEOUT_MS")?
                .parse()
                .context("KAFKA_SESSION_TIMEOUT_MS must be i32")?,
            heartbeat_interval_ms: env::var("KAFKA_HEARTBEAT_INTERVAL_MS")?
                .parse()
                .context("KAFKA_HEARTBEAT_INTERVAL_MS must be i32")?,
            isolation_level: env::var("KAFKA_ISOLATION_LEVEL")?,
            auto_offset_reset: env::var("KAFKA_AUTO_OFFSET_RESET")?,
        })
    }
}

