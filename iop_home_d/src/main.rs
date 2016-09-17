#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mio;

use std::io::prelude::*;
use mio::*;
use mio::tcp::TcpListener;
use error::Result;
use std::collections::HashMap;

pub trait Reactive
{
    fn register(&self, poll: &Poll, token: Token) -> Result<()>;
    fn act(&mut self, event: &Event) -> Result<()>;
}

pub struct Reactor {
    next_token: usize,
    reactives_by_token: HashMap<Token, Box<Reactive>>,
    poll: Poll,
}

impl Reactor {
    pub fn new() -> Result<Reactor> {
        let result = Reactor {
            next_token: 0, 
            reactives_by_token: HashMap::new(),
            poll: try!(Poll::new()),
        };
        Ok(result)
    }
    pub fn register(&mut self, reactive: Box<Reactive>) -> Result<()> {
        let t = self.next_token;
        self.next_token = t + 1;
        let token = Token(t);
        try!((*reactive).register(&self.poll, token));

        let old_reactive = self.reactives_by_token.insert(token, reactive);
        debug_assert!(old_reactive.is_none());

        Ok(())
    }
    pub fn run(&mut self) -> ! {
        let mut events = Events::with_capacity(1024);
        loop {
            self.poll.poll(&mut events, None).unwrap();
            for event in events.iter() {
                if let Some(reactive) = self.reactives_by_token.get_mut(&event.token()) {
                    (*reactive).act(&event).unwrap();
                }
            }
        }

    }
}

#[derive(Debug)]
pub struct Port {
    server: TcpListener,
}

impl Port {
    pub fn new(address: &str) -> Result<Port> {
        let addr = try!(address.parse());
        info!("Binding port {}", addr);
        let server = try!(TcpListener::bind(&addr));

        let result = Port { server: server };
        Ok(result)
    }
}

impl Reactive for Port {
    fn register(&self, poll: &Poll, token: Token) -> Result<()> {
        try!(poll.register(&self.server, token, Ready::readable(), PollOpt::edge()));
        Ok(())
    }
    fn act(&mut self, _: &Event) -> Result<()>
    {
        info!("Accepting connection on {}", try!(self.server.local_addr()));
        let (mut stream, peer_addr) = try!(self.server.accept());
        debug!("Connection from {:?}", peer_addr);
        try!(write!(&mut stream, "Hello, World!"));
        Ok(())
    }
}

pub fn main() {
    env_logger::init().unwrap();

    let port1 = Port::new("0.0.0.0:3000").unwrap();
    let port2 = Port::new("127.0.0.1:3001").unwrap();

    let mut reactor = Reactor::new().unwrap();
    reactor.register(Box::new(port1)).unwrap();
    reactor.register(Box::new(port2)).unwrap();

    reactor.run();
}

mod error;