use error::{Error,Result};
use mio::{PollOpt,Ready};
use mio::tcp::TcpStream;
use reactor::Reactor;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::str;

#[derive(Debug)]
struct _Connection {
    stream: TcpStream,
}

impl _Connection {
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

#[derive(Debug)]
pub struct Connection (Rc<RefCell<_Connection>>);

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection( Rc::new(RefCell::new(_Connection { stream : stream })) )
    }
    pub fn write(&mut self) -> Result<()> {
        try!(write!(&mut self.0.borrow_mut().stream, "Hello, World!"));
        Ok(())
    }
    pub fn register(&self, reactor: &mut Reactor) -> Result<()> {
        let this : Rc<RefCell<_Connection>> = self.0.clone();
        try!(reactor.add(&self.0.borrow().stream, Ready::readable(), PollOpt::edge(), move |kind, reactor| this.borrow_mut().act(kind, reactor)));
        Ok(())
    }
}
