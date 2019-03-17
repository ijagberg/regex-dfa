// use super::automaton::Automaton;

// pub fn from_tree(parse_tree: &ParseTree) -> Automaton {
//     match parse_tree {
//         ParseTree::Concatenation { left, right } => {
//             let left_dfa = from_tree(left);
//             let right_dfa = from_tree(right);
//             build_concatenation(left_dfa, right_dfa)
//         }
//         ParseTree::Or { left, right } => {
//             let left_dfa = from_tree(left);
//             let right_dfa = from_tree(right);
//             build_or(left_dfa, right_dfa)
//         }
//         ParseTree::Star { inner } => {
//             let inner_dfa = from_tree(inner);
//             build_star(inner_dfa)
//         }
//         ParseTree::Question { inner } => {
//             let inner_dfa = from_tree(inner);
//             build_question(inner_dfa)
//         }
//         ParseTree::Plus { inner } => {
//             let inner_dfa = from_tree(inner);
//             build_plus(inner_dfa)
//         }
//         ParseTree::Atom(c) => build_atom(*c),
//         ParseTree::Empty => build_empty(),
//     }
// }

// fn build_concatenation(left_dfa: Automaton, right_dfa: Automaton) -> Automaton {
//     assert_eq!(1, left_dfa.accepting_states.len());
//     assert_eq!(1, right_dfa.accepting_states.len());

//     let left_start_state = left_dfa.start_state.unwrap();
//     let left_end_state = *left_dfa.accepting_states.iter().next().unwrap();
//     let right_start_state = right_dfa.start_state.unwrap();
//     let right_end_state = *right_dfa.accepting_states.iter().next().unwrap();

//     let mut concatenation_dfa = Automaton::new();
//     concatenation_dfa.add_states_and_transitions(left_dfa);
//     let left_right_offset = concatenation_dfa.states;
//     concatenation_dfa.add_states_and_transitions(right_dfa);

//     // Add transition between left and right dfa
//     concatenation_dfa.add_transition(left_end_state, right_start_state + left_right_offset, None);

//     // Set start and end states
//     concatenation_dfa.set_start_state(left_start_state);
//     concatenation_dfa.clear_accepting();
//     concatenation_dfa.set_accepting(right_end_state + left_right_offset, true);

//     concatenation_dfa
// }

// fn build_or(left_dfa: Automaton, right_dfa: Automaton) -> Automaton {
//     assert_eq!(1, left_dfa.accepting_states.len());
//     assert_eq!(1, right_dfa.accepting_states.len());

//     let left_start_state = left_dfa.start_state.unwrap();
//     let left_end_state = *left_dfa.accepting_states.iter().next().unwrap();
//     let right_start_state = right_dfa.start_state.unwrap();
//     let right_end_state = *right_dfa.accepting_states.iter().next().unwrap();

//     let mut or_dfa = Automaton::new();
//     let or_start_state = or_dfa.add_state();
//     let or_end_state = or_dfa.add_state();

//     // Add states and transitions from left_dfa
//     let left_offset = or_dfa.states;
//     or_dfa.add_states_and_transitions(left_dfa);
//     let right_offset = or_dfa.states;
//     or_dfa.add_states_and_transitions(right_dfa);

//     // Add transitions from or_dfa to left_dfa
//     or_dfa.add_transition(or_start_state, left_start_state + left_offset, None);
//     or_dfa.add_transition(left_end_state + left_offset, or_end_state, None);

//     // Add transitions from or_dfa to right_dfa
//     or_dfa.add_transition(or_start_state, right_start_state + right_offset, None);
//     or_dfa.add_transition(right_end_state + right_offset, or_end_state, None);

//     // Set start and end states
//     or_dfa.set_start_state(or_start_state);
//     or_dfa.clear_accepting();
//     or_dfa.set_accepting(or_end_state, true);

//     or_dfa
// }

// fn build_star(inner_dfa: Automaton) -> Automaton {
//     assert_eq!(1, inner_dfa.accepting_states.len());
//     let inner_start_state = inner_dfa.start_state.unwrap();
//     let inner_end_state = *inner_dfa.accepting_states.iter().next().unwrap();

//     let mut star_dfa = Automaton::new();
//     let inner_offset = star_dfa.states;
//     star_dfa.add_states_and_transitions(inner_dfa);

//     // Add transitions from star to itself
//     star_dfa.add_transition(
//         inner_start_state + inner_offset,
//         inner_end_state + inner_offset,
//         None,
//     );
//     star_dfa.add_transition(
//         inner_end_state + inner_offset,
//         inner_start_state + inner_offset,
//         None,
//     );

//     // Set start and end states
//     star_dfa.set_start_state(inner_start_state);
//     star_dfa.clear_accepting();
//     star_dfa.set_accepting(inner_end_state, true);

//     star_dfa
// }

// fn build_question(inner_dfa: Automaton) -> Automaton {
//     assert_eq!(1, inner_dfa.accepting_states.len());
//     let inner_start_state = inner_dfa.start_state.unwrap();
//     let inner_end_state = *inner_dfa.accepting_states.iter().next().unwrap();

//     let mut question_dfa = Automaton::new();
//     let inner_offset = question_dfa.states;
//     question_dfa.add_states_and_transitions(inner_dfa);

//     question_dfa.add_transition(
//         inner_start_state + inner_offset,
//         inner_end_state + inner_offset,
//         None,
//     );
//     question_dfa.set_start_state(inner_start_state);
//     question_dfa.clear_accepting();
//     question_dfa.set_accepting(inner_end_state, true);

//     question_dfa
// }

// fn build_plus(inner_dfa: Automaton) -> Automaton {
//     assert_eq!(1, inner_dfa.accepting_states.len());
//     let inner_start_state = inner_dfa.start_state.unwrap();
//     let inner_end_state = *inner_dfa.accepting_states.iter().next().unwrap();

//     let mut plus_dfa = Automaton::new();
//     let inner_offset = plus_dfa.states;
//     plus_dfa.add_states_and_transitions(inner_dfa);

//     plus_dfa.add_transition(
//         inner_end_state + inner_offset,
//         inner_start_state + inner_offset,
//         None,
//     );
//     plus_dfa.set_start_state(inner_start_state);
//     plus_dfa.clear_accepting();
//     plus_dfa.set_accepting(inner_end_state, true);

//     plus_dfa
// }



// fn build_empty() -> Automaton {
//     let mut empty_dfa = Automaton::new();
//     let start_state = empty_dfa.add_state();
//     let end_state = empty_dfa.add_state();
//     empty_dfa.set_accepting(end_state, true);
//     empty_dfa.set_start_state(start_state);
//     empty_dfa.add_transition(start_state, end_state, None);
//     empty_dfa
// }
