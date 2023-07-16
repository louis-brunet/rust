pub enum Message {
    Join(String),
    Leave(String),
    Send(String, String),
}

impl Message {
    pub fn log(&self) {
        return log_message(self);
    }
}

fn log_message(msg: &Message) {
    match msg {
        Message::Join(id) => println!("join {}", id),
        Message::Leave(id) => println!("leave {}", id),
        Message::Send(id, text) => println!("send {} -- {}", id, text),
    }
}
