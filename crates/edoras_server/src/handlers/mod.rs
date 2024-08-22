use crate::session::Session;
use edoras_core::{Message, MessageBuilder, MessageType};

pub async fn handle_message(session: &mut Session, message: &Message) {
    match message.mtype() {
        MessageType::Ping => {
            session
                .send(MessageBuilder::new().with_type(MessageType::Pong).build())
                .await
                .unwrap();
        }
        MessageType::Disconnect => {
            session.close();
        }
        _ => {}
    }
}
