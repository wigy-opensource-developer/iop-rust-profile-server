#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mio;

use port::Port;
use reactor::Reactor;

trait ReactorExtensions {
    fn bind_port(&mut self, addr: &str);
}

impl ReactorExtensions for Reactor {
    fn bind_port(&mut self, addr: &str) {
        let port = Port::bind(addr).unwrap();
        port.register(self).unwrap();
    }
}

pub fn main() {
    env_logger::init().unwrap();

    let mut reactor = Reactor::new().unwrap();
    reactor.bind_port("0.0.0.0:3000");
    reactor.bind_port("127.0.0.1:3001");
    reactor.bind_port("[::]:3002");

    reactor.run();
}

pub mod error;
pub mod reactor;
pub mod port;
pub mod connection;