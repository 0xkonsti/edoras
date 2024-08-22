use crate::application::{AppData, CONNECTION_LIMIT, HEALTH_CHECK_INTERVAL};
use crate::handlers::handle_message;
use crate::session::Session;
use anyhow::Result as AnyResult;
use async_std::net::TcpListener;
use async_std::sync::RwLock;
use async_std::task;
use edoras_core::Message;
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
                        let session = Arc::new(RwLock::new(Session::new(stream)));
                        task::spawn(Self::health_check(session.clone(), data.clone()));
                        task::spawn(Self::handle_connection(session.clone(), data));
                    }
                }
            })
            .await;

        Ok(())
    }

    async fn handle_connection(session: Arc<RwLock<Session>>, appdata: Arc<RwLock<AppData>>) {
        if let Err(e) = session.read().await.peer_addr() {
            tracing::error!("Failed to get peer address: {}", e);
            return;
        }

        appdata
            .write()
            .await
            .insert_session(session.read().await.id(), session.clone());

        let addr = session.read().await.peer_addr().unwrap().clone();
        tracing::info!("New connection from {}", addr);

        while !session.read().await.closed() {
            // TODO: Check if the client is still connected > send ping to client on an interval
            if !session.write().await.recieved_msg().await {
                continue;
            }

            let msg = match session.write().await.recv().await {
                Ok(msg) => msg,
                Err(e) => {
                    tracing::error!("Failed to receive message from {}: {}", addr, e);
                    session.write().await.close();
                    break;
                }
            };

            handle_message(session.clone(), appdata.clone(), &msg).await;

            println!("{:#?}", appdata.read().await);
        }

        tracing::info!("Connection from {} closed", addr);
        if let Err(e) = session.write().await.shutdown() {
            tracing::error!("Failed to close connection from {}: {}", addr, e);
        }

        println!("{:#?}", appdata.read().await);
    }

    async fn health_check(session: Arc<RwLock<Session>>, appdata: Arc<RwLock<AppData>>) {
        loop {
            task::sleep(HEALTH_CHECK_INTERVAL).await;

            if session.read().await.closed() {
                break;
            }

            if !session.write().await.health_check().await {
                tracing::error!(
                    "Health check failed for {}",
                    session.read().await.peer_addr().unwrap()
                );

                handle_message(
                    session.clone(),
                    appdata.clone(),
                    &Message::DISCONNECT_MESSAGE,
                )
                .await;

                break;
            }
        }
    }
}
