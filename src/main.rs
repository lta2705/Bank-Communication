use tokio::net::TcpListener;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:8086").await?;
    println!("Tokio Echo Server is running on 127.0.0.1:8086");

    let mut connector = config::connector::TcpHandler:;

    loop {

        let (socket, remote_addr) = listener.accept().await?;
        println!("Accepted new connection from: {}", remote_addr);

        tokio::spawn(async move {

        });
    }
}