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
                    error!("Connection error: {}", e);
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
        match timeout(Duration::from_secs(30), connection.read_data(&mut buffer)).await {
            // Client closed connection
            Ok(Ok(0)) => {
                info!("Client closed connection");
                break;
            }

            // Received data
            Ok(Ok(n)) => {
                info!("Received {} bytes", n);

                // ===== 1. NHẬN RAW EMV (BINARY) =====
                let raw_emv: Vec<u8> = buffer[..n].to_vec();

                // Log HEX cho debug (KHÔNG dùng cho xử lý)
                    info!("Received EMV (hex): {}", hex::encode_upper(&raw_emv));

                // ===== 2. XỬ LÝ TOÀN BỘ GIAO DỊCH =====
                // Parse TLV → build ISO → send bank → wait response
                let response_bytes = handle_message(&raw_emv)
                    .await
                    .map_err(|e| {
                        io::Error::new(io::ErrorKind::Other, e)
                    })?;

                // ===== 3. TRẢ RESPONSE TRÊN CÙNG CONNECTION =====
                connection.write_data(&response_bytes).await?;
            }

            // Read error
            Ok(Err(e)) => {
                error!("Read error: {}", e);
                return Err(e);
            }

            // Timeout
            Err(_) => {
                warn!("Client read timeout");
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "client read timeout",
                ));
            }
        }
    }

    Ok(())
}
