use std::io::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{Duration, sleep};

async fn process_socket(mut socket: TcpStream) -> Result<()> {
    loop {
        sleep(Duration::from_secs(1)).await;

        let mut buffer = [0; 1024];
        let bytes_read = socket.read(&mut buffer).await?;

        if bytes_read == 0 {
            println!("Client disconnected.");
            break;
        }

        let msg = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received: \"{msg}\"");
        println!("Sending: \"{}\"", msg);
        socket.write_all(msg.as_bytes()).await?;
    }

    Ok(())
}

pub async fn run() -> Result<()> {
    println!("Server starting...");

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Listening on 127.0.0.1:8080");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(err) = process_socket(socket).await {
                eprintln!("Client error: {err}");
            }
        });
    }
}
