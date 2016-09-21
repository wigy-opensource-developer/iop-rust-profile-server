use std::io::prelude::*;
use mio::tcp::TcpStream;
use error::Result;

#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection { stream : stream }
    }
    pub fn write(&mut self) -> Result<()> {
        try!(write!(&mut self.stream, "Hello, World!"));
        Ok(())
    }
}