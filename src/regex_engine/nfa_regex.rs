use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use regex_syntax::hir::{Hir, HirKind};

use crate::nfa_engine::state::{self, Matcher};

use super::nfa_engine::EngineNFA;

pub struct NFABuilder {
    state_number: u32,
    states: VecDeque<String>,
}

impl<'a> NFABuilder {
    pub fn new() -> Self {
        NFABuilder {
            state_number: 0,
            states: VecDeque::new(),
        }
    }

    pub fn new_state(&mut self) -> String {
        let c = format!("q{}", self.state_number);
        self.state_number += 1;
        return c;
    }

    pub fn reset_state_numbers(&mut self) {
        self.state_number = 0;
    }

    pub fn regex_to_nfa(self_: &'a Rc<RefCell<NFABuilder>>, regex_ast: &Hir) -> EngineNFA<'a>{
        self_.borrow_mut().reset_state_numbers();

        NFABuilder::_regex_to_nfa(self_, regex_ast)
    }

    fn _regex_to_nfa(self_: &'a Rc<RefCell<NFABuilder>>, regex_ast: &Hir) -> EngineNFA<'a> {

        NFABuilder::_single_regex_to_nfa(self_, regex_ast)

    }

    fn _alternative_to_nfa(self_: &'a Rc<RefCell<NFABuilder>>, regex_ast: &Vec<Hir>) -> EngineNFA<'a> {
        unsafe {
            let start = (*(self_.as_ref().as_ptr())).new_state();
            self_.borrow_mut().states.push_front(start);
            let states_length = self_.borrow().states.len();
            println!("this before btw {}", (*(self_.as_ref().as_ptr())).states[0].as_str());
            let mut nfa = EngineNFA::new((*(self_.as_ref().as_ptr())).states[0].as_str(), vec![]);

            let mut ending_states = vec![];

            nfa.add_state((*(self_.as_ref().as_ptr())).states[0].as_str());

            for alt in regex_ast {
                let mut temp = NFABuilder::_single_regex_to_nfa(self_, alt);
                ending_states.append(&mut temp.ending_states);
                let increase = self_.borrow().states.len() - states_length;
                println!("this after btw {}", (*(self_.as_ref().as_ptr())).states[increase].as_str());

                nfa.append_nfa(temp, (*(self_.as_ref().as_ptr())).states[increase].as_str());
            }

            let end = (*(self_.as_ref().as_ptr())).new_state();
            self_.borrow_mut().states.push_front(end);

            // println!("doing it! {}", (*(self_.as_ref().as_ptr())).states[0].as_str());
            nfa.add_state((*(self_.as_ref().as_ptr())).states[0].as_str());

            ending_states.iter().for_each(|x| nfa.add_transition(*x, (*(self_.as_ref().as_ptr())).states[0].as_str(), 
                Box::new(state::EpsilonMatcher {})));

            nfa.ending_states.append(&mut ending_states);

            nfa
        }
    }

    // _singleRegexToNFA(regexAST) {
    fn _single_regex_to_nfa(self_: &'a Rc<RefCell<NFABuilder>>, regex_ast: &Hir) -> EngineNFA<'a> {
        // let mut nfa: EngineNFA;

        match regex_ast.kind() {
            // HirKind::Empty => self_.clone().borrow_mut()._empty_expression(),
            HirKind::Empty => NFABuilder::_empty_expression(self_),
            HirKind::Literal(x) => NFABuilder::_one_step_nfa(self_, Box::new(state::CharacterMatcher::new(x.0[0] as char))),
            HirKind::Concat(xs) => {
                let mut first = NFABuilder::_single_regex_to_nfa(self_, &xs[0]);
                for i in 1..xs.len() {
                    let union_state = first.ending_states[0];
                    first.append_nfa(NFABuilder::_single_regex_to_nfa(self_, &xs[i]), union_state)
                }

                first
            },
            HirKind::Alternation(x) => NFABuilder::_alternative_to_nfa(self_, x),
            HirKind::Repetition(x) => NFABuilder::_asterisk_plus(self_, NFABuilder::_single_regex_to_nfa(self_, &*(x.sub)), false, 
                if x.min == 0 { true } else { false }),
            HirKind::Look(x) => todo!(),
            HirKind::Class(x) => todo!(),
            HirKind::Capture(x) => todo!(),
        }

        // iterate over subpatterns and do stuff
        //         for c in regex_ast {
        // 
        //         }
        // 
    }

    fn _atomic_pattern_nfa() { }

    fn _empty_expression(self_: &'a Rc<RefCell<NFABuilder>>) -> EngineNFA<'a> {
        NFABuilder::_one_step_nfa(&self_, Box::new(state::EpsilonMatcher {}))
    }

    fn _one_step_nfa(self_: &'a Rc<RefCell<NFABuilder>>, matcher: Box<dyn Matcher>) -> EngineNFA<'a> {
        let a = self_.borrow_mut().new_state();
        let b = self_.borrow_mut().new_state();

        self_.borrow_mut().states.push_front(b);
        self_.borrow_mut().states.push_front(a);

        unsafe {
            let mut nfa = EngineNFA::new((*(self_.as_ref().as_ptr())).states[0].as_str(), 
                vec![(*(self_.as_ref().as_ptr())).states[1].as_str()]);

            nfa.declare_states(vec![(*(self_.as_ref().as_ptr())).states[0].as_str(), 
                (*(self_.as_ref().as_ptr())).states[1].as_str()]);
            nfa.add_transition(self_.borrow().states[0].as_str(), self_.borrow().states[1].as_str(), matcher);

            nfa
        }
    }

    fn _asterisk_plus(self_: &'a Rc<RefCell<NFABuilder>>, mut base: EngineNFA<'a>, lazy: bool, asterisk: bool) -> EngineNFA<'a> {
        let new_init = self_.borrow_mut().new_state();
        let new_end = self_.borrow_mut().new_state();

        self_.borrow_mut().states.push_front(new_end);
        self_.borrow_mut().states.push_front(new_init);

        unsafe {
            let new_init = (*(self_.as_ref().as_ptr())).states[0].as_str();
            let new_end = (*(self_.as_ref().as_ptr())).states[1].as_str();

            base.add_state(new_init);
            base.add_state(new_end);

            if lazy {
                if asterisk {
                    base.add_transition(new_init, new_end, Box::new(state::EpsilonMatcher {}));
                }
                base.add_transition(new_init, base.initial_state, Box::new(state::EpsilonMatcher {}));
                base.add_transition(base.ending_states[0], new_end, Box::new(state::EpsilonMatcher {}));
                base.add_transition(base.ending_states[0], base.initial_state, Box::new(state::EpsilonMatcher {}));
            } else {
                base.add_transition(new_init, base.initial_state, Box::new(state::EpsilonMatcher {}));
                base.add_transition(base.ending_states[0], base.initial_state, Box::new(state::EpsilonMatcher {}));
                base.add_transition(base.ending_states[0], new_end, Box::new(state::EpsilonMatcher {}));
                if asterisk {
                    base.add_transition(new_init, new_end, Box::new(state::EpsilonMatcher {}));
                }
            }

            base.initial_state = new_init;
            base.ending_states = vec![new_end];

            base
        }

    }

}

pub struct NFARegex<'a> {
    source: &'static str,
    nfa: EngineNFA<'a>,
}

impl<'a> NFARegex<'a> {
    pub fn new(source: &'static str, builder: &'a Rc<RefCell<NFABuilder>>) -> Self {
        NFARegex {
            source,
            nfa: {
                // let mut builder = NFABuilder::new();
                let mut parser = regex_syntax::Parser::new();
                let ast = parser.parse(source).unwrap();
                println!("{:#?}", ast);
                NFABuilder::regex_to_nfa(builder, &ast)
            },
        }
    }

    pub fn compute(&mut self, input: String) -> bool {
        self.nfa.compute(input)
    }
}
