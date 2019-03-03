use super::parse_tree::ParseTree;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Automaton {
    states: u32,
    from_transitions: HashMap<u32, HashMap<u32, HashSet<Option<char>>>>,
    to_transitions: HashMap<u32, HashMap<u32, HashSet<Option<char>>>>,
    start_state: Option<u32>,
    accepting_states: HashSet<u32>,
    alphabet: HashSet<char>,
}

impl Automaton {
    pub fn new() -> Automaton {
        Automaton {
            states: 0,
            from_transitions: HashMap::new(),
            to_transitions: HashMap::new(),
            start_state: None,
            accepting_states: HashSet::new(),
            alphabet: HashSet::new(),
        }
    }

    // TODO: figure out a better way to use a set as a key
    fn get_unique_id_for_set(set: &HashSet<u32>) -> String {
        let mut set_as_vector = Vec::new();
        for i in set {
            set_as_vector.push(i);
        }
        set_as_vector.sort();
        let mut unique_id = "".to_string();
        for i in set_as_vector {
            unique_id = format!("{},{}", unique_id, i.to_string());
        }

        unique_id
    }

    fn get_composite_accepting(&self, composite: &HashSet<u32>) -> bool {
        for s in composite {
            if !self.accepting_states.contains(&s) {
                return false;
            }
        }
        true
    }

    pub fn as_dfa(&self) -> Automaton {
        match self.start_state {
            Some(start_state) => {
                println!("Alphabet: {:?}", self.alphabet);
                let mut minimized_dfa = Automaton::new();
                let mut comp_to_dfa = HashMap::new();

                let mut visited_comp = HashSet::new();
                let mut to_visit_comp = VecDeque::new();

                let comp_start_state = self.epsilon_closure(start_state);

                to_visit_comp.push_back(comp_start_state.clone());
                while let Some(from_comp) = to_visit_comp.pop_front() {
                    println!("Visiting {:?}", from_comp);
                    let from_comp_id = Automaton::get_unique_id_for_set(&from_comp);
                    visited_comp.insert(from_comp_id.clone());
                    let from_dfa_id = match comp_to_dfa.get(&from_comp_id) {
                        Some(dfa_id) => *dfa_id,
                        None => {
                            let dfa_id = minimized_dfa.add_state();
                            comp_to_dfa.insert(from_comp_id, dfa_id);
                            dfa_id
                        }
                    };
                    for c in &self.alphabet {
                        let to_comp = self.atom_closure(&from_comp, *c);
                        println!("Traversing via {} takes you to {:?}", *c, to_comp);
                        if to_comp.len() > 0 {
                            let to_comp_id = Automaton::get_unique_id_for_set(&to_comp);
                            if let Some(to_dfa_id) = comp_to_dfa.get(&to_comp_id) {
                                // Composite state is already in the minimized dfa
                                minimized_dfa.add_transition(from_dfa_id, *to_dfa_id, Some(*c));
                            } else {
                                // Composite state is not in the minimized dfa
                                let to_dfa_id = minimized_dfa.add_state();
                                println!(
                                    "Adding a new state for {:?} with id {}",
                                    to_comp, to_dfa_id
                                );
                                minimized_dfa.set_accepting(
                                    to_dfa_id,
                                    self.get_composite_accepting(&to_comp),
                                );
                                to_visit_comp.push_back(to_comp);
                                comp_to_dfa.insert(to_comp_id, to_dfa_id);
                                minimized_dfa.add_transition(from_dfa_id, to_dfa_id, Some(*c));
                            }
                        }
                    }
                }

                minimized_dfa.set_start_state(
                    *comp_to_dfa
                        .get(&Automaton::get_unique_id_for_set(&comp_start_state))
                        .unwrap(),
                );

                minimized_dfa
            }
            None => {
                panic!("No starting state");
            }
        }
    }

    /// Returns the set of states that can be reached from a given starting state
    /// without reading any input (only traversing epsilon-transitions)
    ///
    /// # Arguments
    ///
    /// * `start_state` - The state from which traversal begins
    fn epsilon_closure(&self, start_state: u32) -> HashSet<u32> {
        let mut reachable_states = HashSet::new();
        let mut unvisited_states = VecDeque::new();
        unvisited_states.push_back(start_state);
        while let Some(unvisited_state) = unvisited_states.pop_front() {
            reachable_states.insert(unvisited_state);
            if let Some(from_transitions) = self.from_transitions.get(&unvisited_state) {
                for (to_state, atoms_set) in from_transitions {
                    if atoms_set.contains(&None) && !unvisited_states.contains(to_state) {
                        unvisited_states.push_back(*to_state);
                    }
                }
            }
        }

        reachable_states
    }

    fn epsilon_closures(&self, from_state_set: HashSet<u32>) -> HashSet<u32> {
        let mut epsilon_closures = HashSet::new();
        for s in 0..self.states {
            epsilon_closures = epsilon_closures
                .union(&self.epsilon_closure(s))
                .cloned()
                .collect();
        }

        epsilon_closures
    }

    /// Returns the set of states that can be reached from a given composite state
    /// by reading one given atom
    ///
    /// # Arguments
    ///
    /// * `from_state_set` - The composite state from which traversal begins
    /// * `atom` - The atom via which we are allowed to traverse
    fn atom_closure(&self, from_state_set: &HashSet<u32>, atom: char) -> HashSet<u32> {
        let mut atom_closure = HashSet::new();
        for from_state in from_state_set {
            if let Some(from_transitions) = self.from_transitions.get(&from_state) {
                for (to_state, atoms_set) in from_transitions {
                    if atoms_set.contains(&Some(atom)) {
                        atom_closure = atom_closure
                            .union(&self.epsilon_closure(*to_state))
                            .cloned()
                            .collect();
                    }
                }
            }
        }

        atom_closure
    }

    fn add_states_and_transitions(&mut self, other_dfa: Automaton) {
        let states_offset = self.states;

        // Add states and transitions to concatenated dfa
        self.add_states(other_dfa.states);
        for from_transition in other_dfa.from_transitions {
            let from_state = from_transition.0;
            for to_states in from_transition.1 {
                let to_state = to_states.0;
                for atom in to_states.1 {
                    self.add_transition(from_state + states_offset, to_state + states_offset, atom);
                }
            }
        }
    }

    fn add_transition(&mut self, from_state: u32, to_state: u32, atom: Option<char>) {
        if let Some(c) = atom {
            self.alphabet.insert(c);
        }
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

    fn add_states(&mut self, states: u32) {
        self.states += states;
    }

    fn clear_accepting(&mut self) {
        self.accepting_states.clear();
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
        match parse_tree {
            ParseTree::Concatenation { left, right } => {
                let left_dfa = Automaton::from_tree(left);
                let right_dfa = Automaton::from_tree(right);
                Automaton::build_concatenation(left_dfa, right_dfa)
            }
            ParseTree::Or { left, right } => {
                let left_dfa = Automaton::from_tree(left);
                let right_dfa = Automaton::from_tree(right);
                Automaton::build_or(left_dfa, right_dfa)
            }
            ParseTree::Star { inner } => {
                let inner_dfa = Automaton::from_tree(inner);
                Automaton::build_star(inner_dfa)
            }
            ParseTree::Question { inner } => {
                let inner_dfa = Automaton::from_tree(inner);
                Automaton::build_question(inner_dfa)
            }
            ParseTree::Plus { inner } => {
                let inner_dfa = Automaton::from_tree(inner);
                Automaton::build_plus(inner_dfa)
            }
            ParseTree::Atom(c) => Automaton::build_atom(*c),
            ParseTree::Empty => Automaton::build_empty(),
        }
    }

    fn build_concatenation(left_dfa: Automaton, right_dfa: Automaton) -> Automaton {
        assert_eq!(1, left_dfa.accepting_states.len());
        assert_eq!(1, right_dfa.accepting_states.len());

        let left_start_state = left_dfa.start_state.unwrap();
        let left_end_state = *left_dfa.accepting_states.iter().next().unwrap();
        let right_start_state = right_dfa.start_state.unwrap();
        let right_end_state = *right_dfa.accepting_states.iter().next().unwrap();

        let mut concatenation_dfa = Automaton::new();
        concatenation_dfa.add_states_and_transitions(left_dfa);
        let left_right_offset = concatenation_dfa.states;
        concatenation_dfa.add_states_and_transitions(right_dfa);

        // Add transition between left and right dfa
        concatenation_dfa.add_transition(
            left_end_state,
            right_start_state + left_right_offset,
            None,
        );

        // Set start and end states
        concatenation_dfa.set_start_state(left_start_state);
        concatenation_dfa.clear_accepting();
        concatenation_dfa.set_accepting(right_end_state + left_right_offset, true);

        concatenation_dfa
    }

    fn build_or(left_dfa: Automaton, right_dfa: Automaton) -> Automaton {
        assert_eq!(1, left_dfa.accepting_states.len());
        assert_eq!(1, right_dfa.accepting_states.len());

        let left_start_state = left_dfa.start_state.unwrap();
        let left_end_state = *left_dfa.accepting_states.iter().next().unwrap();
        let right_start_state = right_dfa.start_state.unwrap();
        let right_end_state = *right_dfa.accepting_states.iter().next().unwrap();

        let mut or_dfa = Automaton::new();
        let or_start_state = or_dfa.add_state();
        let or_end_state = or_dfa.add_state();

        // Add states and transitions from left_dfa
        let left_offset = or_dfa.states;
        or_dfa.add_states_and_transitions(left_dfa);
        let right_offset = or_dfa.states;
        or_dfa.add_states_and_transitions(right_dfa);

        // Add transitions from or_dfa to left_dfa
        or_dfa.add_transition(or_start_state, left_start_state + left_offset, None);
        or_dfa.add_transition(left_end_state + left_offset, or_end_state, None);

        // Add transitions from or_dfa to right_dfa
        or_dfa.add_transition(or_start_state, right_start_state + right_offset, None);
        or_dfa.add_transition(right_end_state + right_offset, or_end_state, None);

        // Set start and end states
        or_dfa.set_start_state(or_start_state);
        or_dfa.clear_accepting();
        or_dfa.set_accepting(or_end_state, true);

        or_dfa
    }

    fn build_star(inner_dfa: Automaton) -> Automaton {
        assert_eq!(1, inner_dfa.accepting_states.len());
        let inner_start_state = inner_dfa.start_state.unwrap();
        let inner_end_state = *inner_dfa.accepting_states.iter().next().unwrap();

        let mut star_dfa = Automaton::new();
        let inner_offset = star_dfa.states;
        star_dfa.add_states_and_transitions(inner_dfa);

        // Add transitions from star to itself
        star_dfa.add_transition(
            inner_start_state + inner_offset,
            inner_end_state + inner_offset,
            None,
        );
        star_dfa.add_transition(
            inner_end_state + inner_offset,
            inner_start_state + inner_offset,
            None,
        );

        // Set start and end states
        star_dfa.set_start_state(inner_start_state);
        star_dfa.clear_accepting();
        star_dfa.set_accepting(inner_end_state, true);

        star_dfa
    }

    fn build_question(inner_dfa: Automaton) -> Automaton {
        assert_eq!(1, inner_dfa.accepting_states.len());
        let inner_start_state = inner_dfa.start_state.unwrap();
        let inner_end_state = *inner_dfa.accepting_states.iter().next().unwrap();

        let mut question_dfa = Automaton::new();
        let inner_offset = question_dfa.states;
        question_dfa.add_states_and_transitions(inner_dfa);

        question_dfa.add_transition(
            inner_start_state + inner_offset,
            inner_end_state + inner_offset,
            None,
        );
        question_dfa.set_start_state(inner_start_state);
        question_dfa.clear_accepting();
        question_dfa.set_accepting(inner_end_state, true);

        question_dfa
    }

    fn build_plus(inner_dfa: Automaton) -> Automaton {
        assert_eq!(1, inner_dfa.accepting_states.len());
        let inner_start_state = inner_dfa.start_state.unwrap();
        let inner_end_state = *inner_dfa.accepting_states.iter().next().unwrap();

        let mut plus_dfa = Automaton::new();
        let inner_offset = plus_dfa.states;
        plus_dfa.add_states_and_transitions(inner_dfa);

        plus_dfa.add_transition(
            inner_end_state + inner_offset,
            inner_start_state + inner_offset,
            None,
        );
        plus_dfa.set_start_state(inner_start_state);
        plus_dfa.clear_accepting();
        plus_dfa.set_accepting(inner_end_state, true);

        plus_dfa
    }

    fn build_atom(c: char) -> Automaton {
        let mut atom_dfa = Automaton::new();
        let start_state = atom_dfa.add_state();
        let end_state = atom_dfa.add_state();
        atom_dfa.set_accepting(end_state, true);
        atom_dfa.set_start_state(start_state);
        atom_dfa.add_transition(start_state, end_state, Some(c));
        atom_dfa
    }

    fn build_empty() -> Automaton {
        let mut empty_dfa = Automaton::new();
        let start_state = empty_dfa.add_state();
        let end_state = empty_dfa.add_state();
        empty_dfa.set_accepting(end_state, true);
        empty_dfa.set_start_state(start_state);
        empty_dfa.add_transition(start_state, end_state, None);
        empty_dfa
    }
}
