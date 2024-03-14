use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub struct State<'a> {
    pub name: &'static str,
    pub transitions: VecDeque<(Box<dyn Matcher>, Rc<RefCell<State<'a>>>)>,
    starts_group: Vec<&'static str>,
    ends_group: Vec<&'static str>,
}

impl<'a> State<'a> {
    pub fn new(name: &'static str) -> Self {
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

pub trait Matcher {
    fn matches(&self, _ch: char) -> bool {
        false
    }

    fn is_epsilon(&self) -> bool {
        false
    }

    fn get_label(&self) -> Option<char> {
        None
    }
}

pub struct CharacterMatcher {
    c: char,
}

impl Matcher for CharacterMatcher {

    fn matches(&self, ch: char) -> bool {
        self.c == ch
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
        CharacterMatcher {
            c,
        }
    }
}

pub struct EpsilonMatcher { }

impl Matcher for EpsilonMatcher {
    fn matches(&self, _ch: char) -> bool {
        true
    }

    fn is_epsilon(&self) -> bool {
        true
    }

    fn get_label(&self) -> Option<char> {
        Some('c')
    }
}

