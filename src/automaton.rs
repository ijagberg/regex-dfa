use super::construct_automaton::*;
use super::parse_tree::IntoParseTree;
use super::plot;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Mul;

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

    pub fn match_whole(&self, input: &str) -> bool {
        let mut current_state = self.start_state.expect("No start state set for dfa");
        for current_atom in input.chars() {
            match self.traverse_from(&current_state, &current_atom) {
                Some(next_state) => current_state = next_state,
                None => return false,
            }
        }
        self.accepting_states.contains(&current_state)
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
    fn as_dfa(&self) -> Automaton {
        match self.start_state {
            Some(start_state) => {
                let mut minimized_dfa = Automaton::new();
                let mut comp_to_dfa = HashMap::new();

                let mut visited_comp = HashSet::new();
                let mut to_visit_comp = VecDeque::new();

                let comp_start_state = self.epsilon_closure(start_state);

                to_visit_comp.push_back(comp_start_state.clone());
                while let Some(from_comp) = to_visit_comp.pop_front() {
                    let from_comp_id = get_unique_id_for_set(&from_comp);
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
                        if to_comp.len() > 0 {
                            let to_comp_id = get_unique_id_for_set(&to_comp);
                            if let Some(to_dfa_id) = comp_to_dfa.get(&to_comp_id) {
                                // Composite state is already in the minimized dfa
                                minimized_dfa.add_transition(from_dfa_id, *to_dfa_id, Some(*c));
                            } else {
                                // Composite state is not in the minimized dfa
                                let to_dfa_id = minimized_dfa.add_state();
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
                        .get(&get_unique_id_for_set(&comp_start_state))
                        .unwrap(),
                );

                minimized_dfa
            }
            None => {
                panic!("No starting state");
            }
        }
    }

    fn as_minimized_dfa(&self) -> Automaton {
        let equivalent_states = dbg!(self.get_equivalent_states());
        let mut comp_state_to_dfa = HashMap::new();
        let mut min_dfa = Automaton::new();

        // First add all states to minimized dfa
        for (s1, equivalent_to_s1) in &equivalent_states {
            if let Some(dfa_state_id) = comp_state_to_dfa.get(s1) {
                // s1 has already been added to min_dfa
                comp_state_to_dfa.insert(s1, *dfa_state_id);
            } else {
                let dfa_state_id = min_dfa.add_state();
                for equivalent_state in equivalent_to_s1 {
                    comp_state_to_dfa.insert(equivalent_state, dfa_state_id);
                }

                if let Some(comp_start_state) = self.start_state {
                    if equivalent_to_s1.contains(&comp_start_state) {
                        min_dfa.set_start_state(dfa_state_id);
                    }
                }

                // Set accepting if all states in comp is accepting
                min_dfa.set_accepting(
                    dfa_state_id,
                    equivalent_to_s1
                        .iter()
                        .all(|&state| self.accepting_states.contains(&state)),
                );
            }
        }

        // Then add all transitions
        for (from_state, _) in &equivalent_states {
            for c in &self.alphabet {
                if let Some(to_state) = self.traverse_from(&from_state, c) {
                    if let (Some(dfa_from_state), Some(dfa_to_state)) = (
                        comp_state_to_dfa.get(&from_state),
                        comp_state_to_dfa.get(&to_state),
                    ) {
                        min_dfa.add_transition(*dfa_from_state, *dfa_to_state, Some(*c));
                    }
                }
            }
        }

        min_dfa
    }

    fn get_equivalent_states(&self) -> HashMap<u32, HashSet<u32>> {
        let marked_states = dbg!(self.get_marked_states_table());
        let mut equivalent_composite_states = HashMap::new();
        for s1 in 0..self.states {
            let mut equivalent_to_s1 = HashSet::new();
            for s2 in 0..self.states {
                if !marked_states[s1 as usize][s2 as usize] {
                    equivalent_to_s1.insert(s2);
                }
            }
            equivalent_composite_states.insert(s1, equivalent_to_s1);
        }
        equivalent_composite_states
    }

    fn get_marked_states_table(&self) -> Vec<Vec<bool>> {
        let mut marked_states_table: Vec<Vec<bool>> =
            vec![vec![false; self.states as usize]; self.states as usize];
        for non_accepting_state in 0..self.states {
            if !self.accepting_states.contains(&non_accepting_state) {
                for accepting_state in &self.accepting_states {
                    marked_states_table[non_accepting_state as usize]
                        [(*accepting_state) as usize] = true;
                    marked_states_table[(*accepting_state) as usize]
                        [non_accepting_state as usize] = true;
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
                            match (self.traverse_from(&s1, c), self.traverse_from(&s2, c)) {
                                (None, Some(s2_to_state)) => {
                                    // There is no transition for s1, there is a transition for s2
                                    if self.accepting_states.contains(&s2_to_state) {
                                        marked_states_table[s1 as usize][s2 as usize] = true;
                                        marked_states_table[s2 as usize][s1 as usize] = true;
                                        marked_a_pair = true;
                                        break 'mark;
                                    }
                                }
                                (Some(s1_to_state), None) => {
                                    // There is a transition for s1, there is no transition for s2
                                    if self.accepting_states.contains(&s1_to_state) {
                                        marked_states_table[s1 as usize][s2 as usize] = true;
                                        marked_states_table[s2 as usize][s1 as usize] = true;
                                        marked_a_pair = true;
                                        break 'mark;
                                    }
                                }
                                (Some(s1_to_state), Some(s2_to_state)) => {
                                    if marked_states_table[s1_to_state as usize]
                                        [s2_to_state as usize]
                                    {
                                        marked_states_table[s1 as usize][s2 as usize] = true;
                                        marked_states_table[s2 as usize][s1 as usize] = true;
                                        marked_a_pair = true;
                                        break 'mark;
                                    }
                                }
                                (None, None) => {}
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
                    if atoms_set.contains(&None) && !reachable_states.contains(to_state) {
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

    pub fn from<T>(into_parse_tree: T) -> Automaton
    where
        T: IntoParseTree,
    {
        from_tree(&into_parse_tree.into_parse_tree()).as_dfa()
    }
}

impl Mul for Automaton {
    type Output = Automaton;

    fn mul(self, other: Automaton) -> Automaton {
        // For each pair of states,
        let mut mul_dfa = Automaton::new();
        let mul_alphabet: HashSet<char> = dbg!(self.alphabet.union(&other.alphabet).cloned().collect());
        let mut pair_to_dfa: HashMap<(u32, u32), u32> = HashMap::new();
        for (self_from_state, other_from_state) in (0..self.states).zip(0..other.states) {
            let mul_from_state = match pair_to_dfa.get(&(self_from_state, other_from_state)) {
                Some(mul_state) => mul_state.clone(),
                None => {
                    let mul_from_state = mul_dfa.add_state();
                    pair_to_dfa.insert((self_from_state, other_from_state), mul_from_state);
                    mul_dfa.set_accepting(
                        mul_from_state,
                        self.accepting_states.contains(&self_from_state)
                            && other.accepting_states.contains(&other_from_state),
                    );
                    mul_from_state
                }
            };

            for atom in &mul_alphabet {
                if let (Some(self_to_state), Some(other_to_state)) = (
                    self.traverse_from(&self_from_state, &atom),
                    other.traverse_from(&other_from_state, &atom),
                ) {
                    // There is a transition from from_state to a to_state via atom for both self and other
                    let mul_to_state = match pair_to_dfa.get(&(self_to_state, other_to_state)) {
                        Some(mul_state) => mul_state.clone(),
                        None => {
                            let mul_to_state = mul_dfa.add_state();
                            pair_to_dfa.insert((self_to_state, other_to_state), mul_to_state);
                            mul_dfa.set_accepting(
                                mul_to_state,
                                self.accepting_states.contains(&self_to_state)
                                    && other.accepting_states.contains(&other_to_state),
                            );
                            mul_to_state
                        }
                    };

                    // Add transition from mul_from_state to mul_to_state
                    mul_dfa.add_transition(mul_from_state, mul_to_state, Some(*atom));
                }
            }

            // Set start state in mul_dfa
            if let Some(mul_start_state) =
                pair_to_dfa.get(&(self.start_state.unwrap(), other.start_state.unwrap()))
            {
                dbg!(mul_dfa.set_start_state(dbg!(*mul_start_state)));
            }
        }
        mul_dfa.as_minimized_dfa()
    }
}

// TODO: figure out a better way to use a set as a key
fn get_unique_id_for_set(set: &HashSet<u32>) -> String {
    let mut set_as_vector: Vec<u32> = set.iter().cloned().collect();
    set_as_vector.sort();
    format!("{:?}", set_as_vector)
}

#[test]
fn test_automaton_mul() {
    let automaton_a = Automaton::from("(aa)|(aaa*b)");
    // plot::automaton_pretty_print(&automaton_a);
    let automaton_a_min = automaton_a.as_minimized_dfa();
    // plot::automaton_pretty_print(&automaton_a_min);
    assert!(automaton_a.match_whole("aa"));
    assert!(automaton_a.match_whole("aaaaaab"));
    assert!(!automaton_a.match_whole("b"));
    assert!(!automaton_a.match_whole("baa"));
    assert!(!automaton_a.match_whole("aaabb"));

    assert!(automaton_a_min.match_whole("aa"));
    assert!(automaton_a_min.match_whole("aaaaaab"));
    assert!(!automaton_a_min.match_whole("b"));
    assert!(!automaton_a_min.match_whole("baa"));
    assert!(!automaton_a_min.match_whole("aaabb"));

    let automaton_b = Automaton::from("a*bb*");
    // plot::automaton_pretty_print(&automaton_b);
    let automaton_b_min = automaton_b.as_minimized_dfa();
    // plot::automaton_pretty_print(&automaton_b_min);
    assert!(!automaton_b.match_whole("aa"));
    assert!(automaton_b.match_whole("aaaaaab"));
    assert!(automaton_b.match_whole("aaabb"));

    assert!(!automaton_b_min.match_whole("aa"));
    assert!(automaton_b_min.match_whole("aaaaaab"));
    assert!(automaton_b_min.match_whole("aaabb"));

    let automaton_c = automaton_a * automaton_b;
    plot::automaton_pretty_print(&automaton_c);
    assert!(automaton_c.match_whole("aab"));
}
