mod errors;
mod message;

pub use errors::MessageError;
pub use message::{Message, MessageBuilder, MessageType};

pub const HOST: &str = "127.0.0.1";
pub const PORT: u16 = 42428;
