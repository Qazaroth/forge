use tokio::net::TcpStream;

pub struct ClientSession {
    pub id: u64,
    pub socket: TcpStream,
    pub username: Option<String>,
}
