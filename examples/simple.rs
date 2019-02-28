extern crate regex_dfa;

use regex_dfa::automaton::Automaton;
use regex_dfa::parse_tree::ParseTree;

fn main() {
    println!("{:#?}", ParseTree::from("a*b(cd|e)"));
}
