use std::collections::VecDeque;

struct State<'a> {
    name: String,
    transitions: VecDeque<(&'a Matcher, &'a State<'a>)>,
    starts_group: Vec<&'static str>,
    ends_group: Vec<&'static str>,
}

struct Matcher {

}

impl<'a> State<'a> {
    fn new(name: &str) -> Self {
        State {
            name: name.to_owned(),
            transitions: VecDeque::new(),
            starts_group: Vec::new(),
            ends_group: Vec::new(),
        }
    }

    fn add_transition(&mut self, to_state: &'a State, matcher: &'a Matcher) {
        self.transitions.push_back((matcher, to_state));
    }

    fn pushfront_transition(&mut self, to_state: &'a State, matcher: &'a Matcher) {
        self.transitions.push_front((matcher, to_state));
    }
}

