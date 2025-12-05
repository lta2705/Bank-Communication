mod config;
mod utils;

use tracing::{info, error};
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    dotenvy::dotenv()?;
    for( key, value) in env::vars(){
        println!("{key}:{value}")
    }
    // Initialize logging
    let addr = "127.0.0.1:8887";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on {}", addr);

    loop {
        let (mut socket, peer) = listener.accept().await?;
        info!("Accepted connection from {}", peer);

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        // connection closed
                        info!("Connection closed: {}", peer);
                        return;
                    }
                    Ok(n) => {
                        info!("RAW STRING => {}", String::from_utf8_lossy(&buf[..n]));

                        if let Err(e) = socket.write_all(&buf[..n]).await {
                            error!("Failed to write to {}: {}", peer, e);
                            return;
                        }
                    }
                    Err(e) => {
                        error!("Failed to read from {}: {}", peer, e);
                        return;
                    }
                }
            }
        });
    }
}
