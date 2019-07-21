use regex_dfa::automaton::Automaton;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let regex = &args[1];
    println!("{}", regex);
    let nfa = Automaton::from_string(&regex).unwrap();
    let dfa = nfa.into_dfa();
    let minimized_dfa = dfa.into_min_dfa();
    println!("{}", minimized_dfa.to_dot_format());
}
