extern crate regex_dfa;

use regex_dfa::automaton::Automaton;
use regex_dfa::parse_tree::ParseTree;
use regex_dfa::plot::*;

fn main() {
    let automaton = Automaton::from(&ParseTree::from("(a|b)*abb"));
    automaton_pretty_print(&automaton);
    let dfa = automaton.as_dfa();
    automaton_pretty_print(&dfa);
    let marked_states = dfa.get_marked_states_table();
    println!("{:#?}", marked_states);
}
