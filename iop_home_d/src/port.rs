use connection::Connection;
use error::Result;
use mio::{Token,Poll,Event,PollOpt,Ready};
use mio::tcp::TcpListener;
use reactor::*;

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
    fn register(&self, poll: &Poll, token: Token) -> Result<()> {
        try!(poll.register(&self.server, token, Ready::readable(), PollOpt::edge()));
        Ok(())
    }
    fn act(&self, _: Event) -> Result<()>
    {
        info!("Accepting connection on {}", try!(self.server.local_addr()));
        let (stream, peer_addr) = try!(self.server.accept());
        debug!("Connection from {:?}", peer_addr);

        let mut connection = Connection::new(stream);
        try!(connection.write());
        Ok(())
    }
}
