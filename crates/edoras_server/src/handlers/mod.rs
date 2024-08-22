mod auth;

use crate::application::AppData;
use crate::session::Session;
use async_std::sync::RwLock;
use edoras_core::{Message, MessageBuilder, MessageType};
use std::sync::Arc;

pub async fn handle_message(
    session: Arc<RwLock<Session>>,
    appdata: Arc<RwLock<AppData>>,
    message: &Message,
) {
    match message.mtype() {
        MessageType::Ping => {
            session
                .write()
                .await
                .send(MessageBuilder::new().with_type(MessageType::Pong).build())
                .await
                .unwrap();
        }
        MessageType::Disconnect => {
            appdata
                .write()
                .await
                .remove_session(&session.read().await.id());
            appdata
                .write()
                .await
                .get_user_mut(session.read().await.user().as_ref().unwrap())
                .unwrap()
                .clear_session();
            session.write().await.close();
        }
        MessageType::Login => {
            auth::handle_login(session, appdata, message).await;
        }
        MessageType::Register => {
            auth::handle_register(session, appdata, message).await;
        }
        _ => {}
    }
}
