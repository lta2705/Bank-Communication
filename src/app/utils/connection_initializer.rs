use crate::app::utils::connection_handler::{
    Connection, PlainTcpConnection, TlsTcpConnection,
};
use std::sync::Arc;
use tokio::{io, time::timeout};
use tokio::time::Duration;
use tracing::{info, error, warn};

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
    mut connection: Box<dyn Connection + Send>
) -> io::Result<()> {
    let mut buffer = [0u8; 2048];

    loop {
        match timeout(Duration::from_secs(30), connection.read_data(&mut buffer)).await {
            // timeout OK, read OK
            Ok(Ok(0)) => break, // connection closed

            Ok(Ok(n)) => {
                info!("Received {} bytes", n);
                connection.write_data(b"ACK\n").await?;
            }

            // timeout OK, read ERROR
            Ok(Err(e)) => return Err(e),

            // timeout EXPIRED
            Err(_) => {
                warn!("Read timeout");
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "client read timeout",
                ));
            }
        }
    }

    Ok(())
}