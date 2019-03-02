extern crate regex_dfa;

use regex_dfa::automaton::Automaton;
use regex_dfa::parse_tree::ParseTree;

fn main() {
    let atom_tree = ParseTree::from("a");
    println!("atom_tree: {:#?}", atom_tree);
    let atom_dfa = Automaton::from(&atom_tree);
    println!("atom_dfa: {:#?}", atom_dfa);
}
