use std::{cell::RefCell, collections::HashMap, ptr::hash, rc::Rc, str, usize};

pub mod state;

pub struct EngineNFA<'a> {
    states: HashMap<&'a str, Rc<RefCell<state::State<'a>>>>,
    pub initial_state: &'a str,
    pub ending_states: Vec<&'a str>,
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
        println!("for engine with initial {}", self.initial_state);
        println!("init {}", name);
        self.states.insert(name, Rc::new(RefCell::new(state::State::new(name))));
        println!("states {:?}", self.states.keys());
    }

    pub fn declare_states(&mut self, names: Vec<&'a str>) {
        names.into_iter().for_each(|n| self.add_state(n));
    }

    pub fn add_transition(&mut self, from_state: &str, to_state: &str, matcher: Box<dyn state::Matcher>) {
        println!("{}, {}", from_state, to_state);
        // println!("the states are {:?}", self.states.keys());
        self.states[from_state].borrow_mut().add_transition(self.states[to_state].clone(), dyn_clone::clone_box(&*matcher));
    }

    pub fn pushfront_transition(&mut self, from_state: &str, to_state: &str, matcher: Box<dyn state::Matcher>) {
        self.states[from_state].borrow_mut().pushfront_transition(self.states[to_state].clone(), matcher);
    }

    pub fn append_nfa(&mut self, other_nfa: EngineNFA<'a>, union_state: &'a str) {
        for (s, state) in other_nfa.states.iter() {
            // self.add_state(s);
            self.states.insert(s, state.clone());
        }

        println!("Removing {}", other_nfa.initial_state);
        println!("union {}", union_state);

        self.states.remove(other_nfa.initial_state);

        println!("the states are {:?}", self.states.keys());
        for (matcher, to_transition) in other_nfa.states[other_nfa.initial_state].borrow().transitions.iter() {
            self.add_transition(union_state, to_transition.borrow().name, dyn_clone::clone_box(&**matcher));
        }

        if !self.ending_states.is_empty() && self.ending_states.contains(&union_state) {
            let i = self.ending_states.iter().position(|x| *x == union_state).unwrap();
            self.ending_states.splice(i..i+(1 as usize), other_nfa.ending_states);
        }
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
