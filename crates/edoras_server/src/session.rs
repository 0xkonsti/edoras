use async_std::net::TcpStream;
use edoras_core::{Message, MessageBuilder, MessageError, MessageType};
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct Session {
    id: Uuid,
    stream: TcpStream,
    closed: bool,

    user: Option<String>,
}

impl Session {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            id: Uuid::new_v4(),
            stream,
            closed: false,

            user: None,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user(&self) -> Option<&String> {
        self.user.as_ref()
    }

    pub fn set_user(&mut self, user: String) {
        if self.user.is_some() {
            return;
        }
        self.user = Some(user);
    }

    pub fn remove_user(&mut self) {
        self.user = None;
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

    pub async fn health_check(&mut self) -> bool {
        self.send(MessageBuilder::new().with_type(MessageType::Ping).build())
            .await
            .is_ok()
    }
}
