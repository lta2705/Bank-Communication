
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    // Bật TCP keepalive
    stream.set_keepalive(Some(Duration::from_secs(60)))?;

    let mut buffer = [0u8; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(n) => {
                let bytes = &buffer[..n];

                println!("Received bytes: {:02X?}", bytes);

                //If data is ASCII / UTF-8
                if let Ok(text) = std::str::from_utf8(bytes) {
                    println!("As string: {}", text);
                }

                // Echo lại client (optional)
                stream.write_all(b"ACK\n")?;
            }
            Err(e) => {
                eprintln!("Read error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9000")?;
    println!("TCP server listening on port 9000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from {:?}", stream.peer_addr());
                thread::spawn(|| {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Connection error: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Accept error: {}", e),
        }
    }

    Ok(())
}
