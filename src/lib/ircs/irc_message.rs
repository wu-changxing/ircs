pub struct IrcMessage {
    pub command: IrcCommand,
    pub params: Vec<String>,
    pub from_nick: Option<String>,
    pub to_nick: Option<String>,
}

impl IrcMessage {
    pub fn parse(message: &str) -> Option<Self> {
        let parts: Vec<&str> = message.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        let command = IrcCommand::from_str(parts[0])?;
        let mut params: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
        let mut from_nick: Option<String> = None;
        let mut to_nick: Option<String> = None;
        if command == IrcCommand::PRIVMSG && params.len() >= 2 {
            to_nick = Some(parts[1].to_string());
            let msg_start_index = message
                .find(':')
                .map(|index| index + 1)
                .expect("Message part not found");
            let full_message = message[msg_start_index..].to_string();
            params.push(full_message);
        }
        Some(Self {
            command,
            params,
            from_nick,
            to_nick,
        })
    }
    pub fn set_from_nick(&mut self, nick: &str) {
        self.from_nick = Some(nick.to_string());
    }
}

#[derive(Debug, PartialEq)]
pub enum IrcCommand {
    NICK,
    USER,
    PING,
    PONG,
    QUIT,
    PRIVMSG,
    JOIN,
    PART,
}

impl IrcCommand {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_ascii_uppercase().as_str() {
            "NICK" => Some(Self::NICK),
            "USER" => Some(Self::USER),
            "PING" => Some(Self::PING),
            "PONG" => Some(Self::PONG),
            "QUIT" => Some(Self::QUIT),
            "PRIVMSG" => Some(Self::PRIVMSG),
            "JOIN" => Some(Self::JOIN),
            "PART" => Some(Self::PART),
            _ => None,
        }
    }
}
