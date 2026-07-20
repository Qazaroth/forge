use tokio::net::TcpStream;

pub struct ClientSession {
    pub socket: TcpStream,
    pub username: Option<String>,
}
