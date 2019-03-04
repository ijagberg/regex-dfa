extern crate regex_dfa;

use regex_dfa::automaton::Automaton;
use regex_dfa::parse_tree::ParseTree;
use regex_dfa::plot::*;
use std::time::Instant;

fn main() {
    let regex = "(a|b)*abb";
    let timer = Instant::now();
    let dfa = Automaton::from(&ParseTree::from(regex));
    println!("Created minimized dfa for regex {:?} in {:?}", regex, timer.elapsed());
    automaton_pretty_print(&dfa);
    let test_string_1 = "aaaabb";
    let test_string_2 = "aabab";
    let timer = Instant::now();
    println!("Input string {:?}: {} in {:?}", test_string_1, dfa.match_whole(test_string_1), timer.elapsed());
    let timer = Instant::now();
    println!("Input string {:?}: {} in {:?}", test_string_2, dfa.match_whole(test_string_2), timer.elapsed());
}
