use anyhow::Result as AnyResult;
use async_std::channel::{Receiver, Sender};
use async_std::net::TcpStream;
use async_std::sync::{Mutex, RwLock};
use async_std::task;
use edoras_core::{Message, MessageBuilder, MessageType, HOST, PORT};
use std::collections::HashMap;
use std::sync::Arc;

const TRACING_LEVEL: tracing::Level = tracing::Level::DEBUG;

pub(crate) struct AppData {
    send: HashMap<String, Message>, // user_from -> message
    recv: HashMap<String, Message>, // user_to -> message
}

pub(crate) struct Stream {
    stream: Option<TcpStream>,
}

pub(crate) struct App {
    stream: Arc<Mutex<Stream>>,
    data: Arc<RwLock<AppData>>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            send: HashMap::new(),
            recv: HashMap::new(),
        }
    }
}

impl Stream {
    pub fn new() -> Self {
        Self {
            stream: None,
        }
    }

    pub async fn set_stream(&mut self, stream: TcpStream) {
        self.stream = Some(stream);
    }

    pub fn stream(&self) -> &TcpStream {
        self.stream.as_ref().unwrap()
    }

    pub fn stream_mut(&mut self) -> &mut TcpStream {
        self.stream.as_mut().unwrap()
    }
}

impl App {
    pub fn new() -> Self {
        tracing_subscriber::fmt()
            .with_max_level(TRACING_LEVEL)
            .compact()
            .init();

        Self {
            stream: Arc::new(Mutex::new(Stream::new())),
            data: Arc::new(RwLock::new(AppData::new())),
        }
    }

    pub async fn run(&mut self) -> AnyResult<()> {
        tracing::debug!("Starting application");
        tracing::debug!("Connecting to {}:{}", HOST, PORT);
        // self.data
        //     .set_stream(TcpStream::connect((HOST, PORT)).await?);
        let mut stream = self.stream.lock().await;
        stream
            .set_stream(TcpStream::connect((HOST, PORT)).await?)
            .await;
        drop(stream);

        tracing::debug!("Application started");

        let (tx, rx): (Sender<Message>, Receiver<Message>) = async_std::channel::unbounded(); // channel for telling the stream handler to send a message

        let handler = task::spawn(Self::stream_handler(
            self.stream.clone(),
            self.data.clone(),
            rx,
        ));

        if let Err(e) = tx
            .send(
                MessageBuilder::new()
                    .with_type(MessageType::Register)
                    .with_field("luffy")
                    .build(),
            )
            .await
        {
            tracing::error!("Failed to send message to channel: {}", e);
        }

        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            if let Err(e) = tx.send(Message::DISCONNECT_MESSAGE).await {
                tracing::error!("Failed to send message to channel: {}", e);
            } else {
                break;
            }
        }

        handler.await;

        Ok(())
    }

    pub async fn stream_handler(
        stream: Arc<Mutex<Stream>>,
        appdata: Arc<RwLock<AppData>>,
        rx: Receiver<Message>,
    ) {
        let mut stream = stream.lock().await;

        loop {
            if let Ok(msg) = rx.recv().await {
                if let Err(e) = msg.send(stream.stream_mut()).await {
                    tracing::error!("Failed to send message: {}", e);
                }
                if msg.mtype() == MessageType::Disconnect {
                    break;
                }
                continue;
            }

            if !Message::peek_for_header(stream.stream_mut()).await {
                continue;
            }

            let msg = match Message::recv(stream.stream_mut()).await {
                Ok(msg) => msg,
                Err(e) => {
                    tracing::error!("Failed to receive message: {}", e);
                    continue;
                }
            };

            // TODO: handle message
        }
    }
}
