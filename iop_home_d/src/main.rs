#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mio;

use connection::Connection;
use port::Port;
use reactor::ReactiveSet;
use reactor::Reactor;
use std::cell::RefCell;
use std::rc::Rc;

pub fn create_port(addr: &str) -> Rc<RefCell<Port>> {
    let port = Port::new(addr).unwrap();
    Rc::new(RefCell::new(port))
}

pub fn main() {
    env_logger::init().unwrap();

    let port1 = create_port("0.0.0.0:3000");
    let port2 = create_port("127.0.0.1:3001");
    let port3 = create_port("[::]:3002");
    let _connections: Vec<Connection> = Vec::with_capacity(100);

    let mut reactor = Reactor::new().unwrap();
    reactor.register(port1).unwrap();
    reactor.register(port2).unwrap();
    reactor.register(port3).unwrap();

    reactor.run();
}

pub mod error;
pub mod reactor;
pub mod port;
pub mod connection;