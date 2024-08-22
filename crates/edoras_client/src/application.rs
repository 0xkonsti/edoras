use anyhow::Result as AnyResult;
use async_std::net::TcpStream;
use edoras_core::{HOST, PORT};

const TRACING_LEVEL: tracing::Level = tracing::Level::DEBUG;

pub(crate) struct AppData {
    stream: Option<TcpStream>,
}

pub(crate) struct App {
    data: AppData,
}

impl AppData {
    pub fn new() -> Self {
        Self { stream: None }
    }

    pub fn stream(&self) -> Option<&TcpStream> {
        self.stream.as_ref()
    }

    pub fn stream_mut(&mut self) -> Option<&mut TcpStream> {
        self.stream.as_mut()
    }

    pub fn set_stream(&mut self, stream: TcpStream) {
        self.stream = Some(stream)
    }
}

impl App {
    pub fn new() -> Self {
        tracing_subscriber::fmt()
            .with_max_level(TRACING_LEVEL)
            .compact()
            .init();

        Self {
            data: AppData::new(),
        }
    }

    pub async fn run(&mut self) -> AnyResult<()> {
        tracing::debug!("Starting application");
        tracing::debug!("Connecting to {}:{}", HOST, PORT);
        self.data
            .set_stream(TcpStream::connect((HOST, PORT)).await?);
        tracing::debug!("Application started");

        Ok(())
    }
}
