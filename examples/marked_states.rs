extern crate regex_dfa;

use regex_dfa::automaton::Automaton;
use regex_dfa::parse_tree::ParseTree;

fn main() {
    let tree = Automaton::from(&ParseTree::from("(a|b)*abb"));
    println!("{:?}", tree);
    let dfa = tree.as_dfa();
    println!("{:?}", dfa);
}
