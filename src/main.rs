mod nfa_engine;

use nfa_engine::state;

fn main() {
    let mut engine = nfa_engine::EngineNFA::new("q0", vec!["q3"]);

    engine.declare_states(vec!["q0", "q1", "q2", "q3"]);

    engine.add_transition("q0", "q1", Box::new(state::CharacterMatcher::new('a')));
    engine.add_transition("q1", "q2", Box::new(state::CharacterMatcher::new('b')));
    engine.add_transition("q2", "q2", Box::new(state::CharacterMatcher::new('b')));
    engine.add_transition("q2", "q3", Box::new(state::EpsilonMatcher {}));

    println!("For abbbbbb: {}", engine.compute("abbbbbb".to_string()));
    println!("For aabbbbbb: {}", engine.compute("aabbbbbb".to_string()));
    println!("For ab: {}", engine.compute("ab".to_string()));
    println!("For a: {}", engine.compute("a".to_string()));
}
