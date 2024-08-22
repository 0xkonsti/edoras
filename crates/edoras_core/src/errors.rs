use std::fmt::Display;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum MessageError {
    UnknownError,
    ReadError(IoError),
    WriteError(IoError),
    InvalidMessage(Vec<u8>),
}

impl MessageError {
    pub fn to_string(&self) -> String {
        match self {
            MessageError::ReadError(err) => format!("Failed to read from stream: {}", err),
            MessageError::WriteError(err) => format!("Failed to write to stream: {}", err),
            MessageError::InvalidMessage(msg) => format!("Invalid message: {:x?}", msg),
            _ => String::from("Unknown error"),
        }
    }
}

impl Display for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::error::Error for MessageError {}
