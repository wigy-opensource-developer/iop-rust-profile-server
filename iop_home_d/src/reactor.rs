use error::Result;
use mio::*;
use std::collections::HashMap;
use std::rc::Rc;

type Reaction = Rc<Fn(Ready, &mut Reactor) -> Result<()>>;

pub struct Reactor {
    next_token: usize,
    reactions: HashMap<Token, Reaction>,
    poll: Poll,
}

impl Reactor {
    pub fn new() -> Result<Reactor> {
        let result = Reactor {
            next_token: 0, 
            reactions: HashMap::new(),
            poll: try!(Poll::new()),
        };
        Ok(result)
    }

    pub fn add<R>(&mut self, evented: &Evented, kind: Ready, opt: PollOpt, reaction: R) -> Result<Token>
        where R : Fn(Ready, &mut Reactor) -> Result<()> + 'static
    {
        let token = self.create_token();
        try!(self.poll.register(evented, token, kind, opt));

        let old_reaction = self.reactions.insert(token, Rc::new(reaction));
        debug_assert!(old_reaction.is_none());
        
        Ok(token)
    }

    // pub fn remove(&mut self, evented: &Evented) -> Result<()>
    // {
    //     try!(self.poll.deregister(evented));        
    //     Ok(())
    // }

    pub fn run(&mut self) -> ! {
        let mut events = Events::with_capacity(1024);
        loop {
            self.poll.poll(&mut events, None).unwrap();
            let mut action_needed : Vec<(Reaction, Ready)> = Vec::new();
            for event in events.iter() {
                if let Some(reaction) = self.reactions.get(&event.token()) {
                    action_needed.push((reaction.clone(), event.kind()));
                }
            }
            for (reaction, kind) in action_needed {
                reaction(kind, self).unwrap();
            }
        }
    }

    fn create_token(&mut self) -> Token {
        let t = self.next_token;
        self.next_token = t + 1;
        Token(t)
    }
}
