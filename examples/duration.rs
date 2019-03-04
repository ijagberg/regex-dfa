extern crate regex_dfa;

use regex_dfa::automaton::Automaton;
use regex_dfa::parse_tree::ParseTree;
use regex_dfa::plot::*;
use std::time::Instant;

fn main() {
    let timer = Instant::now();
    let dfa = Automaton::from(&ParseTree::from("(a|b)*abb"));
    println!("Created minimized dfa in {:?}", timer.elapsed());
    automaton_pretty_print(&dfa);
}
