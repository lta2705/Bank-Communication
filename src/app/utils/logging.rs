use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{EnvFilter, fmt};

pub fn setup_tracing() -> Result<WorkerGuard, Box<dyn std::error::Error>> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let file_appender = RollingFileAppender::new(Rotation::DAILY, "log", "app.log");
    let (non_blocking_file_writer, guard) = tracing_appender::non_blocking(file_appender);

    let console_layer = fmt::layer().pretty().with_writer(std::io::stdout);

    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_file_writer);

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    Ok(guard)
}
