use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct User {
    username: String,

    session_id: Option<Uuid>,
}

impl User {
    pub fn new(username: String) -> Self {
        Self {
            username,
            session_id: None,
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn session(&self) -> Option<Uuid> {
        self.session_id
    }

    pub fn set_session(&mut self, session_id: Uuid) {
        self.session_id = Some(session_id);
    }

    pub fn clear_session(&mut self) {
        self.session_id = None;
    }
}
