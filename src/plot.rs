use super::automaton::Automaton;

pub fn automaton_pretty_print(automaton: &Automaton) {
    println!("Starting state: {:?}", automaton.start_state);
    for from_state in 0..automaton.states {
        if let Some(from_transitions) = automaton.from_transitions.get(&from_state) {
            let from_state_str = if automaton.accepting_states.contains(&from_state) {
                format!("_{}_", from_state)
            } else {
                format!("{}", from_state)
            };
            for (to_state, atoms) in from_transitions {
                let to_state_str = if automaton.accepting_states.contains(to_state) {
                    format!("_{}_", to_state)
                } else {
                    format!("{}", to_state)
                };
                for atom in atoms {
                    match atom {
                        Some(c) => println!("{} -{}-> {}", from_state_str, c, to_state_str),
                        None => println!("{} -€-> {}", from_state_str, to_state_str),
                    }
                }
            }
        }
    }
}
