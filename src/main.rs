mod nfa_engine;
mod regex_engine;

use std::{cell::RefCell, rc::Rc};

fn main() {

    // let regex_str = r"^[a-z]+\d*$";
    let regex_str = r"a+|b+";

    let builder = regex_engine::nfa_regex::NFABuilder::new();
    let bulder_binding = Rc::new(RefCell::new(builder));
    let mut engine = regex_engine::nfa_regex::NFARegex::new(regex_str, &bulder_binding);

    // Do something with the parsed AST (Abstract Syntax Tree)
    println!("Regex: {}", regex_str);
    let str1 = "aab";
    let str2 = "";
    println!("For {}: {}", str1, engine.compute(str1.to_string()));
    println!("For {}: {}", str2, engine.compute(str2.to_string()));

}
