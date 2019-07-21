use regex_dfa::automaton::StateMachine;

fn main() {
    let args : Vec<_>= std::env::args().collect();
    let regex = &args[1];
    println!("{}", regex);
    let nfa = StateMachine::from_string(&regex).unwrap();
    let dfa = nfa.as_dfa();
    let minimized_dfa = dfa.as_minimized_dfa();
    println!("{}", minimized_dfa.to_dot_format());
}
