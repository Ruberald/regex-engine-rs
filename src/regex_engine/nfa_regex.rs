use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use regex_syntax::hir::{Hir, HirKind};

use crate::nfa_engine::state::{self, Matcher};

use super::nfa_engine::EngineNFA;

struct NFABuilder {
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

    pub fn regex_to_nfa(&mut self, regex_ast: Hir) -> EngineNFA<'_>{
        self.reset_state_numbers();

        self._regex_to_nfa(regex_ast)
    }

    fn _regex_to_nfa(&mut self, regex_ast: Hir) -> EngineNFA<'_> {
        // GOTTA FUCKING FIGURE IT OUT 
        // BECAUSE MY BOOK DECIDED TO GIVE UP
        // if regex_ast.

        EngineNFA::new("", vec![])
    }

    fn _alternative_to_nfa(&mut self, regex_ast: Hir) {

    }

    // _singleRegexToNFA(regexAST) {
    fn _single_regex_to_nfa(self_: &'a Rc<RefCell<NFABuilder>>, regex_ast: &Hir) -> EngineNFA<'a> {
        // let mut nfa: EngineNFA;

        match regex_ast.kind() {
            // HirKind::Empty => self_.clone().borrow_mut()._empty_expression(),
            HirKind::Empty => NFABuilder::_empty_expression(&self_),
            HirKind::Literal(x) => NFABuilder::_one_step_nfa(&self_, Box::new(state::CharacterMatcher::new(x.0[0] as char))),
            HirKind::Concat(xs) => {
                let mut first = NFABuilder::_single_regex_to_nfa(&self_, xs.first().unwrap());
                for i in 1..xs.len() {
                    let union_state = first.ending_states[0];
                    first.append_nfa(NFABuilder::_single_regex_to_nfa(&self_, &xs[i]), union_state)
                }

                first
            },
            HirKind::Look(x) => todo!(),
            HirKind::Class(x) => todo!(),
            HirKind::Capture(x) => todo!(),
            HirKind::Repetition(x) => todo!(),
            HirKind::Alternation(x) => todo!(),
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

        self_.borrow_mut().states.push_front(a);
        self_.borrow_mut().states.push_front(b);

        unsafe {
            let mut nfa = EngineNFA::new((*(self_.as_ref().as_ptr())).states[0].as_str(), 
                vec![(*(self_.as_ref().as_ptr())).states[1].as_str()]);

            nfa.declare_states(vec![(*(self_.as_ref().as_ptr())).states[0].as_str(), 
                (*(self_.as_ref().as_ptr())).states[1].as_str()]);
            nfa.add_transition(self_.borrow_mut().states[0].as_str(), self_.borrow_mut().states[1].as_str(), matcher);

            nfa
        }
    }

    // fn append_nfa(&mut self, other_nfa: EngineNFA, union_state: State) {

    // }

}

struct NFARegex<'a> {
    source: &'static str,
    nfa: EngineNFA<'a>,
}

impl<'a> NFARegex<'a> {
    pub fn new(source: &'static str, builder: &'a mut NFABuilder) -> Self {
        NFARegex {
            source,
            nfa: {
                // let mut builder = NFABuilder::new();
                let mut parser = regex_syntax::Parser::new();
                let ast = parser.parse(source).unwrap();
                builder.regex_to_nfa(ast)
            },
        }
    }


}
