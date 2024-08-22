use crate::errors::MessageError;
use async_std::net::TcpStream;
use futures::{AsyncReadExt, AsyncWriteExt};

type MessageTypeCode = u8;
type BaseLength = u32;

const HEADER_SIZE: usize = 4;
const MESSAGE_TYPE_SIZE: usize = size_of::<MessageTypeCode>();
const BASE_LENGTH_SIZE: usize = size_of::<BaseLength>();

pub const HEADER: [u8; HEADER_SIZE] = [0x23, 0x3c, 0x21, 0x3e]; // #<!>

const EMPTY: MessageTypeCode = 0x00;
const PING: MessageTypeCode = 0x3c; // >
const PONG: MessageTypeCode = 0x3e; // <
const OKAY: MessageTypeCode = 0x6; // ACK
const ERROR: MessageTypeCode = 0x3f; // ?
const DISCONNECT: MessageTypeCode = 0x1b; // ESC
                                          //
#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    // General
    Empty,
    Ping,
    Pong,
    Okay,
    Error,
    Disconnect,
}

#[derive(Debug, Clone)]
struct MessageField {
    length: BaseLength,
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
struct MessageBody {
    count: BaseLength,
    fields: Vec<MessageField>,
}

/// a message that can be sent/reveived by the erodas-protocol
#[derive(Debug, Clone)]
pub struct Message {
    mtype: MessageType,
    body: MessageBody,
}

pub struct MessageBuilder {
    mtype: MessageType,
    body: MessageBody,
}

// IMPLEMENTATION

impl MessageType {
    fn from_code(code: MessageTypeCode) -> Self {
        match code {
            EMPTY => Self::Empty,
            PING => Self::Ping,
            PONG => Self::Pong,
            OKAY => Self::Okay,
            ERROR => Self::Error,
            DISCONNECT => Self::Disconnect,
            _ => panic!("Unknown message type code: {}", code),
        }
    }

    fn to_code(&self) -> MessageTypeCode {
        match self {
            Self::Empty => EMPTY,
            Self::Ping => PING,
            Self::Pong => PONG,
            Self::Okay => OKAY,
            Self::Error => ERROR,
            Self::Disconnect => DISCONNECT,
        }
    }
}

impl MessageField {
    fn new(data: Vec<u8>) -> Self {
        Self {
            length: data.len() as BaseLength,
            data,
        }
    }

    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.data).to_string()
    }
}

impl MessageBody {
    fn add_field(&mut self, field: MessageField) {
        self.count += 1;
        self.fields.push(field);
    }

    fn add_fields(&mut self, fields: Vec<MessageField>) {
        self.count += fields.len() as BaseLength;
        self.fields.extend(fields);
    }
}

impl Default for MessageBody {
    fn default() -> Self {
        Self {
            count: 0,
            fields: vec![],
        }
    }
}

impl Message {
    pub fn mtype(&self) -> MessageType {
        self.mtype
    }

    pub fn data(&self) -> Vec<Vec<u8>> {
        self.body
            .fields
            .iter()
            .map(|field| field.data.clone())
            .collect()
    }

    pub fn data_ref(&self) -> Vec<&[u8]> {
        self.body
            .fields
            .iter()
            .map(|field| field.data.as_slice())
            .collect()
    }

    pub async fn peek_for_header(stream: &mut TcpStream) -> bool {
        let mut buf = [0u8; HEADER_SIZE];

        if stream.peek(&mut buf).await.is_err() {
            return false;
        }

        buf == HEADER
    }

    pub async fn send(&self, stream: &mut TcpStream) -> Result<(), MessageError> {
        let mut buf: Vec<u8> = vec![];
        buf.extend_from_slice(&HEADER);
        buf.extend_from_slice(&self.mtype.to_code().to_le_bytes());

        buf.extend_from_slice(&self.body.count.to_le_bytes());
        for field in &self.body.fields {
            buf.extend_from_slice(&field.length.to_le_bytes());
            buf.extend_from_slice(&field.data);
        }

        tracing::debug!("Sending message | {:x?}", buf);

        match stream.write_all(&buf).await {
            Ok(_) => Ok(()),
            Err(err) => Err(MessageError::WriteError(err)),
        }
    }

    pub async fn recv(stream: &mut TcpStream) -> Result<Message, MessageError> {
        let mut buf = [0u8; HEADER_SIZE];
        if let Err(e) = stream.read_exact(&mut buf).await {
            return Err(MessageError::ReadError(e));
        }

        if buf != HEADER {
            return Err(MessageError::InvalidMessage(buf.to_vec()));
        }

        tracing::debug!("Header is valid | {:x?}", buf);

        let mut buf = [0u8; MESSAGE_TYPE_SIZE];
        if let Err(e) = stream.read_exact(&mut buf).await {
            return Err(MessageError::ReadError(e));
        }

        let mut mtype: MessageTypeCode = 0;
        for i in 0..MESSAGE_TYPE_SIZE {
            mtype |= (buf[i] as MessageTypeCode) << (i * 8);
        }

        let mut builder = MessageBuilder::new().with_type(MessageType::from_code(mtype));

        tracing::debug!(
            "Message type is valid | {:?}",
            MessageType::from_code(mtype)
        );

        let mut buf = [0u8; BASE_LENGTH_SIZE];
        if let Err(e) = stream.read_exact(&mut buf).await {
            return Err(MessageError::ReadError(e));
        }

        let mut count: BaseLength = 0;
        for i in 0..BASE_LENGTH_SIZE {
            count |= (buf[i] as BaseLength) << (i * 8);
        }

        tracing::debug!(
            "Fields length is valid | {}, Raw: {:?}",
            count,
            buf.to_vec()
        );

        if count == 0 {
            return Ok(builder.build());
        }

        for _ in 0..count {
            let mut buf = [0u8; BASE_LENGTH_SIZE];
            if let Err(e) = stream.read_exact(&mut buf).await {
                return Err(MessageError::ReadError(e));
            }

            let mut length: BaseLength = 0;
            for i in 0..BASE_LENGTH_SIZE {
                length |= (buf[i] as BaseLength) << (i * 8);
            }

            tracing::debug!(
                "Field length is valid | {}, Raw: {:?}",
                length,
                buf.to_vec()
            );

            let mut data = vec![0u8; length as usize];
            if let Err(e) = stream.read_exact(&mut data).await {
                return Err(MessageError::ReadError(e));
            }

            tracing::debug!("Field data is valid | {:x?}", data);

            builder = builder.with_field(data);
        }

        Ok(builder.build())
    }
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self {
            mtype: MessageType::Empty,
            body: MessageBody::default(),
        }
    }

    pub fn with_type(mut self, mtype: MessageType) -> Self {
        self.mtype = mtype;
        self
    }

    pub fn with_field(mut self, data: Vec<u8>) -> Self {
        self.body.add_field(MessageField::new(data));
        self
    }

    pub fn with_fields(mut self, fields: Vec<Vec<u8>>) -> Self {
        self.body
            .add_fields(fields.into_iter().map(MessageField::new).collect());
        self
    }

    pub fn build(&self) -> Message {
        Message {
            mtype: self.mtype,
            body: self.body.clone(),
        }
    }
}
