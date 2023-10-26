// src/lib/handlers.rs
use crate::types::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct UserManager {
    users: Arc<Mutex<HashMap<Nick, String>>>,
}

impl UserManager {
    pub fn new() -> Self {
        UserManager {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn handle_command(&self, message: UnparsedMessage) -> Result<Reply, ErrorType> {
        let parsed_message = ParsedMessage::try_from(message)?;
        match parsed_message.message {
            Message::Nick(nick_msg) => self.handle_nick(parsed_message.sender_nick, nick_msg.nick),
            Message::User(user_msg) => {
                self.handle_user(parsed_message.sender_nick, user_msg.real_name)
            }
            _ => Err(ErrorType::UnknownCommand),
        }
    }

    fn handle_nick(&self, old_nick: Nick, new_nick: Nick) -> Result<Reply, ErrorType> {
        let mut users = self.users.lock().unwrap();

        if users.contains_key(&new_nick) {
            Err(ErrorType::NickCollision)
        } else {
            users.remove(&old_nick);
            users.insert(new_nick, users.get(&old_nick).unwrap().clone());
            Ok(Reply::Welcome(WelcomeReply {
                target_nick: new_nick,
                message: format!("Welcome to the server, {}", new_nick),
            }))
        }
    }

    fn handle_user(&self, sender_nick: Nick, real_name: String) -> Result<Reply, ErrorType> {
        let mut users = self.users.lock().unwrap();

        if !users.contains_key(&sender_nick) {
            return Err(ErrorType::NoNickNameGiven);
        }

        users.insert(sender_nick.clone(), real_name);
        Ok(Reply::Welcome(WelcomeReply {
            target_nick: sender_nick,
            message: format!("Welcome to the server, {}", sender_nick),
        }))
    }
}
