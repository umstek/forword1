use chrono::{DateTime, Utc};
use dotenv::dotenv;
use log::{debug, error, info};
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
        let now: DateTime<Utc> = Utc::now();
        info!("[{}] {} -> {}", now.to_rfc3339(), label, data);
        writer.write_all(&buf[..n]).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let now: DateTime<Utc> = Utc::now();
    // println!("UTC now is: {}", now.format("%Y-%m-%d__%H-%M-%S-%f"));
    info!("UTC now is: {}", now.to_rfc3339());

    dotenv().ok();
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or("127.0.0.1:12369".to_string());
    let send_addr = env::var("SEND_ADDR").unwrap_or("127.0.0.1:12368".to_string());

    let listener = TcpListener::bind(listen_addr.clone()).await?;
    info!("Listening on {}", listen_addr);
    loop {
        let (downstream, client_addr) = listener.accept().await?;
        info!("Connected client {}", client_addr);
        let upstream = TcpStream::connect(send_addr.clone()).await?;
        info!("Forwarding to {}", send_addr);

        let (rd1, wr1) = downstream.into_split();
        let (rd2, wr2) = upstream.into_split();

        spawn(async move {
            if let Err(e) = log_and_forward(rd1, wr2, "downstream").await {
                error!("Failed to forward: {}", e);
            }
        });

        spawn(async move {
            if let Err(e) = log_and_forward(rd2, wr1, "upstream").await {
                error!("Failed to forward: {}", e);
            }
        });
    }
}
