extern crate regex_dfa;

use regex_dfa::automaton::Automaton;
use regex_dfa::parse_tree::ParseTree;

fn main() {
    let or_tree = ParseTree::from("a|b");
    println!("or_tree: {:#?}", or_tree);
    let or_dfa = Automaton::from(&or_tree);
    println!("or_dfa: {:#?}", or_dfa);
}
