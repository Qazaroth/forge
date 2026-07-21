use std::error::Error;
use std::io::Write;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct Client {
    pub stream: TcpStream,
    pub username: Option<String>,
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let target_addr = "127.0.0.1:8080";
    let stdin = std::io::stdin();
    let mut input = String::new();

    println!("Client starting...");

    print!("Enter username: ");
    std::io::stdout().flush()?;
    let _bytes = stdin.read_line(&mut input)?;
    let username = input.trim_end();

    println!("Connecting to {}...", target_addr);

    let stream = TcpStream::connect(target_addr).await?;
    println!("Connected successfully!");

    let mut client = Client {
        stream,
        username: Some(String::from(username)),
    };
    let username_changed = format!("NEW_USERNAME {username}");
    client.stream.write_all(username_changed.as_bytes()).await?;

    loop {
        input.clear();
        let _bytes = stdin.read_line(&mut input)?;
        let msg = input.trim_end();
        let mut outgoing = msg.to_owned();

        if msg == "/quit" {
            break;
        }

        if msg.starts_with("/") {
            match msg.split_once(' ') {
                Some((cmd, arg)) => {
                    let cmd = cmd.strip_prefix('/').unwrap_or(cmd);
                    println!("Command: {cmd}");
                    println!("Argument: {arg}");

                    match cmd {
                        "name" => {
                            outgoing = format!("NEW_USERNAME {arg}");
                        }
                        _ => {
                            println!("Unknown command.");
                        }
                    }
                }
                None => {
                    println!("{msg}");
                }
            }
        } else {
            outgoing = format!("MESSAGE {msg}");
        }

        //println!("Sending: \"{outgoing}\"");
        client.stream.write_all(outgoing.as_bytes()).await?;

        let mut buffer = [0; 1024];
        let bytes_read = client.stream.read(&mut buffer).await?;

        if bytes_read == 0 {
            println!("Server closed connection.");
        } else {
            let response = String::from_utf8_lossy(&buffer[..bytes_read]);
            match response.split_once(' ') {
                Some((protocol, args)) => {
                    if protocol == "MESSAGE" {
                        println!("[SERVER] : {args}");
                    } else if protocol == "NEW_USERNAME" {
                        println!("Notification: Username successfully changed to {args}");
                        client.username = Some(String::from(args));
                    }
                    //println!("{protocol} - {args}")
                }
                None => {
                    eprintln!("Server-client connection issue.");
                }
            }
        }
    }

    Ok(())
}
