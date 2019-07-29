use regex_dfa::automaton::Automaton;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let regex = &args[1];
    //println!("{}", regex);
    let minimized_dfa = Automaton::from_string(&regex).unwrap().into_min_dfa();
    println!("{}", minimized_dfa.to_dot_format());
}
