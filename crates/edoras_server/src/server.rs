use crate::application::CONNECTION_LIMIT;
use crate::session::Session;
use anyhow::Result as AnyResult;
use async_std::net::TcpListener;
use async_std::task;
use futures::StreamExt;

pub(crate) struct Server {
    host: &'static str,
    port: u16,
}

impl Server {
    pub fn new(host: &'static str, port: u16) -> Self {
        Self { host, port }
    }

    pub async fn serve(&self) -> AnyResult<()> {
        tracing::info!("Starting server on {}:{}", self.host, self.port);
        let listener = TcpListener::bind((self.host, self.port)).await?;
        tracing::info!("Server started");

        listener
            .incoming()
            .for_each_concurrent(Some(CONNECTION_LIMIT), |stream| async move {
                if let Ok(stream) = stream {
                    task::spawn(Self::handle_connection(Session::new(stream)));
                }
            })
            .await;

        Ok(())
    }

    // TODO: instead of passing the stream, pass a session object > needs yet to be implemented
    async fn handle_connection(mut session: Session) {
        match session.peer_addr() {
            Ok(addr) => {
                tracing::info!("New connection from {}", addr);

                while !session.closed() {
                    if !session.recieved_msg().await {
                        continue;
                    }

                    todo!("read and process message");
                }

                tracing::info!("Connection closed by {}", addr);
                if let Err(e) = session.shutdown() {
                    tracing::error!("Failed to shutdown connection for {}: {}", addr, e);
                }
            }
            Err(e) => {
                tracing::error!("Failed to get peer address: {}", e);
            }
        }
    }
}
