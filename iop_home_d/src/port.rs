use connection::Connection;
use error::Result;
use mio::{PollOpt,Ready};
use mio::tcp::TcpListener;
use reactor::Reactor;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct _Port {
    server: TcpListener,
}

impl _Port {
    fn act(&mut self, _: Ready, reactor: &mut Reactor) -> Result<()>
    {
        info!("Accepting connection on {}", try!(self.server.local_addr()));
        let (stream, peer_addr) = try!(self.server.accept());
        debug!("Connection from {:?}", peer_addr);

        let mut connection = Connection::new(stream);
        try!(connection.write());

        try!(connection.register(reactor));
        Ok(())
    }
}

#[derive(Debug)]
pub struct Port (Rc<RefCell<_Port>>);

impl Port {
    pub fn new(address: &str) -> Result<Port> {
        let addr = try!(address.parse());
        info!("Binding port {}", addr);
        let server = try!(TcpListener::bind(&addr));

        let result = Port(Rc::new(RefCell::new(_Port { server: server } )));
        Ok(result)
    }
    
    pub fn register(&self, reactor: &mut Reactor) -> Result<()> {
        let this = self.0.clone();
        try!(reactor.add(&self.0.borrow().server, Ready::readable(), PollOpt::edge(), move |kind, reactor| this.borrow_mut().act(kind, reactor)));
        Ok(())
    }
}
