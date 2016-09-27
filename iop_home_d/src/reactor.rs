use error::Result;
use mio::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub trait Reactive
{
    fn register(&self, reactor: &mut Reactor) -> Result<Token>;
    fn act(&mut self, ready: Ready, reactor: &mut Reactor) -> Result<()>;
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

    pub fn add(&mut self, evented: &Evented, kind: Ready, opt: PollOpt) -> Result<Token>
    {
        let token = self.create_token();
        try!(self.poll.register(evented, token, kind, opt));

        Ok(token)
    }

    pub fn register(&mut self, reactive: Rc<RefCell<Reactive>>) -> Result<()> {

        let token = try!(reactive.borrow_mut().register(self));

        let old_reactive = self.reactives_by_token.insert(token, reactive);
        debug_assert!(old_reactive.is_none());
        
        Ok(())
    }

    pub fn run(&mut self) -> ! {
        let mut events = Events::with_capacity(1024);
        loop {
            self.poll.poll(&mut events, None).unwrap();
            let mut action_needed : Vec<(Rc<RefCell<Reactive>>, Ready)> = Vec::new();
            for event in events.iter() {
                let maybe_reactive : Option<&Rc<RefCell<Reactive>>> = self.reactives_by_token.get(&event.token());
                if let Some(reactive) = maybe_reactive {
                    action_needed.push((reactive.clone(), event.kind()));
                }
            }
            for (reactive, kind) in action_needed {
                reactive.borrow_mut().act(kind, self).unwrap();
            }
        }
    }

    fn create_token(&mut self) -> Token {
        let t = self.next_token;
        self.next_token = t + 1;
        Token(t)
    }
}
