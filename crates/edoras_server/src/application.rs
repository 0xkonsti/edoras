use crate::server;
use crate::session::Session;
use crate::user::User;
use anyhow::Result as AnyResult;
use async_std::sync::RwLock;
use edoras_core::{HOST, PORT};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub(crate) const CONNECTION_LIMIT: usize = 8;

const TRACING_LEVEL: tracing::Level = tracing::Level::DEBUG;

#[derive(Debug)]
pub(crate) struct AppData {
    users: HashMap<String, User>,                  // username -> User
    sessions: HashMap<Uuid, Arc<RwLock<Session>>>, // session_id -> Session
}

pub(crate) struct App {
    server: server::Server,
    data: Arc<RwLock<AppData>>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            sessions: HashMap::new(),
        }
    }

    pub fn get_user(&self, username: &str) -> Option<&User> {
        self.users.get(username)
    }

    pub fn get_user_mut(&mut self, username: &str) -> Option<&mut User> {
        self.users.get_mut(username)
    }

    pub fn insert_user(&mut self, username: String, user: User) -> Option<User> {
        self.users.insert(username, user)
    }

    pub fn remove_user(&mut self, username: &str) -> Option<User> {
        self.users.remove(username)
    }

    pub fn get_session(&self, session_id: &Uuid) -> Option<Arc<RwLock<Session>>> {
        self.sessions.get(session_id).cloned()
    }

    pub fn get_session_mut(&mut self, session_id: &Uuid) -> Option<&mut Arc<RwLock<Session>>> {
        self.sessions.get_mut(session_id)
    }

    pub fn insert_session(
        &mut self,
        session_id: Uuid,
        session: Arc<RwLock<Session>>,
    ) -> Option<Arc<RwLock<Session>>> {
        self.sessions.insert(session_id, session)
    }

    pub fn remove_session(&mut self, session_id: &Uuid) -> Option<Arc<RwLock<Session>>> {
        self.sessions.remove(session_id)
    }

    pub fn user_exists(&self, username: &str) -> bool {
        self.users.contains_key(username)
    }

    pub fn get_user_session(&self, username: &str) -> Option<Uuid> {
        self.users.get(username).and_then(|user| user.session())
    }
}

impl App {
    pub fn new() -> Self {
        tracing_subscriber::fmt()
            .with_max_level(TRACING_LEVEL)
            .compact()
            .init();

        Self {
            server: server::Server::new(HOST, PORT),
            data: Arc::new(RwLock::new(AppData::new())),
        }
    }

    pub async fn run(&mut self) -> AnyResult<()> {
        self.server.serve(self.data.clone()).await?;

        Ok(())
    }
}
