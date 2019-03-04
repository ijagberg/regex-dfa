extern crate regex_dfa;

use regex_dfa::automaton::Automaton;
use regex_dfa::plot::*;
use std::time::Instant;

fn main() {
    let regex_list = vec!["(a|b)*abb", "a*b*c*d", "(a|b)*|(c|d)"];
    for regex in regex_list {
        construct_dfa(regex);
    }
}

fn construct_dfa(s: &str) {
    let timer = Instant::now();
    let dfa = Automaton::from(s);
    println!("Created minimized dfa for {:?} in {:?}", s, timer.elapsed());
    automaton_pretty_print(&dfa);
}
