use std::{
    error::Error,
    fmt::{Debug, Display},
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use tokio::{
    io::{split, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

pub struct ConnectionManager {
    listener: TcpListener,
    reader: Option<ReadHalf<TcpStream>>,
    writer: Option<WriteHalf<TcpStream>>,
}

impl ConnectionManager {
    pub async fn launch(address: impl Into<IpAddr>, port: u16) -> Self {
        let address = address.into();
        let listener = TcpListener::bind((address, port))
            .await
            .unwrap_or_else(|_| panic!("failed to bind to {address}:{port}"));

        Self {
            listener,
            reader: None,
            writer: None,
        }
    }

    pub async fn accept_new_connection(&mut self) -> (ConnectionRead, ConnectionWrite) {
        loop {
            match self.listener.accept().await {
                Ok((socket, addr)) => {
                    let (reader, writer) = split(socket);
                    self.reader = Some(reader);
                    self.writer = Some(writer);

                    let shared_reader = Arc::new(Mutex::new(self.reader.take().unwrap()));
                    let shared_writer = Arc::new(Mutex::new(self.writer.take().unwrap()));

                    return (
                        ConnectionRead::from_reader(shared_reader.clone(), addr),
                        ConnectionWrite::from_writer(shared_writer, addr),
                    );
                }
                Err(err) => {
                    eprintln!("[WARN] failed to connect to client: {err}");
                }
            }
        }
    }
}

pub struct ConnectionRead {
    reader: Arc<Mutex<ReadHalf<TcpStream>>>,
    socket_addr: SocketAddr,
    buffer: Box<[u8; 512]>,
    buflen: usize,
}

#[derive(Debug, Clone)]
pub struct ConnectionWrite {
    writer: Arc<Mutex<WriteHalf<TcpStream>>>,
    socket_addr: SocketAddr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionError {
    ConnectionLost,
    ConnectionClosed,
    MessageTooLong,
    MessageInvalidUtf8,
}

impl Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

impl Error for ConnectionError {}

impl ConnectionRead {
    fn from_reader(reader: Arc<Mutex<ReadHalf<TcpStream>>>, socket_addr: SocketAddr) -> Self {
        Self {
            reader,
            socket_addr,
            buffer: Box::from([0; 512]),
            buflen: 0,
        }
    }

    fn buffer_crlf(&self) -> Option<usize> {
        self.buffer[..self.buflen]
            .windows(2)
            .enumerate()
            .find(|(_, bytes)| bytes[0] == b'\r' && bytes[1] == b'\n')
            .map(|(index, _)| index)
    }

    pub async fn read_message(&mut self) -> Result<String, ConnectionError> {
        use std::io::ErrorKind;

        if self.buffer_crlf().is_none() {
            let n_bytes = loop {
                let mut locked_reader = self.reader.lock().await;
                break match locked_reader.read(&mut self.buffer[self.buflen..]).await {
                    Ok(0) => return Err(ConnectionError::ConnectionClosed),
                    Ok(n_bytes) => n_bytes,
                    Err(err) => {
                        match err.kind() {
                            // Retry `read` if interrupted...
                            ErrorKind::Interrupted => continue,
                            _ => return Err(ConnectionError::ConnectionLost),
                        }
                    }
                };
            };

            self.buflen += n_bytes;
        }

        let end = self.buffer_crlf().ok_or_else(|| {
            // Clear out their data...
            self.buflen = 0;
            ConnectionError::MessageTooLong
        })?;

        let bytes = Vec::from(&self.buffer[0..end]);

        // end + '\r' + '\n'
        let after_crlf = end + 2;

        self.buffer.copy_within(after_crlf..self.buflen, 0);
        self.buflen -= after_crlf;

        let message = String::from_utf8(bytes).map_err(|_| ConnectionError::MessageInvalidUtf8)?;

        Ok(message)
    }

    pub fn id(&self) -> String {
        self.socket_addr.to_string()
    }
}

impl ConnectionWrite {
    fn from_writer(writer: Arc<Mutex<WriteHalf<TcpStream>>>, socket_addr: SocketAddr) -> Self {
        Self {
            writer,
            socket_addr,
        }
    }

    pub fn socket_addr(&self) -> std::net::SocketAddr {
        self.socket_addr
    }
    pub async fn write_message(&mut self, message: &str) -> Result<(), ConnectionError> {
        let mut locked_writer = self.writer.lock().await;
        dbg!(&message);
        locked_writer
            .write_all(message.as_bytes())
            .await
            .map_err(|_| ConnectionError::ConnectionClosed)?;
        let _ = locked_writer.flush().await;
        dbg!("Message sent and flushed");
        Ok(())
    }

    pub fn id(&self) -> String {
        self.socket_addr.to_string()
    }
}
