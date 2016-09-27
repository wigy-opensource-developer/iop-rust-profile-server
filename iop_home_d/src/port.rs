use connection::Connection;
use error::Result;
use mio::{Token,PollOpt,Ready};
use mio::tcp::TcpListener;
use reactor::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Port {
    server: TcpListener,
}

impl Port {
    pub fn new(address: &str) -> Result<Port> {
        let addr = try!(address.parse());
        info!("Binding port {}", addr);
        let server = try!(TcpListener::bind(&addr));

        let result = Port {
            server: server,
        };
        Ok(result)
    }
}

impl Reactive for Port {
    fn register(&self, reactor: &mut Reactor) -> Result<Token> {
        let token = try!(reactor.add(&self.server, Ready::readable(), PollOpt::edge()));
        Ok(token)
    }
    fn act(&mut self, _: Ready, reactor: &mut Reactor) -> Result<()>
    {
        info!("Accepting connection on {}", try!(self.server.local_addr()));
        let (stream, peer_addr) = try!(self.server.accept());
        debug!("Connection from {:?}", peer_addr);

        let mut connection = Connection::new(stream);
        try!(connection.write());

        try!(reactor.register(Rc::new(RefCell::new(connection))));
        Ok(())
    }
}
