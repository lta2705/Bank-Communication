use std::sync::Arc;

use tracing::info;

use crate::app::config::connection_config::ConnAttr;
use crate::app::config::kafka_config::KafkaConfig;
use crate::app::error::AppError;
use crate::app::handlers::iso8583_msg_handler;
use crate::app::utils::connection_initializer::{ConnectionMode, TcpServer};
use crate::app::utils::database::establish_db_conn;

use crate::app::utils::kafka_consumer::create_consumer;
use crate::app::utils::kafka_producer::create_producer;
use crate::app::utils::kafka_topic_manager::initialize_application_topics;

use crate::models::app_context::AppContext;

pub async fn run() -> Result<(), AppError> {
    let kafka_cfg = KafkaConfig::from_env().map_err(AppError::KafkaConfig)?;

    info!("Kafka config loaded");

    // Initialize topics if auto-create is enabled
    if kafka_cfg.auto_create_topic {
        info!("Auto-create topics is enabled, initializing application topics...");
        if let Err(e) = initialize_application_topics(&kafka_cfg.bootstrap_servers).await {
            tracing::warn!(
                "Failed to initialize topics (they may be created on first send): {}",
                e
            );
        } else {
            info!("All application topics initialized successfully");
        }
    } else {
        info!("Auto-create topics is disabled, topics must be created manually");
    }

    let kafka_producer = Arc::new(create_producer(&kafka_cfg).map_err(AppError::KafkaConfig)?);

    let kafka_consumer = Arc::new(create_consumer(&kafka_cfg).map_err(AppError::KafkaConfig)?);

    let db_pool = establish_db_conn().await.map_err(AppError::Database)?;

    let db_pool = Arc::new(db_pool);
    info!("Database connection established");

    let ctx = Arc::new(AppContext {
        kafka_producer,
        kafka_consumer,
        db_pool: db_pool.clone(),
    });

    // Initialize ISO8583 transaction service
    iso8583_msg_handler::init_service(db_pool.clone(), ctx).await;
    info!("ISO8583 transaction service initialized");

    // let consumer_handle = {
    //     let consumer = Arc::clone(&ctx.kafka_consumer);
    //     let auto_commit = kafka_cfg.enable_auto_commit;
    //
    //     tokio::spawn(async move {
    //         info!("Kafka consumer started");
    //
    //         loop {
    //             let msg = consumer.recv().await
    //                 .map_err(AppError::Kafka)?;
    //
    //             if let Some(payload) = msg.payload() {
    //                 info!(
    //                     "Kafka message received: {}",
    //                     String::from_utf8_lossy(payload)
    //                 );
    //             }
    //
    //             if !auto_commit {
    //                 consumer.commit_message(
    //                     &msg,
    //                     rdkafka::consumer::CommitMode::Async,
    //                 )
    //                 .map_err(AppError::Kafka)?;
    //             }
    //         }
    //
    //         #[allow(unreachable_code)]
    //         Ok::<(), AppError>(())
    //     })
    // };

    let conn_cfg = ConnAttr::load_env().map_err(AppError::Config)?;

    let tcp_address = format!("{}:{}", conn_cfg.host, conn_cfg.port);

    let tcp_server = TcpServer::new(tcp_address, ConnectionMode::Plain);

    let tcp_handle = tokio::spawn(async move { tcp_server.start().await });

    info!("Application started");

    tokio::select! {
        res = tcp_handle => {
            res.map_err(AppError::TaskJoin)?
               .map_err(AppError::Io)?;
        }

        // res = consumer_handle => {
        //     res.map_err(AppError::TaskJoin)?
        //        .map_err(|e| {
        //            error!("Kafka consumer crashed: {}", e);
        //            e
        //        })?;
        // }

        _ = tokio::signal::ctrl_c() => {
            info!("Shutdown signal received");
        }
    }

    info!("Application shutting down");
    Ok(())
}
