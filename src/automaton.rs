use super::construct_automaton::*;
use super::parse_tree::ParseTree;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Automaton {
    pub states: u32,
    pub from_transitions: HashMap<u32, HashMap<u32, HashSet<Option<char>>>>,
    pub to_transitions: HashMap<u32, HashMap<u32, HashSet<Option<char>>>>,
    pub start_state: Option<u32>,
    pub accepting_states: HashSet<u32>,
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

    /// Returns true if every state in the given composite state is accepting
    ///
    /// # Arguments
    ///
    /// * `composite` - The composite state set to check
    fn get_composite_accepting(&self, composite: &HashSet<u32>) -> bool {
        for s in composite {
            if self.accepting_states.contains(&s) {
                return true;
            }
        }
        false
    }

    /// Returns a dfa simulating the same functionality of this automaton
    pub fn as_dfa(&self) -> Automaton {
        match self.start_state {
            Some(start_state) => {
                // println!("Alphabet: {:?}", self.alphabet);
                let mut minimized_dfa = Automaton::new();
                let mut comp_to_dfa = HashMap::new();

                let mut visited_comp = HashSet::new();
                let mut to_visit_comp = VecDeque::new();

                let comp_start_state = self.epsilon_closure(start_state);

                to_visit_comp.push_back(comp_start_state.clone());
                // println!("Starting at composite: {:?}", comp_start_state);
                while let Some(from_comp) = to_visit_comp.pop_front() {
                    // println!("Visiting {:?}", from_comp);
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
                        // println!("Traversing via {} takes you to {:?}", *c, to_comp);
                        if to_comp.len() > 0 {
                            let to_comp_id = Automaton::get_unique_id_for_set(&to_comp);
                            if let Some(to_dfa_id) = comp_to_dfa.get(&to_comp_id) {
                                // Composite state is already in the minimized dfa
                                minimized_dfa.add_transition(from_dfa_id, *to_dfa_id, Some(*c));
                            } else {
                                // Composite state is not in the minimized dfa
                                let to_dfa_id = minimized_dfa.add_state();
                                // println!(
                                //     "Adding a new state for {:?} with id {}",
                                //     to_comp, to_dfa_id
                                // );
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

    pub fn as_minimized_dfa(&self) {
        let x = self.get_marked_states_table();

        for s1 in 0..self.states {
            for s2 in 0..s1 {
                if !x[s1 as usize][s2 as usize] {
                    // (s1, s2) is unmarked, find its equivalences

                }
            }
        }
    }

    fn get_marked_states_table(&self) -> Vec<Vec<bool>> {
        let mut marked_states_table: Vec<Vec<bool>> =
            vec![vec![false; self.states as usize]; self.states as usize];
        for non_accepting_state in 0..self.states {
            if !self.accepting_states.contains(&non_accepting_state) {
                for accepting_state in &self.accepting_states {
                    marked_states_table[non_accepting_state as usize]
                        [(*accepting_state) as usize] = true;
                }
            }
        }
        let mut marked_a_pair = true;
        while marked_a_pair {
            marked_a_pair = false;

            // Choose a pair of states
            'mark: for s1 in 0..self.states {
                for s2 in 0..s1 {
                    if !marked_states_table[s1 as usize][s2 as usize] {
                        // Check if there is any transition from (s1, s2) to a marked pair
                        for c in &self.alphabet {
                            if let Some(s1_to_state) = self.traverse_from(&s1, c) {
                                if let Some(s2_to_state) = self.traverse_from(&s2, c) {
                                    if marked_states_table[s1_to_state as usize]
                                        [s2_to_state as usize]
                                    {
                                        marked_states_table[s1 as usize][s2 as usize] = true;
                                        marked_a_pair = true;
                                        break 'mark;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        marked_states_table
    }

    pub fn traverse_from(&self, from_state: &u32, atom: &char) -> Option<u32> {
        if let Some(transitions) = self.from_transitions.get(from_state) {
            for (to_state, atoms_set) in transitions {
                if atoms_set.contains(&Some(*atom)) {
                    return Some(*to_state);
                }
            }
        }
        None
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

    pub fn add_states_and_transitions(&mut self, other_dfa: Automaton) {
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

    pub fn add_transition(&mut self, from_state: u32, to_state: u32, atom: Option<char>) {
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

    pub fn add_state(&mut self) -> u32 {
        let states_before_adding = self.states;
        self.states += 1;
        states_before_adding
    }

    fn add_states(&mut self, states: u32) {
        self.states += states;
    }

    pub fn clear_accepting(&mut self) {
        self.accepting_states.clear();
    }

    pub fn set_accepting(&mut self, state: u32, accepting: bool) {
        if state < self.states {
            if accepting {
                self.accepting_states.insert(state);
            } else {
                self.accepting_states.remove(&state);
            }
        }
    }

    pub fn set_start_state(&mut self, state: u32) {
        if state < self.states {
            self.start_state = Some(state);
        }
    }

    pub fn from(parse_tree: &ParseTree) -> Automaton {
        from_tree(&parse_tree)
    }
}
