#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mio;

use reactor::Reactor;
use port::Port;
use connection::Connection;

pub fn main() {
    env_logger::init().unwrap();

    let port1 = Port::new("0.0.0.0:3000").unwrap();
    let port2 = Port::new("127.0.0.1:3001").unwrap();
    let port3 = Port::new("[::]:3002").unwrap();
    let _connections: Vec<Connection> = Vec::with_capacity(100);

    let mut reactor = Reactor::new().unwrap();
    reactor.register(Box::new(port1)).unwrap();
    reactor.register(Box::new(port2)).unwrap();
    reactor.register(Box::new(port3)).unwrap();

    reactor.run();
}

pub mod error;
pub mod reactor;
pub mod port;
pub mod connection;