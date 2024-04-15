use std::{cell::RefCell, collections::HashMap, rc::Rc, usize};

pub mod state;

pub struct EngineNFA<'a> {
    states: HashMap<&'a str, Rc<RefCell<state::State<'a>>>>,
    initial_state: &'a str,
    ending_states: Vec<&'a str>,
}

impl<'a> EngineNFA<'a> {
    pub fn new(initial_state: &'a str, ending_states: Vec<&'a str>) -> Self {
        EngineNFA {
            states: HashMap::new(),
            initial_state,
            ending_states,
        }
    }

    pub fn add_state(&mut self, name: &'a str) {
        self.states.insert(name, Rc::new(RefCell::new(state::State::new(name))));
    }

    pub fn declare_states(&mut self, names: Vec<&'a str>) {
        names.into_iter().for_each(|n| self.add_state(n));
    }

    pub fn add_transition(&mut self, from_state: &str, to_state: &str, matcher: Box<dyn state::Matcher>) {
        self.states[from_state].borrow_mut().add_transition(self.states[to_state].clone(), matcher);
    }

    pub fn pushfront_transition(&mut self, from_state: &str, to_state: &str, matcher: Box<dyn state::Matcher>) {
        self.states[from_state].borrow_mut().pushfront_transition(self.states[to_state].clone(), matcher);
    }

    pub fn compute(&self, input: String) -> bool {
        // todo!("Can't compute yet!");

        let mut stack: Vec<(
        usize, 
        Rc<RefCell<state::State<'a>>>,
        Vec<&'a str>)> = Vec::new();

        stack.push((0, self.states[self.initial_state].clone(), vec![]));

        // while !stack.is_empty() {
        while let Some((i, current_state, epsilon_visited)) = stack.pop() {
            // println!("{}", current_state.borrow().name);
            //if self.ending_states.iter().any(|&i| i == current_state.borrow().name) {
            if self.ending_states.contains(&current_state.borrow().name) { 
                return true;
            }

            let ch = if i >= input.len() {None} else {Some(input.as_bytes()[i] as char)};

            for (matcher, to_state) in current_state.borrow().transitions.iter().rev() {
                if matcher.matches(ch) {

                    let mut copy_mem: Vec<&str>;// = epsilon_visited.clone();
                    if matcher.is_epsilon() {
                        copy_mem = epsilon_visited.clone();
                        if copy_mem.contains(&to_state.borrow().name) {
                            continue;
                        }
                        copy_mem.push(current_state.borrow().name);
                    } else {
                        copy_mem = vec![];
                    }

                    let next_i = if matcher.is_epsilon() {i} else {i+1};
                    stack.push((next_i, to_state.clone(), copy_mem));
                }
            }
        }
        // }

        false
    }
}
