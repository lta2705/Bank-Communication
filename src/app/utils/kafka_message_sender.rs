use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};

/// Kafka Message Sender - Utility for sending messages to Kafka topics
pub struct KafkaMessageSender {
    producer: Arc<FutureProducer>,
}

impl KafkaMessageSender {
    pub fn new(producer: Arc<FutureProducer>) -> Self {
        Self { producer }
    }

    /// Send a message to Kafka topic
    ///
    /// # Arguments
    /// * `topic` - The Kafka topic name (will be auto-created if broker allows)
    /// * `key` - The message key (for partitioning)
    /// * `payload` - The message payload (must be serializable)
    ///
    /// # Note
    /// If the topic doesn't exist and the Kafka broker has `auto.create.topics.enable=true`
    /// (default setting), the topic will be automatically created on first send.
    pub async fn send<T: Serialize>(
        &self,
        topic: &str,
        key: &str,
        payload: &T,
    ) -> Result<(), String> {
        let payload_bytes = serde_json::to_vec(payload).map_err(|e| {
            error!("Failed to serialize payload: {:?}", e);
            format!("JSON serialize error: {:?}", e)
        })?;

        let record = FutureRecord::to(topic).payload(&payload_bytes).key(key);

        match self.producer.send(record, Duration::from_secs(30)).await {
            Ok(_) => {
                info!(
                    "Successfully sent message to Kafka topic '{}' with key '{}'",
                    topic, key
                );
                Ok(())
            }
            Err((e, _)) => {
                error!("Failed to send message to Kafka topic '{}': {:?}", topic, e);

                // Check if error is due to unknown topic
                let err_str = format!("{:?}", e);
                if err_str.contains("UnknownTopicOrPartition") {
                    warn!(
                        "Topic '{}' does not exist. If auto-create is enabled on broker, \
                        it will be created. Otherwise, create it manually using: \
                        kafka-topics.sh --create --topic {} --bootstrap-server <server>",
                        topic, topic
                    );
                }

                Err(format!("Kafka error: {:?}", e))
            }
        }
    }

    /// Send a message to Kafka topic with custom timeout
    ///
    /// # Arguments
    /// * `topic` - The Kafka topic name (will be auto-created if broker allows)
    /// * `key` - The message key (for partitioning)
    /// * `payload` - The message payload (must be serializable)
    /// * `timeout` - Custom timeout duration for sending
    pub async fn send_with_timeout<T: Serialize>(
        &self,
        topic: &str,
        key: &str,
        payload: &T,
        timeout: Duration,
    ) -> Result<(), String> {
        let payload_bytes = serde_json::to_vec(payload).map_err(|e| {
            error!("Failed to serialize payload: {:?}", e);
            format!("JSON serialize error: {:?}", e)
        })?;

        let record = FutureRecord::to(topic).payload(&payload_bytes).key(key);

        match self.producer.send(record, timeout).await {
            Ok(_) => {
                info!(
                    "Successfully sent message to Kafka topic '{}' with key '{}' (timeout: {:?})",
                    topic, key, timeout
                );
                Ok(())
            }
            Err((e, _)) => {
                error!("Failed to send message to Kafka topic '{}': {:?}", topic, e);

                // Check if error is due to unknown topic
                let err_str = format!("{:?}", e);
                if err_str.contains("UnknownTopicOrPartition") {
                    warn!(
                        "Topic '{}' does not exist. If auto-create is enabled on broker, \
                        it will be created. Otherwise, create it manually.",
                        topic
                    );
                }

                Err(format!("Kafka error: {:?}", e))
            }
        }
    }
}
