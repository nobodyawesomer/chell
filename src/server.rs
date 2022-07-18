use log::{debug, error, info};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, ToSocketAddrs},
};

use crate::Result;

pub async fn listen(host: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(host).await?;
    loop {
        let (mut socket, addr) = listener.accept().await?;
        info!("Connection from {addr}");
        tokio::spawn(async move {
            // let mut buf = [0u8; 1024];
            let mut buf = [0u8; 64]; // testing... todo put under cfg debug vs release

            // Handle connection
            loop {
                // Read
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => {
                        info!("Connection from {addr} closed");
                        return;
                    }
                    Ok(n) => n,
                    Err(e) => {
                        error!("failed to read from socket: {e:?}");
                        return;
                    }
                };
                debug!("Read {n} bytes from {addr}");

                // Process input
                // for now, just echoes
                debug!(
                    "Got '{}'",
                    &buf[..n].iter().map(|&i| i as char).collect::<String>()
                );

                // Write
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    error!("failed to write to socket: {e:?}");
                    return;
                }
            }
        });
    }
}
