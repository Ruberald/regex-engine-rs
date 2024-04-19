use std::{cell::RefCell, collections::VecDeque, ops::Deref, rc::Rc};

use dyn_clone::DynClone;

pub struct State<'a> {
    pub name: &'a str,
    pub transitions: VecDeque<(Box<dyn Matcher>, Rc<RefCell<State<'a>>>)>,
    starts_group: Vec<&'a str>,
    ends_group: Vec<&'a str>,
}

impl<'a> State<'a> {
    pub fn new(name: &'a str) -> Self {
        State {
            name,
            transitions: VecDeque::new(),
            starts_group: Vec::new(),
            ends_group: Vec::new(),
        }
    }

    pub fn add_transition(&mut self, to_state: Rc<RefCell<State<'a>>>, matcher: Box<dyn Matcher>) {
        self.transitions.push_back((matcher, to_state));
    }

    pub fn pushfront_transition(&mut self, to_state: Rc<RefCell<State<'a>>>, matcher: Box<dyn Matcher>) {
        self.transitions.push_front((matcher, to_state));
    }
}

pub trait Matcher: DynClone {
    fn matches(&self, _ch: Option<char>) -> bool {
        false
    }

    fn is_epsilon(&self) -> bool {
        false
    }

    fn get_label(&self) -> Option<char> {
        None
    }
}

#[derive(Clone)]
pub struct CharacterMatcher {
    c: char,
}

impl Matcher for CharacterMatcher {

    fn matches(&self, ch: Option<char>) -> bool {
        match ch {
            Some(ch) => self.c == ch,
            None => false
        }
    }

    fn is_epsilon(&self) -> bool {
        false
    }

    fn get_label(&self) -> Option<char> {
        Some(self.c)
    }
}

impl CharacterMatcher {
    pub fn new(c: char) -> Self {
        println!("on {c}");
        CharacterMatcher {
            c,
        }
    }
}

#[derive(Clone)]
pub struct EpsilonMatcher { }

impl Matcher for EpsilonMatcher {
    fn matches(&self, _ch: Option<char>) -> bool {
        true
    }

    fn is_epsilon(&self) -> bool {
        true
    }

    fn get_label(&self) -> Option<char> {
        Some('c')
    }
}

