use tokio::io;

use crate::app::utils::connection_handler::{Connection, PlainTcpConnection, TlsTcpConnection};
use tracing::{info, debug, error,warn};

pub enum ConnectionMode {
    Plain,
    Tls(native_tls::Identity),
}

pub async fn run_server(address: &str, mode: ConnectionMode) -> io::Result<()> {
    let listener = tokio::net::TcpListener::bind(address).await?;

    // If TLS, setup acceptor
    let tls_acceptor = if let ConnectionMode::Tls(identity) = &mode {
        let connector = native_tls::TlsAcceptor::new(identity.clone())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Some(tokio_native_tls::TlsAcceptor::from(connector))
    } else {
        None
    };

    loop {
        let (stream, _) = listener.accept().await?;
        let acceptor = tls_acceptor.clone();

        tokio::spawn(async move {
            let conn: Box<dyn Connection> = match acceptor {
                Some(acc) => {
                    // Perform TLS Handshake
                    let tls_stream = acc.accept(stream).await.expect("TLS Handshake failed");
                    Box::new(TlsTcpConnection { stream: tls_stream })
                }
                None => Box::new(PlainTcpConnection { stream }),
            };

            let _ = handle_client_logic(conn).await;
        });
    }
}

pub async fn handle_client_logic(mut connection: Box<dyn Connection>) -> io::Result<()> {
    let mut buffer = [0u8; 1024];
    
    loop {
        // The logic remains the same regardless of TCP or TLS
        match connection.read_data(&mut buffer).await {
            Ok(0) => break, // Connection closed
            Ok(n) => {
                info!("Received {} bytes", n);
                connection.write_data(b"ACK\n").await?;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}
