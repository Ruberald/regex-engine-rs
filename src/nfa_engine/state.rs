use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub struct State<'a> {
    name: String,
    transitions: VecDeque<(&'a dyn Matcher, Rc<RefCell<State<'a>>>)>,
    starts_group: Vec<&'static str>,
    ends_group: Vec<&'static str>,
}

impl<'a> State<'a> {
    pub fn new(name: &str) -> Self {
        State {
            name: name.to_owned(),
            transitions: VecDeque::new(),
            starts_group: Vec::new(),
            ends_group: Vec::new(),
        }
    }

    pub fn add_transition(&mut self, to_state: Rc<RefCell<State<'a>>>, matcher: &'a dyn Matcher) {
        self.transitions.push_back((matcher, to_state));
    }

    pub fn pushfront_transition(&mut self, to_state: Rc<RefCell<State<'a>>>, matcher: &'a dyn Matcher) {
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

struct EpsilonMatcher { }

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

