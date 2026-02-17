use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::config::ClientConfig;
use std::time::Duration;
use tracing::{error, info, warn};

/// Kafka Topic Manager - Utility for managing Kafka topics
pub struct KafkaTopicManager {
    admin_client: AdminClient<DefaultClientContext>,
}

impl KafkaTopicManager {
    /// Create a new KafkaTopicManager
    ///
    /// # Arguments
    /// * `bootstrap_servers` - Kafka broker addresses (e.g., "localhost:9092")
    pub fn new(bootstrap_servers: &str) -> anyhow::Result<Self> {
        let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
            .set("bootstrap.servers", bootstrap_servers)
            .create()?;

        Ok(Self { admin_client })
    }

    /// Create a topic if it doesn't exist
    ///
    /// # Arguments
    /// * `topic_name` - Name of the topic to create
    /// * `num_partitions` - Number of partitions (default: 3)
    /// * `replication_factor` - Replication factor (default: 1)
    pub async fn create_topic_if_not_exists(
        &self,
        topic_name: &str,
        num_partitions: i32,
        replication_factor: i32,
    ) -> Result<(), String> {
        info!(
            "Attempting to create topic '{}' with {} partitions and replication factor {}",
            topic_name, num_partitions, replication_factor
        );

        let new_topic = NewTopic::new(
            topic_name,
            num_partitions,
            TopicReplication::Fixed(replication_factor),
        );

        let opts = AdminOptions::new().request_timeout(Some(Duration::from_secs(5)));

        match self.admin_client.create_topics(&[new_topic], &opts).await {
            Ok(results) => {
                if let Some(result) = results.into_iter().next() {
                    match result {
                        Ok(topic) => {
                            info!("Successfully created topic: {}", topic);
                            return Ok(());
                        }
                        Err((topic, err_code)) => {
                            // Check if error is "topic already exists" - this is OK
                            if format!("{:?}", err_code).contains("TopicAlreadyExists") {
                                info!("Topic '{}' already exists, skipping creation", topic);
                                return Ok(());
                            } else {
                                error!("Failed to create topic '{}': {:?}", topic, err_code);
                                return Err(format!("Topic creation failed: {:?}", err_code));
                            }
                        }
                    }
                }
                Ok(())
            }
            Err(e) => {
                error!("Failed to create topic '{}': {:?}", topic_name, e);
                Err(format!("Admin client error: {:?}", e))
            }
        }
    }

    /// Create multiple topics at once
    ///
    /// # Arguments
    /// * `topics` - List of (topic_name, num_partitions, replication_factor)
    pub async fn create_topics_batch(
        &self,
        topics: &[(&str, i32, i32)],
    ) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        for (topic_name, num_partitions, replication_factor) in topics {
            if let Err(e) = self
                .create_topic_if_not_exists(topic_name, *num_partitions, *replication_factor)
                .await
            {
                warn!("Failed to create topic '{}': {}", topic_name, e);
                errors.push(format!("Topic '{}': {}", topic_name, e));
            }
        }

        if errors.is_empty() {
            info!("All topics created successfully");
            Ok(())
        } else {
            error!("Some topics failed to create: {:?}", errors);
            Err(errors)
        }
    }
}

/// Initialize application topics
///
/// This function creates all the topics needed by the application
/// Call this during application startup if auto-create is disabled
pub async fn initialize_application_topics(bootstrap_servers: &str) -> Result<(), String> {
    info!("Initializing application topics...");

    let topic_manager = KafkaTopicManager::new(bootstrap_servers)
        .map_err(|e| format!("Failed to create topic manager: {:?}", e))?;

    // Define application topics
    // Format: (topic_name, num_partitions, replication_factor)
    let topics = vec![("payment_notifications", 3, 1)];

    topic_manager
        .create_topics_batch(&topics)
        .await
        .map_err(|errors| format!("Topic creation errors: {:?}", errors))?;

    info!("Application topics initialized successfully");
    Ok(())
}
