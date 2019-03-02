extern crate regex_dfa;

use regex_dfa::automaton::Automaton;
use regex_dfa::parse_tree::ParseTree;

fn main() {
    let left_dfa = Automaton::from(&ParseTree::from("a"));
    let right_dfa = Automaton::from(&ParseTree::from("b"));
    let concatenation_dfa = Automaton::from(&ParseTree::from("ab"));
    println!("concatenation_dfa: {:#?}", concatenation_dfa);
}
