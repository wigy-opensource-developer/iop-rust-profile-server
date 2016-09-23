use error::Result;
use mio::tcp::TcpStream;
use std::io::prelude::*;

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