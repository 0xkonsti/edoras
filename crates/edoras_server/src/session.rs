use async_std::net::TcpStream;

pub(crate) struct Session {
    stream: TcpStream,

    closed: bool,
}

impl Session {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            closed: false,
        }
    }

    pub fn peer_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.stream.peer_addr()
    }

    pub fn closed(&self) -> bool {
        self.closed
    }

    pub fn close(&mut self) {
        self.closed = true;
    }

    pub fn shutdown(&self) -> std::io::Result<()> {
        self.stream.shutdown(std::net::Shutdown::Both)
    }

    pub async fn recieved_msg(&mut self) -> bool {
        unimplemented!()
    }
}
