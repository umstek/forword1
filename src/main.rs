use dotenv::dotenv;
use log::{error, info, trace};
use std::env;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    spawn,
};

async fn log_and_forward(
    mut reader: impl AsyncReadExt + Unpin,
    mut writer: impl AsyncWriteExt + Unpin,
    label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = [0; 1024];
    loop {
        let n = reader.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        let data = String::from_utf8_lossy(&buf[..n]);
        trace!("{} -> {}", label, data);
        writer.write_all(&buf[..n]).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::builder().format_timestamp_nanos().init();

    let listen_addr = env::var("LISTEN_ADDR").unwrap_or("0.0.0.0:12369".to_string());
    let send_addr = env::var("SEND_ADDR").unwrap_or("127.0.0.1:12368".to_string());

    let listener = TcpListener::bind(listen_addr.clone()).await?;
    info!("Listening on {}", listen_addr);
    loop {
        let (downstream, client_addr) = listener.accept().await?;
        info!("Client connected {}", client_addr);
        let upstream = TcpStream::connect(send_addr.clone()).await?;
        info!("Forwarding to {}", send_addr);

        let (rd1, wr1) = downstream.into_split();
        let (rd2, wr2) = upstream.into_split();

        spawn(async move {
            if let Err(e) = log_and_forward(rd1, wr2, "dn").await {
                error!("Failed to forward: {}", e);
            }
        });

        spawn(async move {
            if let Err(e) = log_and_forward(rd2, wr1, "up").await {
                error!("Failed to forward: {}", e);
            }
        });
    }
}
