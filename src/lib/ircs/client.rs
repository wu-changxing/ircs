#[derive(Clone)]
pub struct Client {
    pub nick: String,
    pub realname: Option<String>,
    pub channels: Vec<String>,
}
