use dotenv::dotenv;
use std::env;
use tokio::{
    io::copy_bidirectional,
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or("127.0.0.1:12369".to_string());
    let send_addr = env::var("SEND_ADDR").unwrap_or("127.0.0.1:12368".to_string());

    let mut upstream = TcpStream::connect(send_addr).await?;

    let listener = TcpListener::bind(listen_addr).await?;
    // Accept just one connection
    let (mut downstream, _) = listener.accept().await?;

    if copy_bidirectional(&mut upstream, &mut downstream)
        .await
        .is_err()
    {
        eprintln!("failed to copy");
    }

    Ok(())
}
