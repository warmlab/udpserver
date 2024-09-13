use std::net::SocketAddr;

use message::{self, Message, MessageType};

use message::StatusType;
use message::MessageHandler;


pub struct Processor {
}

impl Processor {
    pub fn new() -> Processor {
        Processor {}
    }
}

impl MessageHandler for Processor {
    fn handle_message(&mut self, message: Message, src: SocketAddr) {
        if self.check_message(&message) == StatusType::STATUS_REFUSE {
            return;
        }
        // 
        match message.head.message_type {
            // TODO 
            MessageType::MSG_UNKNOWN => {},
            _ => todo!("not implement"),
        }
    }

    fn check_message(&mut self, message: &Message) -> StatusType {
        StatusType::STATUS_OK
    }
}
