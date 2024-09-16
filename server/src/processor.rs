use std::net::SocketAddr;

use message::{self, Message, MessageType};

use message::StatusType;
use message::{Handler, MessageHandler};


pub struct Processor {
}

impl Processor {
    pub fn new() -> Processor {
        Processor {}
    }

    pub fn process(&self, message: Message, src: SocketAddr) -> StatusType {
        let mut handler = MessageHandler::new(&message, &src);

        // TODO
        

        StatusType::STATUS_ERROR
    }
}
