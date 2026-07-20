use std::io::Result;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{Duration, sleep};

async fn process_socket(mut socket: TcpStream) -> Result<()> {
    loop {
        sleep(Duration::from_secs(1)).await;

        let msg = "Hello from Tokio TCP Server!";
        println!("Sending: \"{}\"", msg);
        socket.write_all(msg.as_bytes()).await?;
        //socket.write_all(msg.as_bytes()).await?;
        break;
    }

    println!("Client disconnected.");
    Ok(())
}

pub async fn run() -> Result<()> {
    println!("Server starting...");

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Listening on 127.0.0.1:8080");

    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await?;
    }
}
