use crate::app::utils::connection_handler::{
    Connection, PlainTcpConnection, TlsTcpConnection,
};
use std::sync::Arc;
use tokio::{io, time::timeout};
use tokio::time::Duration;
use tracing::{info, error, warn};
use crate::app::service::iso8583_msg_handler::handle_message;

pub enum ConnectionMode {
    Plain,
    Tls(Arc<native_tls::Identity>),
}

pub struct TcpServer {
    address: String,
    mode: ConnectionMode,
}

impl TcpServer {
    pub fn new(address: String, mode: ConnectionMode) -> Self {
        Self { address, mode }
    }

    pub async fn start(self) -> io::Result<()> {
        let listener = tokio::net::TcpListener::bind(&self.address).await?;
        info!("TCP server listening on {}", self.address);

        let tls_acceptor = match &self.mode {
            ConnectionMode::Tls(identity) => {
                let acceptor = native_tls::TlsAcceptor::new((**identity).clone())
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                Some(tokio_native_tls::TlsAcceptor::from(acceptor))
            }
            ConnectionMode::Plain => None,
        };

        loop {
            let (stream, peer) = listener.accept().await?;
            info!("===> Raw TCP connection detected from: {}", peer);
            let acceptor = tls_acceptor.clone();

            tokio::spawn(async move {
                info!("Accepted connection from {}", peer);

                let conn: Box<dyn Connection + Send> = match acceptor {
                    Some(acc) => {
                        let tls_stream = match acc.accept(stream).await {
                            Ok(s) => s,
                            Err(e) => {
                                error!("TLS handshake failed: {}", e);
                                return;
                            }
                        };
                        Box::new(TlsTcpConnection { stream: tls_stream })
                    }
                    None => Box::new(PlainTcpConnection { stream }),
                };

                if let Err(e) = handle_client_logic(conn).await {
                    error!("Connection error for {}: {}", peer, e);
                }
            });
        }
    }
}

pub async fn handle_client_logic(
    mut connection: Box<dyn Connection + Send>,
) -> io::Result<()> {
    let mut buffer = [0u8; 4096];

    loop {
        // Đợi nhận dữ liệu với timeout 30s
        match timeout(Duration::from_secs(30), connection.read_data(&mut buffer)).await {
            // Case 1: Client đóng kết nối chủ động
            Ok(Ok(0)) => {
                info!("Client closed connection");
                break;
            }

            // Case 2: Nhận được byte dữ liệu
            Ok(Ok(n)) => {
                info!("Received {} bytes raw", n);

                // --- Bước 1: Chuyển đổi Bytes sang String (UTF-8) ---
                let raw_string = String::from_utf8(buffer[..n].to_vec())
                    .map_err(|e| {
                        error!("Dữ liệu nhận được không phải UTF-8: {}", e);
                        io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 sequence")
                    })?;

                // Loại bỏ ký tự thừa như xuống dòng (\n, \r) hoặc khoảng trắng đầu cuối
                let trimmed_data = raw_string.trim();
                info!("Processed string: '{}'", trimmed_data);

                // --- Bước 2: Xử lý logic nghiệp vụ ---
                // Hàm handle_message bây giờ nhận vào &str
                let response_bytes = handle_message(trimmed_data)
                    .await
                    .map_err(|e| {
                        error!("Business logic error: {}", e);
                        io::Error::new(io::ErrorKind::Other, e)
                    })?;

                // --- Bước 3: Gửi phản hồi (binary) về cho client ---
                connection.write_data(&response_bytes).await?;
            }

            // Case 3: Lỗi trong quá trình đọc socket
            Ok(Err(e)) => {
                error!("Socket read error: {}", e);
                return Err(e);
            }

            // Case 4: Không nhận được gì sau 30s
            Err(_) => {
                warn!("Client read timeout after 30 seconds");
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "Client read timeout",
                ));
            }
        }
    }
    Ok(())
}