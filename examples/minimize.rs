extern crate regex_dfa;

use regex_dfa::automaton::Automaton;
use regex_dfa::parse_tree::ParseTree;

fn main() {
    let automaton = Automaton::from(&ParseTree::from("a|b")).as_dfa();
    println!("automaton: {:#?}", automaton);
    let marked_table = automaton.as_minimized_dfa();
    println!("marked_table: {:#?}", marked_table);
}
