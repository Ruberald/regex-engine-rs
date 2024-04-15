use std::collections::VecDeque;

use regex_syntax::{ast::Ast, hir::Hir};

use crate::nfa_engine::{self, state::{self, Matcher}};

use super::nfa_engine::EngineNFA;

struct NFABuilder {
    state_number: u32,
    states: VecDeque<String>,
}

impl NFABuilder {
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
    fn _single_regex_to_nfa(&mut self, regex_ast: Hir) {
        let mut nfa: EngineNFA;

        // if regex_ast.() {
        //     nfa = self._empty_expression();
        // }

        // iterate over subpatterns and do stuff
//         for c in regex_ast {
// 
//         }
// 
    }

    fn _atomic_pattern_nfa() { }

    fn _empty_expression(&mut self) -> EngineNFA<'_> {
        self._one_step_nfa(Box::new(state::EpsilonMatcher {}))
    }

    fn _one_step_nfa(&mut self, matcher: Box<dyn Matcher>) -> EngineNFA<'_> {
        let a = self.new_state();
        let b = self.new_state();

        self.states.push_front(a);
        self.states.push_front(b);

        let mut nfa = EngineNFA::new(self.states[0].as_str(), vec![self.states[1].as_str()]);

        nfa.declare_states(vec![self.states[0].as_str(), self.states[1].as_str()]);
        nfa.add_transition(self.states[0].as_str(), self.states[1].as_str(), matcher);

        nfa
    }

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
