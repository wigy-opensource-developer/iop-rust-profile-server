#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mio;

use error::Result;
use port::Port;
use reactor::Reactor;
use std::cell::RefCell;
use std::rc::Rc;

pub fn add_port(reactor: &mut Reactor, addr: &str) -> Result<()> {
    let port = try!(Port::new(addr));
    let ref_port = Rc::new(RefCell::new(port));
    try!(reactor.register(ref_port));
    Ok(())
}

pub fn main() {
    env_logger::init().unwrap();

    let mut reactor = Reactor::new().unwrap();
    add_port(&mut reactor, "0.0.0.0:3000").unwrap();
    add_port(&mut reactor, "127.0.0.1:3001").unwrap();
    add_port(&mut reactor, "[::]:3002").unwrap();

    reactor.run();
}

pub mod error;
pub mod reactor;
pub mod port;
pub mod connection;