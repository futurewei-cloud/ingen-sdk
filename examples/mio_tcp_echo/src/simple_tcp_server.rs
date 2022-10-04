use anyhow::Result;
use bytes::{Bytes, BytesMut};
use log::*;
use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use std::collections::HashMap;
use std::io::{self, Read, Write};

// Setup some tokens to allow us to identify which event is for which socket.
const SERVER: Token = Token(0);

type SessionHandlerFactory = Box<dyn Fn() -> Box<dyn SessionHandler>>;

pub trait SessionHandler {
    fn handle_incoming_data(&mut self, incoming: &[u8]) -> Result<(usize, Bytes)>;
}

pub struct SimpleTcpServer {
    server_endpoint: String,
    poll: Poll,

    // Token to TcpStream mapping
    sessions: HashMap<Token, SessionContext>,
    session_handler_factory: SessionHandlerFactory,
}

impl SimpleTcpServer {
    pub fn new(server_endpoint: &str, session_handler_factory: SessionHandlerFactory) -> Self {
        SimpleTcpServer {
            server_endpoint: server_endpoint.to_string(),
            poll: Poll::new().unwrap(),
            sessions: HashMap::new(),
            session_handler_factory,
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let mut server = TcpListener::bind(self.server_endpoint.parse()?)?;

        // Register the server with poll we can receive events for it.
        debug!("Register server listener to poll: {:?}", server);
        self.poll
            .registry()
            .register(&mut server, SERVER, Interest::READABLE)?;

        // Unique token for each incoming connection.
        let mut unique_token = Token(SERVER.0 + 1);
        let mut events = Events::with_capacity(128);

        loop {
            self.poll.poll(&mut events, None)?;

            for event in events.iter() {
                match event.token() {
                    SERVER => loop {
                        let (connection, address) = match server.accept() {
                            Ok((connection, address)) => (connection, address),
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                break;
                            }
                            Err(e) => {
                                return Err(anyhow::Error::new(e));
                            }
                        };

                        info!("New connection accepted from: {}", address);

                        let token = next(&mut unique_token);
                        let registry = self.poll.registry();
                        let handler = (self.session_handler_factory)();
                        let session = SessionContext::new(token, connection, registry, handler)?;

                        self.sessions.insert(token, session);
                        debug!("Connection registered with token: {:?}", token);
                    },
                    token => {
                        let done = self.handle_connection_event(event)?;

                        if done {
                            if let Some(mut session) = self.sessions.remove(&token) {
                                session.deregister(self.poll.registry())?;
                            }
                        }
                    }
                }
            }
        }
    }

    /// Returns `true` if the connection is done.
    fn handle_connection_event(&mut self, event: &Event) -> Result<bool> {
        let token = event.token();
        let session = match self.sessions.get_mut(&token) {
            Some(v) => v,
            None => return Ok(false),
        };

        let registry = self.poll.registry();

        if event.is_writable() {
            session.on_write_ready(registry)?;
        } else if event.is_readable() {
            if session.on_read_ready(registry)? {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

struct SessionContext {
    token: Token,
    connection: TcpStream,

    read_buf: BytesMut,
    read_buf_start: usize,
    next_read_start: usize,

    write_buf: BytesMut,
    write_buf_start: usize,

    handler: Box<dyn SessionHandler>,
}

impl SessionContext {
    pub fn new(
        token: Token,
        connection: TcpStream,
        registry: &Registry,
        handler: Box<dyn SessionHandler>,
    ) -> Result<Self> {
        let initial_buf_size = 512;

        let mut context = SessionContext {
            token,
            connection,
            read_buf: BytesMut::with_capacity(initial_buf_size),
            read_buf_start: 0,
            next_read_start: 0,
            write_buf: BytesMut::with_capacity(initial_buf_size),
            write_buf_start: 0,
            handler,
        };

        context.read_buf.resize(initial_buf_size, 0);

        registry.register(&mut context.connection, context.token, Interest::READABLE)?;

        Ok(context)
    }

    pub fn has_pending_read_data(&self) -> bool {
        self.read_buf_start < self.next_read_start
    }

    pub fn has_pending_write_data(&self) -> bool {
        self.write_buf_start < self.write_buf.len()
    }

    pub fn register_read(&mut self, registry: &Registry) -> Result<()> {
        debug!("Registering connection for read: {:?}", self.connection);
        registry.reregister(&mut self.connection, self.token, Interest::READABLE)?;
        Ok(())
    }

    pub fn register_write(&mut self, registry: &Registry) -> Result<()> {
        debug!("Registering connection for write: {:?}", self.connection);
        registry.reregister(&mut self.connection, self.token, Interest::WRITABLE)?;
        Ok(())
    }

    pub fn deregister(&mut self, registry: &Registry) -> Result<()> {
        debug!("Deregistering connection: {:?}", self.connection);
        registry.deregister(&mut self.connection)?;
        Ok(())
    }

    pub fn on_read_ready(&mut self, registry: &Registry) -> Result<bool> {
        debug!(
            "Connection is ready to read. Connection = {:?}",
            self.connection
        );

        let mut connection_closed = false;

        // We can (maybe) read from the connection.
        let mut bytes_read = 0;
        loop {
            match self
                .connection
                .read(&mut self.read_buf[self.next_read_start..])
            {
                Ok(0) => {
                    // Reading 0 bytes means the other side has closed the connection or is done writing, then so are we.
                    connection_closed = true;
                    break;
                }
                Ok(n) => {
                    debug!(
                        "Got data from connection: Connection = {:?}, Size = {}",
                        self.connection, n
                    );

                    bytes_read += n;
                    self.next_read_start += n;
                    if self.next_read_start == self.read_buf.len() {
                        self.read_buf.resize(self.read_buf.len() + 1024, 0);
                    }
                }
                // Would block "errors" are the OS's way of saying that the
                // connection is not actually ready to perform this I/O operation.
                Err(ref err) if would_block(err) => break,
                Err(ref err) if interrupted(err) => break,
                // Other errors we'll consider fatal.
                Err(err) => return Err(anyhow::Error::new(err)),
            }
        }

        if bytes_read != 0 {
            loop {
                let received_data = &self.read_buf[self.read_buf_start..self.next_read_start];
                match self.handler.handle_incoming_data(received_data) {
                    Ok((consumed, response)) => {
                        debug!(
                            "Incoming data is handled successfully: Connection = {:?}, Consumed = {}, ResponseLength = {}",
                            self.connection,
                            consumed,
                            response.len());

                        // If we have any response, append it to write buffer.
                        if response.len() > 0 {
                            self.write_buf.extend_from_slice(&response[..]);
                        }

                        // If we cannot consume any data now, it means we need more data to proceed. Break to read more.
                        if consumed == 0 {
                            break;
                        }

                        self.read_buf_start += consumed;

                        // If all data is processed, we can reset the read buffer to save some memory usage.
                        if !self.has_pending_read_data() {
                            self.read_buf_start = 0;
                            self.next_read_start = 0;
                            debug!("No more data to read. Break incoming data handling loop: Connection = {:?}", self.connection);
                            break;
                        }
                    }
                    Err(e) => {
                        // Invalid data received, we need to force terminate the connection.
                        error!("Invalid data received! Incoming data handler throws failures. Force terminating connection: {:?}, Error = {:?}", self.connection, e);

                        connection_closed = true;
                    }
                }
            }

            // If we have responses to send, we switch to send the responses back.
            if self.has_pending_write_data() {
                self.register_write(registry)?;
            }
        }

        if connection_closed {
            info!("Connection closed: Connection = {:?}", self.connection);
            return Ok(true);
        }

        Ok(false)
    }

    pub fn on_write_ready(&mut self, registry: &Registry) -> Result<()> {
        debug!(
            "Connection is ready to write. Connection = {:?}",
            self.connection
        );

        // If we have nothing to write, we reset the write buf and register for read and move on.
        if !self.has_pending_write_data() {
            self.write_buf_start = 0;
            self.write_buf.clear();

            self.register_read(registry)?;
            return Ok(());
        }

        // We can (maybe) write to the connection.
        loop {
            let to_write = &self.write_buf[self.write_buf_start..];

            match self.connection.write(to_write) {
                Ok(n) => {
                    self.write_buf_start += n;

                    // If we completed writing everything, we reset the write buffer and register read.
                    if self.write_buf_start == self.write_buf.len() {
                        self.write_buf_start = 0;
                        self.write_buf.clear();

                        self.register_read(registry)?;
                        break;
                    }
                }
                // Would block "errors" are the OS's way of saying that the
                // connection is not actually ready to perform this I/O operation.
                Err(ref err) if would_block(err) => break,
                // Got interrupted (how rude!). We will wait for our next run.
                Err(ref err) if interrupted(err) => break,
                // Other errors we'll consider fatal.
                Err(err) => return Err(anyhow::Error::new(err)),
            }
        }

        Ok(())
    }
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}

fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}
