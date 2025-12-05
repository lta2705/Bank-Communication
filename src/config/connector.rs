use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

trait AbstractTcpHandler {
    fn new() -> Self;
    async fn handle_connection(socket: TcpStream);
}

pub struct TcpHandler;

impl AbstractTcpHandler for TcpHandler {
    fn new() -> Self {
        TcpHandler {}
    }

    async fn handle_connection(mut socket: TcpStream) { // <-- Nhận vào tokio::net::TcpStream
        let mut buf = [0; 2048];

        let remote_addr = socket
            .peer_addr()
            .map_or_else(|_| "unknown address".to_string(), |addr| addr.to_string());

        println!("New connection from {}.", remote_addr);

        loop {
            // Đọc dữ liệu từ socket (bất đồng bộ).
            let n = match socket.read(&mut buf).await {
                // socket đóng
                Ok(0) => {
                    println!("Connection from {} closed.", remote_addr);
                    return;
                }
                Ok(n) => n,
                Err(e) => {
                    eprintln!(
                        "Failed to read from socket ({}); error: {:?}",
                        remote_addr, e
                    );
                    return;
                }
            };

            // Ghi dữ liệu ngược lại (echo) (bất đồng bộ).
            if let Err(e) = socket.write_all(&buf[0..n]).await {
                eprintln!(
                    "Failed to write to socket ({}); error: {:?}",
                    remote_addr, e
                );
                return;
            }
        }
    }
}