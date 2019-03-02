extern crate regex_dfa;

use regex_dfa::automaton::Automaton;
use regex_dfa::parse_tree::ParseTree;

fn main() {
    let star_tree = ParseTree::from("a*");
    println!("star_tree: {:#?}", star_tree);
    let star_dfa = Automaton::from(&star_tree);
    println!("star_dfa: {:#?}", star_dfa);
}
