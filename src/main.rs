mod client;
mod client_manager;
mod models;
mod server;

#[tokio::main]
async fn main() {
    let cmd = std::env::args().nth(1);

    match cmd.as_deref() {
        Some("server") => {
            if let Err(e) = server::run().await {
                eprintln!("Server error: {}", e);
            }
        }
        Some("client") => {
            if let Err(e) = client::run().await {
                eprintln!("Client error: {}", e);
            }
        }
        Some(other) => {
            println!("Unknown command: {}", other);
        }
        None => {
            println!("Usage:");
            println!("  cargo run -- server");
            println!("  cargo run -- client");
        }
    }
}
