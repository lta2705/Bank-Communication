use tracing_subscriber::prelude::*;
use tracing_subscriber::{EnvFilter, fmt, Registry};
use tracing_appender::rolling::{Rotation, RollingFileAppender};

pub fn setup_tracing() -> Result<(), Box<dyn std::error::Error>> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let file_appender = RollingFileAppender::new(Rotation::DAILY, "log", "app.log");
    let (non_blocking_file_writer, _guard) = tracing_appender::non_blocking(file_appender);

    // Configure logging write to console (Stdout):
    let console_layer = fmt::layer()
        .pretty() // Dùng định dạng đẹp, có màu sắc
        .with_writer(std::io::stdout);

    //
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_file_writer);

    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .with(file_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}