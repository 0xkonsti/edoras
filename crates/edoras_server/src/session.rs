use async_std::net::TcpStream;
use edoras_core::{Message, MessageError};

pub(crate) struct Session {
    stream: TcpStream,

    closed: bool,
}

impl Session {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            closed: false,
        }
    }

    pub fn peer_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.stream.peer_addr()
    }

    pub fn closed(&self) -> bool {
        self.closed
    }

    pub fn close(&mut self) {
        self.closed = true;
    }

    pub fn shutdown(&self) -> std::io::Result<()> {
        self.stream.shutdown(std::net::Shutdown::Both)
    }

    pub async fn recieved_msg(&mut self) -> bool {
        Message::peek_for_header(&mut self.stream).await
    }

    pub async fn send(&mut self, message: Message) -> Result<(), MessageError> {
        message.send(&mut self.stream).await
    }

    pub async fn recv(&mut self) -> Result<Message, MessageError> {
        Message::recv(&mut self.stream).await
    }
}
