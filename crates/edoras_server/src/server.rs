use crate::application::{AppData, CONNECTION_LIMIT};
use crate::handlers::handle_message;
use crate::session::Session;
use anyhow::Result as AnyResult;
use async_std::net::TcpListener;
use async_std::sync::RwLock;
use async_std::task;
use futures::StreamExt;
use std::sync::Arc;

pub(crate) struct Server {
    host: &'static str,
    port: u16,
}

impl Server {
    pub fn new(host: &'static str, port: u16) -> Self {
        Self {
            host,
            port,
        }
    }

    pub async fn serve(&self, appdata: Arc<RwLock<AppData>>) -> AnyResult<()> {
        tracing::info!("Starting server on {}:{}", self.host, self.port);
        let listener = TcpListener::bind((self.host, self.port)).await?;
        tracing::info!("Server started");

        listener
            .incoming()
            .for_each_concurrent(Some(CONNECTION_LIMIT), |stream| {
                let data = Arc::clone(&appdata);
                async move {
                    if let Ok(stream) = stream {
                        task::spawn(Self::handle_connection(Session::new(stream), data));
                    }
                }
            })
            .await;

        Ok(())
    }

    async fn handle_connection(mut session: Session, appdata: Arc<RwLock<AppData>>) {
        match session.peer_addr() {
            Ok(addr) => {
                tracing::info!("New connection from {}", addr);
                // TODO: Session needs to be stored in the appdata (likley needs to be changed to Arc<RwLock<Session>>)
                // appdata.write().await.insert_session(session.id(), session);

                while !session.closed() {
                    // TODO: Check if the client is still connected > send ping to client on an interval

                    if !session.recieved_msg().await {
                        continue;
                    }

                    match session.recv().await {
                        Ok(msg) => handle_message(&mut session, appdata.clone(), &msg).await,
                        Err(e) => {
                            tracing::error!("Failed to receive message from {}: {}", addr, e);
                            session.close();
                            break;
                        }
                    }

                    println!("{:?}", appdata.read().await);
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
