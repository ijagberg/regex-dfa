use super::parse_tree::ParseTree;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Automaton {
    states: u32,
    from_transitions: HashMap<u32, HashMap<u32, HashSet<Option<char>>>>,
    to_transitions: HashMap<u32, HashMap<u32, HashSet<Option<char>>>>,
    start_state: Option<u32>,
    accepting_states: HashSet<u32>,
}

impl Automaton {
    fn new() -> Automaton {
        Automaton {
            states: 0,
            from_transitions: HashMap::new(),
            to_transitions: HashMap::new(),
            start_state: None,
            accepting_states: HashSet::new(),
        }
    }

    fn add_transition(&mut self, from_state: u32, to_state: u32, atom: Option<char>) {
        self.add_from_transition(from_state, to_state, atom);
        self.add_to_transition(from_state, to_state, atom);
    }

    fn add_from_transition(&mut self, from_state: u32, to_state: u32, atom: Option<char>) {
        match self.from_transitions.get_mut(&from_state) {
            Some(to_states) => {
                // There is some transition from from_state to some other state
                if let Some(atoms) = to_states.get_mut(&to_state) {
                    // There is already some transition from from_state to to_state
                    atoms.insert(atom);
                } else {
                    // There is no transition from from_state to to_state
                    let mut atoms_set = HashSet::new();
                    atoms_set.insert(atom);
                    to_states.insert(to_state, atoms_set); // Create empty atoms set
                }
            }
            None => {
                // There is no transition from from_state to any other state
                let mut to_states = HashMap::new();
                let mut atoms_set = HashSet::new(); // atoms_set for transitions from from_state to to_state
                atoms_set.insert(atom);
                to_states.insert(to_state, atoms_set);
                self.from_transitions.insert(from_state, to_states);
            }
        }
    }

    fn add_to_transition(&mut self, from_state: u32, to_state: u32, atom: Option<char>) {
        match self.to_transitions.get_mut(&to_state) {
            Some(from_states) => {
                // There is some transition from some other state to to_state
                if let Some(atoms) = from_states.get_mut(&from_state) {
                    // There is already some transition from from_state to to_state
                    atoms.insert(atom);
                } else {
                    // There is no transition from from_state to to_state
                    let mut atoms_set = HashSet::new();
                    atoms_set.insert(atom);
                    from_states.insert(from_state, atoms_set); // Create empty atoms set
                }
            }
            None => {
                // There is no transition from any other state to to_state
                let mut from_states = HashMap::new();
                let mut atoms_set = HashSet::new(); // atoms_set for transitions from from_state to to_state
                atoms_set.insert(atom);
                from_states.insert(from_state, atoms_set);
                self.to_transitions.insert(to_state, from_states);
            }
        }
    }

    fn add_state(&mut self) -> u32 {
        let states_before_adding = self.states;
        self.states += 1;
        states_before_adding
    }

    fn set_accepting(&mut self, state: u32, accepting: bool) {
        if state < self.states {
            if accepting {
                self.accepting_states.insert(state);
            } else {
                self.accepting_states.remove(&state);
            }
        }
    }

    fn set_start_state(&mut self, state: u32) {
        if state < self.states {
            self.start_state = Some(state);
        }
    }

    pub fn from(parse_tree: &ParseTree) -> Automaton {
        Automaton::from_tree(&parse_tree)
    }

    fn from_tree(parse_tree: &ParseTree) -> Automaton {
        let mut dfa = Automaton::new();
        match parse_tree {
            ParseTree::Concatenation { left, right } => {
                let left_dfa = Automaton::from_tree(left);
                let right_dfa = Automaton::from_tree(right);
                let concatenation_dfa = Automaton::new();

                assert_eq!(1, left_dfa.accepting_states.len());
                assert_eq!(1, right_dfa.accepting_states.len());

                let left_dfa_end_state = left_dfa.accepting_states.iter().next().unwrap();

                concatenation_dfa
            }
            ParseTree::Or { left, right } => {
                let left_dfa = Automaton::from_tree(left);
                let right_dfa = Automaton::from_tree(right);
                let or_dfa = Automaton::new();

                or_dfa
            }
            ParseTree::Star { inner } => {
                let inner_dfa = Automaton::from_tree(inner);
                inner_dfa
            }
            ParseTree::Question { inner } => {
                let inner_dfa = Automaton::from_tree(inner);
                inner_dfa
            }
            ParseTree::Plus { inner } => {
                let inner_dfa = Automaton::from_tree(inner);
                inner_dfa
            }
            ParseTree::Atom(c) => {
                let mut atom_dfa = Automaton::new();
                let start_state = atom_dfa.add_state();
                let end_state = atom_dfa.add_state();
                atom_dfa.set_accepting(end_state, true);
                atom_dfa.set_start_state(start_state);
                atom_dfa.add_transition(start_state, end_state, Some(*c));
                atom_dfa
            }
            ParseTree::Empty => {
                let empty_dfa = Automaton::new();
                empty_dfa
            }
        }
    }
}
