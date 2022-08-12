use chrono::{DateTime, Utc};
use dotenv::dotenv;
use std::env;
use tokio::{
    io::copy_bidirectional,
    net::{TcpListener, TcpStream},
    spawn,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now: DateTime<Utc> = Utc::now();
    // println!("UTC now is: {}", now.format("%Y-%m-%d__%H-%M-%S-%f"));
    println!("UTC now is: {}", now.to_rfc3339());

    dotenv().ok();
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or("127.0.0.1:12369".to_string());
    let send_addr = env::var("SEND_ADDR").unwrap_or("127.0.0.1:12368".to_string());

    let listener = TcpListener::bind(listen_addr).await?;
    println!("Listening on {}", send_addr);
    loop {
        let (mut downstream, client_addr) = listener.accept().await?;
        println!("Connected to {}", client_addr);
        let mut upstream = TcpStream::connect(send_addr.clone()).await?;
        spawn(async move {
            if copy_bidirectional(&mut upstream, &mut downstream)
                .await
                .is_err()
            {
                eprintln!("Failed to copy.");
            }
        });
    }
}
