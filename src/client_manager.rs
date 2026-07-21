use std::collections::HashMap;

pub struct ClientInfo {
    pub username: Option<String>,
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

    pub fn generate_id(&self) -> u64 {
        let id = self.next_client_id + 1;
        return id;
    }

    pub fn add_client(&mut self, id: u64, client: ClientInfo) {
        self.clients.insert(id, client);
    }

    pub fn remove_client(&mut self, id: u64) {
        self.clients.remove(&id);
    }

    pub fn get_client(&self, id: u64) -> Option<&ClientInfo> {
        return self.clients.get(&id);
    }

    pub fn get_client_mut(&mut self, id: u64) -> Option<&mut ClientInfo> {
        return self.clients.get_mut(&id);
    }

    pub fn broadcast(&self, sender_id: u64, message: &str) {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.clients.len()
    }

    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }
}
