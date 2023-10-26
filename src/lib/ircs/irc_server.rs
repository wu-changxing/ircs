// src/lib/ircs/irc_server.rs
use crate::connect::ConnectionWrite;
use std::collections::hash_map::Entry;

use crate::ircs::channel::Channel;
use crate::ircs::client::Client;
use crate::ircs::irc_message::IrcMessage;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct IrcServer {
    clients: Vec<Client>,
    channels: Vec<Channel>,
    connection_map: HashMap<String, Arc<Mutex<ConnectionWrite>>>,
}
fn valid_nickname(nick: &str) -> bool {
    // Check for length and allowed characters
    nick.len() <= 9
        && nick.chars().all(|c| c.is_ascii_alphanumeric())
        && !nick.chars().nth(0).unwrap().is_ascii_digit()
}
impl IrcServer {
    fn send_err_needmoreparams(&mut self, command: &str) -> String {
        format!(":{} 461  :Need more parameters\r\n", command)
    }

    fn send_err_nosuchchannel(&mut self, channel_name: &str) -> String {
        format!(" 403  {} :No such channel\r\n", channel_name)
    }
    pub fn new() -> Self {
        Self {
            clients: Vec::new(),
            channels: Vec::new(),
            connection_map: HashMap::new(),
        }
    }

    // Add this function to the `impl IrcServer`
    pub fn add_connection(&mut self, client_nick: String, conn_write: Arc<Mutex<ConnectionWrite>>) {
        self.connection_map.insert(client_nick, conn_write);
    }

    pub fn remove_connection(&mut self, client_nick: &str) {
        self.connection_map.remove(client_nick);
    }

    pub async fn send_privmsg_from_server(&mut self, target: &str, message: &str) {
        let sender_nick = "server";
        let formatted_message = format!(":{} PRIVMSG {} :{}\r\n", sender_nick, target, message);

        if let Some(conn_write) = self.connection_map.get_mut(target) {
            let mut conn_write = conn_write.lock().await;
            conn_write.write_message(&formatted_message).await.unwrap();
        } else {
            println!("Target client not found");
        }
    }
    pub async fn welcome_client(
        &self,
        client: &Client,
        conn_write: &mut Arc<Mutex<ConnectionWrite>>,
    ) {
        let realname = client.realname.as_ref().unwrap_or(&client.nick);
        let message = format!(
            ":iris-server 001 {} :Hi {}, welcome to IRC\r\n",
            client.nick, realname
        );
        let mut conn_write = conn_write.lock().await;
        conn_write.write_message(&message).await.unwrap();
    }
    pub async fn handle_quit_command(
        &mut self,
        irc_message: IrcMessage,
        conn_write: &mut Arc<Mutex<ConnectionWrite>>,
    ) {
        let message = if !irc_message.params.is_empty() {
            irc_message.params[0].clone()
        } else {
            format!("{} has quit", irc_message.from_nick.unwrap())
        };

        let mut conn_write = conn_write.lock().await;
        conn_write
            .write_message(&format!("QUIT :{}\r\n", message))
            .await
            .unwrap();
    }

    fn get_channel(&self, channel_name: &str) -> Option<usize> {
        for (i, channel) in self.channels.iter().enumerate() {
            if channel.name == channel_name {
                return Some(i);
            }
        }
        None
    }

    /// join a channel

    pub async fn handle_join_command(
        &mut self,
        irc_message: IrcMessage,
        conn_write: &mut Arc<Mutex<ConnectionWrite>>,
        client: &Client,
    ) {
        let mut response = String::new();

        if irc_message.params.len() < 1 {
            response = self.send_err_needmoreparams("JOIN");
        } else {
            let channel_name = &irc_message.params[0];
            if !channel_name.starts_with('#') {
                response = self.send_err_nosuchchannel(channel_name);
            } else if let Some(channel_index) = self.get_channel(channel_name) {
                self.channels[channel_index].join(client);
            } else {
                // Handle the case where the channel does not exist, if necessary
            }
        }

        if !response.is_empty() {
            let mut conn_write = conn_write.lock().await;
            conn_write.write_message(&response).await.unwrap();
        }
    }

    pub async fn handle_part_command(
        &mut self,
        irc_message: IrcMessage,
        conn_write: &mut Arc<Mutex<ConnectionWrite>>,
        client: &Client,
    ) {
        let mut response = String::new();

        if irc_message.params.len() < 1 {
            response = self.send_err_needmoreparams("PART");
        } else {
            let channel_name = &irc_message.params[0];
            if !channel_name.starts_with('#') {
                response = self.send_err_nosuchchannel(channel_name);
            } else if let Some(channel_index) = self.get_channel(channel_name) {
                if !self.channels[channel_index].part(client) {
                    response = self.send_err_nosuchchannel(channel_name);
                }
            } else {
                // Handle the case where the channel does not exist, if necessary
            }
        }

        if !response.is_empty() {
            let mut conn_write = conn_write.lock().await;
            conn_write.write_message(&response).await.unwrap();
        }
    }

    pub async fn handle_privmsg_command(
        &mut self,
        irc_message: IrcMessage,
        from_conn_write: &mut Arc<Mutex<ConnectionWrite>>,
        from_nick: &str,
    ) {
        let target = irc_message.to_nick.unwrap_or("No target".to_string());
        let mut message = irc_message.params[1].clone();

        if !message.ends_with("\r\n") {
            message.push_str("\r\n");
        }

        println!(
            "[{}] About to send PRIVMSG from {} to {}: {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            from_nick,
            target,
            message
        );

        match self.connection_map.entry(target.clone()) {
            Entry::Occupied(mut target_conn_write) => {
                let mut target_conn_write = target_conn_write.get_mut().lock().await;
                target_conn_write.write_message(&message).await.unwrap();
                dbg!("Message sent to target");
                println!(
                    "[{}] PRIVMSG sent from {} to {}: {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    from_nick,
                    target,
                    message
                );
            }
            Entry::Vacant(_) => {
                let mut from_conn_write = from_conn_write.lock().await;
                from_conn_write
                    .write_message("Target client not found")
                    .await
                    .unwrap();
            }
        }
    }
    pub async fn handle_ping_command(
        &self,
        irc_message: IrcMessage,
        conn_write: &mut Arc<Mutex<ConnectionWrite>>,
    ) {
        let hostname = &irc_message.params[0];
        let mut conn_write = conn_write.lock().await;
        conn_write
            .write_message(&format!("PONG :{}\r\n", hostname))
            .await
            .unwrap();
    }

    pub async fn handle_user_command(
        &mut self,
        irc_message: IrcMessage,
        conn_write: &mut Arc<Mutex<ConnectionWrite>>,
        client: &mut Client, // Add client parameter
    ) {
        // Ensure there are at least 4 parameters
        if irc_message.params.len() < 4 {
            let mut conn_write = conn_write.lock().await;
            conn_write
                .write_message("ERR_NEEDMOREPARAMS\r\n")
                .await
                .unwrap();
            return;
        }

        let realname = match irc_message.params.last() {
            Some(realname) if realname.starts_with(':') => realname[1..].to_string(),
            _ => {
                let mut conn_write = conn_write.lock().await;
                conn_write
                    .write_message("ERR_NEEDMOREPARAMS\r\n")
                    .await
                    .unwrap();
                return;
            }
        };
        // update realname for client
        dbg!(&realname);
        client.realname = Some(realname);
        //print the realname
    }

    pub async fn handle_nick_command(
        &mut self,
        irc_message: IrcMessage,
        conn_write: &mut Arc<Mutex<ConnectionWrite>>,
        client: &mut Client, // Add client parameter
    ) {
        let new_nick = irc_message.params[0].clone();
        if !valid_nickname(&new_nick) {
            let mut conn_write = conn_write.lock().await;
            conn_write
                .write_message("ERR_ERRONEUSNICKNAME\r\n")
                .await
                .unwrap();
            return;
        }

        if self.clients.iter().any(|client| client.nick == new_nick) {
            let mut conn_write = conn_write.lock().await;
            conn_write
                .write_message("ERR_NICKCOLLISION\r\n")
                .await
                .unwrap();
            return;
        }

        client.nick = new_nick;
        println!("New client with nickname {}", client.nick);
        self.clients.push(client.clone());
    }
    // Implementations for IrcServer
}
