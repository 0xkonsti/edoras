use crate::server;
use anyhow::Result as AnyResult;
use edoras_core::{HOST, PORT};

pub(crate) const CONNECTION_LIMIT: usize = 8;

const TRACING_LEVEL: tracing::Level = tracing::Level::DEBUG;

pub(crate) struct App {
    server: server::Server,
}

impl App {
    pub fn new() -> Self {
        tracing_subscriber::fmt()
            .with_max_level(TRACING_LEVEL)
            .compact()
            .init();

        Self {
            server: server::Server::new(HOST, PORT),
        }
    }

    pub async fn run(&self) -> AnyResult<()> {
        self.server.serve().await?;

        Ok(())
    }
}
