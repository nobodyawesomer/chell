//! Functionality relating to spawning a chell client connection.

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
};

use crate::Result;

pub async fn connect(host: impl ToSocketAddrs) -> Result<()> {
    let mut stream = TcpStream::connect(host).await?;

    stream.write_all(b"hello!").await?;

    // TODO: wait for input and send to stdout

    Ok(())
}
