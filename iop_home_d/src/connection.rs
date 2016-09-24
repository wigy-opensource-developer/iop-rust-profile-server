use error::Result;
use mio::{Token,Poll,PollOpt,Ready};
use mio::tcp::TcpStream;
use reactor::{Reactive,ReactiveSet};
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
    fn act(&mut self, ready: Ready, _: &mut ReactiveSet) -> Result<()> {
        info!("Got an event {:?}", &ready);
        if ready.is_readable() {
            let mut buf = [0u8; 1024];
            let mut read = String::new();
            loop {
                let read_bytes = try!(self.stream.read(&mut buf));
                if read_bytes == 0 {
                    break;
                }
                read.push_str(try!(str::from_utf8(&buf[..read_bytes])));
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