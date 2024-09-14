mod types;
mod message;
mod status;
mod handler;

pub use types::MessageType;
pub use message::{MessageHead, Message};
pub use status::StatusType;
pub use handler::MessageHandler;

pub use message::{MESSAGE_BEGIN, MESSAGE_FILL_LEN, MESSAGE_END};

pub trait EnumNumber<T, U> {
    fn try_from_number(value: U) -> T;
    fn try_into_number(value: T) -> U;
}
