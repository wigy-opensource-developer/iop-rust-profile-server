use std::io::prelude::*;
use mio::{Token,Poll,Event,PollOpt,Ready};
use mio::tcp::TcpListener;
use reactor::*;
use error::Result;

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
