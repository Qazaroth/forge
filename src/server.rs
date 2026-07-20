use std::io::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::time::{Duration, sleep};

use crate::models::client_session::ClientSession;

async fn process_client(mut client: ClientSession) -> Result<()> {
    loop {
        sleep(Duration::from_secs(1)).await;

        let mut buffer = [0; 1024];
        let bytes_read = client.socket.read(&mut buffer).await?;

        if bytes_read == 0 {
            println!("Client disconnected.");
            break;
        }

        let msg = String::from_utf8_lossy(&buffer[..bytes_read]).into_owned();
        let mut outgoing = msg.clone();
        match msg.split_once(' ') {
            Some((protocol, args)) => {
                if protocol == "MESSAGE" {
                    let username = client.username.as_deref().unwrap_or("Undefined");

                    println!("Received MESSAGE from {username}: \"{args}\"");
                    println!("Sending to {username}: \"{}\"", outgoing);
                } else if protocol == "NEW_USERNAME" {
                    let old_username = client.username.as_deref().unwrap_or("Undefined");
                    println!("Changing username from {old_username} to {args}");
                    client.username = Some(args.to_owned());
                    outgoing = format!("NEW_USERNAME {args}");
                }
            }
            None => {
                outgoing = String::from("INVALID INVALID");
                println!("Unknown packet received: {msg}");
            }
        }

        client.socket.write_all(outgoing.as_bytes()).await?;
    }

    Ok(())
}

pub async fn run() -> Result<()> {
    println!("Server starting...");

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Listening on 127.0.0.1:8080");

    loop {
        let (socket, _) = listener.accept().await?;
        let client = ClientSession {
            socket,
            username: Some(String::from("Undefined")),
        };
        tokio::spawn(async move {
            if let Err(err) = process_client(client).await {
                eprintln!("Client error: {err}");
            }
        });
    }
}
