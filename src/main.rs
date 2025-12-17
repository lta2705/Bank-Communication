mod app;
mod models;

use tracing::{info, error};
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let builder = app::builder::builder::run();
    
    let app = match app {
    Ok(result) => {
        
    }
    }
}