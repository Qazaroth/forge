use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let target_addr = "127.0.0.1:8080";
    println!("Client starting...");
    println!("Connecting to {}...", target_addr);

    let mut stream = TcpStream::connect(target_addr).await?;
    println!("Connected successfully!");

    let msg = "Hello from Tokio TCP Client!";
    println!("Sending: \"{}\"", msg);
    stream.write_all(msg.as_bytes()).await?;

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).await?;

    if bytes_read == 0 {
        println!("Server closed connection.");
    } else {
        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received: \"{}\"", response);
    }

    Ok(())
}
