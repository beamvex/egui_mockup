#[derive(Clone)]
pub struct Message {
    pub author: String,
    pub content: String,
    pub timestamp: String,
}

#[derive(Clone)]
pub struct Channel {
    pub name: String,
    pub messages: Vec<Message>,
}

#[derive(Clone)]
pub struct Server {
    pub name: String,
    pub channels: Vec<Channel>,
}
