use std::net::SocketAddr;

use crate::{Message, StatusType};

pub trait MessageHandler {
    fn handle_message(&mut self, message: Message, src: SocketAddr);
    fn check_message(&mut self, message: &Message) -> StatusType;
}
