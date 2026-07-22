use std::collections::HashMap;
use tokio::sync::mpsc;

pub struct ClientInfo {
    #[allow(dead_code)]
    pub username: Option<String>,
    pub sender: mpsc::Sender<String>,
}

pub struct ClientManager {
    clients: HashMap<u64, ClientInfo>,
    next_client_id: u64,
}

impl ClientManager {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            next_client_id: 0,
        }
    }

    pub fn generate_id(&mut self) -> u64 {
        self.next_client_id += 1;
        self.next_client_id
    }

    pub fn add_client(&mut self, id: u64, client: ClientInfo) {
        self.clients.insert(id, client);
    }

    // pub fn remove_client(&mut self, id: u64) {
    //     self.clients.remove(&id);
    // }

    // pub fn get_client(&self, id: u64) -> Option<&ClientInfo> {
    //     return self.clients.get(&id);
    // }

    // pub fn get_client_mut(&mut self, id: u64) -> Option<&mut ClientInfo> {
    //     return self.clients.get_mut(&id);
    // }

    pub fn get_receivers(&self, sender_id: u64) -> Vec<mpsc::Sender<String>> {
        let mut receivers = Vec::new();

        for (id, client) in &self.clients {
            if *id != sender_id {
                receivers.push(client.sender.clone());
            }
        }

        receivers
    }

    // pub fn len(&self) -> usize {
    //     self.clients.len()
    // }

    // pub fn is_empty(&self) -> bool {
    //     self.clients.is_empty()
    // }
}
