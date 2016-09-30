use connection::Connection;
use error::Result;
use mio::{PollOpt,Ready};
use mio::tcp::TcpListener;
use reactor::Reactor;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Port (Rc<RefCell<_Port>>);

impl Port {
    pub fn bind(address: &str) -> Result<Port> {
        let addr = try!(address.parse());
        info!("Binding port {}", addr);
        let server = try!(TcpListener::bind(&addr));

        let result = Port(Rc::new(RefCell::new(_Port { server: server } )));
        Ok(result)
    }
    
    pub fn register(&self, reactor: &mut Reactor) -> Result<()> {
        let impl1 = self.0.clone();
        try!(reactor.add(
            &self.0.borrow().server,
            Ready::readable(),
            PollOpt::edge(),
            move |_kind, reactor| impl1.borrow_mut().on_accept(reactor))
        );
        Ok(())
    }
}

#[derive(Debug)]
struct _Port {
    server: TcpListener,
}

impl _Port {
    fn on_accept(&mut self, reactor: &mut Reactor) -> Result<()>
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
