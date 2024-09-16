use std::net::SocketAddr;

use crate::{Message, MessageType, StatusType};

pub trait Handler<'a> {
    fn new(message: &'a Message, src: &'a SocketAddr) -> Self;
    fn handle_message(&mut self) -> StatusType;
    fn check_message(&mut self) -> StatusType;
}

pub struct MessageHandler<'a> {
    message: &'a Message,
    src: &'a SocketAddr
}

impl<'a> MessageHandler<'a> {
    pub fn new(message: &'a Message, src: &'a SocketAddr) -> impl Handler<'a> {
        MessageHandler {
            message,
            src
        }
    }
}

impl<'a> Handler<'a> for MessageHandler<'a> {
    fn new(message: &'a Message, src: &'a SocketAddr) -> Self {
        MessageHandler {
            message,
            src
        }
    }

    fn handle_message(&mut self) -> StatusType {
        if self.check_message() == StatusType::STATUS_REFUSE {
            return StatusType::STATUS_REFUSE;
        }
        // 
        match self.message.head.message_type {
            // TODO 
            MessageType::MSG_UNKNOWN => {},
            _ => todo!("not implement"),
        }

        StatusType::STATUS_ERROR
    }

    fn check_message(&mut self) -> StatusType {
        StatusType::STATUS_OK
    }
}
