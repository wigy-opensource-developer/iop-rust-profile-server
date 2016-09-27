use error::{Error,Result};
use mio::{Token,Poll,PollOpt,Ready};
use mio::tcp::TcpStream;
use reactor::{Reactor,Reactive};
use std::io::ErrorKind;
use std::io::prelude::*;
use std::str;

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

impl Reactive for Connection {
    fn register(&self, poll: &Poll, token: Token) -> Result<()> {
        try!(poll.register(&self.stream, token, Ready::readable(), PollOpt::edge()));
        Ok(())
    }
    fn act(&mut self, ready: Ready, _: &mut Reactor) -> Result<()> {
        info!("Got an event {:?}", &ready);
        if ready.is_readable() {
            let mut buf = [0u8; 1024];
            let mut read = String::new();
            loop {
                let result = self.stream.read(&mut buf);
                match result {
                    Ok(read_bytes) => read.push_str(try!(str::from_utf8(&buf[..read_bytes]))),
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => break,
                    Err(e) => return Err(Error::Io(e)),
                }
            }
            info!("read: {}", read);
        }
        // if ready.is_writable() {
            
        // }
        // if ready.is_error() {
            
        // }
        // if ready.is_hup() {
            
        // }
        Ok(())
    }
}