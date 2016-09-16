#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mio;

use std::io::prelude::*;
use mio::*;
use mio::tcp::TcpListener;
use error::Result;
use std::collections::HashMap;

trait Reactive
{
    fn register(&self, poll: &Poll, token: Token);
    fn act(&mut self, event: &Event);
}

struct Reactor {
    next_token: usize,
    reactives_by_token: HashMap<Token, Box<Reactive>>,
    poll: Poll,
}

impl Reactor {
    fn new() -> Reactor {
        Reactor {
            next_token: 0, 
            reactives_by_token: HashMap::new(),
            poll: Poll::new().unwrap(),
        }
    }
    fn register(&mut self, reactive: Box<Reactive>) {
        let t = self.next_token;
        self.next_token = t + 1;
        let token = Token(t);
        (*reactive).register(&self.poll, token);

        self.reactives_by_token.insert(token, reactive);
    }
    fn run(&mut self) -> ! {
        let mut events = Events::with_capacity(1024);
        loop {
            self.poll.poll(&mut events, None).unwrap();
            for event in events.iter() {
                if let Some(reactive) = self.reactives_by_token.get_mut(&event.token()) {
                    (*reactive).act(&event);
                }
            }
        }

    }
}

#[derive(Debug)]
struct Port {
    token: Token,
    server: TcpListener,
}

impl Port {
    fn new(address: &str) -> Result<Port> {
        let token = Token(0);
        let addr = try!(address.parse());
        info!("Binding port {}", addr);
        let server = try!(TcpListener::bind(&addr));

        let result: Result<Port> = Ok(Port { token: token, server: server });
        result
    }
}

impl Reactive for Port {
    fn register(&self, poll: &Poll, token: Token) {
        poll.register(&self.server, token, Ready::readable(), PollOpt::edge()).unwrap();
    }
    fn act(&mut self, _: &Event)
    {
        info!("Accepting connection on {}", self.server.local_addr().unwrap());
        let (mut stream, peer_addr) = self.server.accept().unwrap();
        debug!("Connection from {:?}", peer_addr);
        stream.write_fmt(format_args!("Hello, World!")).unwrap();
    }
}

fn main() {
    env_logger::init().unwrap();

    let port = Port::new("0.0.0.0:3000").unwrap();
    let mut reactor = Reactor::new();
    reactor.register(Box::new(port));
    reactor.run();
}

mod error;