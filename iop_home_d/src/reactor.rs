use error::Result;
use mio::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub trait Reactive
{
    fn register(&self, poll: &Poll, token: Token) -> Result<()>;
    fn act(&self, event: Event) -> Result<()>;
}

pub struct Reactor {
    next_token: usize,
    reactives_by_token: HashMap<Token, Rc<RefCell<Reactive>>>,
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
    pub fn register(&mut self, reactive: Rc<RefCell<Reactive>>) -> Result<()> {
        let t = self.next_token;
        self.next_token = t + 1;
        let token = Token(t);
        try!(reactive.borrow_mut().register(&self.poll, token));

        let old_reactive = self.reactives_by_token.insert(token, reactive);
        debug_assert!(old_reactive.is_none());

        Ok(())
    }
    pub fn run(&mut self) -> ! {
        let mut events = Events::with_capacity(1024);
        loop {
            self.poll.poll(&mut events, None).unwrap();
            for event in events.iter() {
                if let Some(reactive) = self.reactives_by_token.get(&event.token()) {
                    reactive.borrow_mut().act(event).unwrap();
                }
            }
        }
    }
}
