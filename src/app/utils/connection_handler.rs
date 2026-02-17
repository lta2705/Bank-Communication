use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_native_tls::TlsStream;
use std::io;

/// Interface (Trait) for any type of Connection
#[async_trait]
pub trait Connection: Send + Sync {
    async fn read_data(&mut self, buf: &mut [u8]) -> io::Result<usize>;
    async fn write_data(&mut self, data: &[u8]) -> io::Result<()>;
}

/// Implementation for Standard TCP
pub struct PlainTcpConnection {
    pub stream: TcpStream,
}

#[async_trait]
impl Connection for PlainTcpConnection {
    async fn read_data(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf).await
    }
    async fn write_data(&mut self, data: &[u8]) -> io::Result<()> {
        self.stream.write_all(data).await
    }
}

/// Implementation for TLS TCP
pub struct TlsTcpConnection {
    pub stream: TlsStream<TcpStream>,
}

#[async_trait]
impl Connection for TlsTcpConnection {
    async fn read_data(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf).await
    }
    async fn write_data(&mut self, data: &[u8]) -> io::Result<()> {
        self.stream.write_all(data).await
    }
}