extern crate regex_dfa;

use regex_dfa::automaton::Automaton;
use regex_dfa::parse_tree::ParseTree;
use regex_dfa::plot::*;

fn main() {
    let automaton = Automaton::from(&ParseTree::from("(a|b)*abb"));
    automaton_pretty_print(&automaton);
    automaton_pretty_print(&automaton.as_dfa());
}
