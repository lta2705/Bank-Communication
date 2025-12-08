mod utils;
mod adapters;
mod api;
mod config;
mod core;
mod security;

use tracing::{info, error};
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use std::env;
use crate::utils::logging::setup_tracing;
use crate::utils::database::establish_conn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_tracing()?;

    match establish_conn().await{
        Ok(pool) => {
            println!("Connect to DB successfully",);
        }
        Err(err) => {
            eprintln!("Failed to connect: {:?}", err);
        }
    }

    // Initialize logging
    let addr = "127.0.0.1:8887";
    let listener = TcpListener::bind(addr).await?;

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
