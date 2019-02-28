extern crate regex_dfa;

use regex_dfa::parse_tree::ParseTree;
use regex_dfa::automaton::Automaton;

fn main() {
    println!("{:#?}", ParseTree::from("a*b(cd|e)"));
    println!("{:#?}", Automaton{id: "tja".to_string()});
}