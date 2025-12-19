// use crate::app::config::kafka_config::KafkaConfig;
// use rdkafka::config::ClientConfig;
// use rdkafka::consumer::{Consumer, StreamConsumer};

// pub fn create_consumer(cfg: &KafkaConfig) -> anyhow::Result<StreamConsumer> {
//     let mut client_cfg = ClientConfig::new();

//     client_cfg
//         // Common
//         .set("bootstrap.servers", &cfg.bootstrap_servers)
//         .set("group.id", &cfg.group_id)

//         // Consumer configs
//         .set("enable.auto.commit", cfg.enable_auto_commit.to_string())
//         .set("auto.offset.reset", &cfg.auto_offset_reset)
//         .set(
//             "max.poll.interval.ms",
//             cfg.max_poll_interval_ms.to_string(),
//         )
//         .set(
//             "session.timeout.ms",
//             cfg.session_timeout_ms.to_string(),
//         )
//         .set(
//             "heartbeat.interval.ms",
//             cfg.heartbeat_interval_ms.to_string(),
//         )
//         .set("isolation.level", &cfg.isolation_level);

//     let consumer: StreamConsumer = client_cfg.create()?;

//     consumer.subscribe(&[&cfg.consumer_topic])?;

//     Ok(consumer)
// }

