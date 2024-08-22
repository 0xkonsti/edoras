use crate::application::AppData;
use crate::session::Session;
use crate::user::User;
use async_std::sync::RwLock;
use edoras_core::Message;
use std::str;
use std::sync::Arc;

// TODO: Error handling

pub(crate) async fn handle_register(
    session: &mut Session,
    appdata: Arc<RwLock<AppData>>,
    message: &Message,
) {
    if session.user().is_some() {
        return;
    }

    if message.field_count() != 1 {
        return;
    }

    let username = match str::from_utf8(message.data_ref()[0]) {
        Ok(username) => username,
        Err(_) => return,
    };

    tracing::info!("Registering user {}", username);

    if appdata.read().await.get_user(username).is_some() {
        tracing::info!("User {} already exists", username);
        return;
    }

    let mut user = User::new(username.to_string());
    user.set_session(session.id());
    session.set_user(username.to_string());
    appdata
        .write()
        .await
        .insert_user(username.to_string(), user);
}

pub(crate) async fn handle_login(
    session: &mut Session,
    appdata: Arc<RwLock<AppData>>,
    message: &Message,
) {
    if session.user().is_some() {
        return;
    }

    if message.field_count() != 1 {
        return;
    }

    let username = match str::from_utf8(message.data_ref()[0]) {
        Ok(username) => username,
        Err(_) => return,
    };

    if appdata.read().await.get_user(username).is_none() {
        tracing::info!("User {} not found", username);
        return;
    }

    session.set_user(username.to_string());
    appdata
        .write()
        .await
        .get_user_mut(username)
        .unwrap()
        .set_session(session.id());
}
