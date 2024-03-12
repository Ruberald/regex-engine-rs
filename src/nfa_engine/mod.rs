use std::{cell::RefCell, collections::HashMap, rc::Rc};

mod state;

struct EngineNFA<'a> {
    states: HashMap<String, Rc<RefCell<state::State<'a>>>>,
    initial_state: &'a state::State<'a>,
    ending_states: Vec<&'a state::State<'a>>,
}

impl<'a> EngineNFA<'a> {
    fn new(initial_state: &'a state::State<'a>, ending_states: Vec<&'a state::State<'a>>) -> Self {
        EngineNFA {
            states: HashMap::new(),
            initial_state,
            ending_states,
        }
    }

    fn add_state(&mut self, name: String) {
        self.states.insert(name.clone(), Rc::new(RefCell::new(state::State::new(name.as_str()))));
    }

    fn declare_states(&mut self, names: Vec<String>) {
        names.into_iter().for_each(|n| self.add_state(n));
    }

    fn add_transition(&'a mut self, from_state: &String, to_state: &String, matcher: &'a dyn state::Matcher) {
        self.states[from_state].borrow_mut().add_transition(self.states[to_state].clone(), matcher)
    }

    fn pushfront_transition(&'a mut self, from_state: &String, to_state: &String, matcher: &'a dyn state::Matcher) {
        self.states[from_state].borrow_mut().pushfront_transition(self.states[to_state].clone(), matcher)
    }

    fn compute(input: String) -> bool {
        todo!("Can't compute yet!")
    }
}
