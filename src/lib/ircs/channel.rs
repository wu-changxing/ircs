use crate::connect::{ConnectionError, ConnectionManager, ConnectionWrite};
use crate::ircs::client::Client;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Channel {
    pub name: String,
    pub clients: Vec<String>,
}

impl Channel {
    pub fn join(&mut self, client: &Client) {
        self.clients.push(client.nick.to_string());
        // Send a join message to the client or other clients in the channel if needed
    }

    pub fn part(&mut self, client: &Client) -> bool {
        if let Some(client_index) = self.clients.iter().position(|nick| nick == &client.nick) {
            self.clients.remove(client_index);
            // Send a part message to the client or other clients in the channel if needed
            true
        } else {
            false
        }
    }

    pub async fn broadcast_message(
        &self,
        message: &str,
        connection_map: &mut HashMap<String, ConnectionWrite>,
    ) {
        for client_nick in &self.clients {
            if let Some(conn_write) = connection_map.get_mut(client_nick) {
                conn_write.write_message(message).await.unwrap();
            }
        }
    }
}
