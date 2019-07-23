use std::collections::{BTreeSet, HashMap, VecDeque};
use std::ops::Range;

#[derive(Debug)]
pub struct Automaton {
    pub states: u32,
    pub from_transitions: HashMap<u32, HashMap<u32, BTreeSet<Option<char>>>>,
    pub to_transitions: HashMap<u32, HashMap<u32, BTreeSet<Option<char>>>>,
    pub start_state: Option<u32>,
    pub accepting_states: BTreeSet<u32>,
    alphabet: BTreeSet<char>,
    kind: AutomatonKind,
}

#[derive(Debug)]
enum AutomatonKind {
    Nfa,
    Dfa,
    MinDfa,
}

impl Automaton {
    pub fn new() -> Self {
        Self {
            states: 0,
            from_transitions: HashMap::new(),
            to_transitions: HashMap::new(),
            start_state: None,
            accepting_states: BTreeSet::new(),
            alphabet: BTreeSet::new(),
            kind: AutomatonKind::Nfa,
        }
    }

    pub fn from_string(s: &str) -> Result<Automaton, Box<std::error::Error>> {
        let automaton = crate::translator::translate(s)?;
        Ok(automaton)
    }

    pub fn into_dfa(self) -> Automaton {
        match self.kind {
            AutomatonKind::Nfa => nfa_to_dfa(&self),
            _ => self,
        }
    }

    pub fn into_min_dfa(self) -> Automaton {
        match self.kind {
            AutomatonKind::Nfa => {
                let dfa = nfa_to_dfa(&self);
                dfa_to_minimized_dfa(&dfa)
            }
            AutomatonKind::Dfa => dfa_to_minimized_dfa(&self),
            _ => self,
        }
    }

    /// Traverses the dfa via the characters in `input` to determine if it matches the whole string
    ///
    /// # Arguments
    /// * `input` - The string to be matched
    ///
    /// # Return
    /// `true` if the dfa is in an accepting state after traversing through all of `input`.
    ///
    /// `false` if the dfa is in a non-accepting state after traversing through all of `input`, or if there is some point where there is no transition for the current atom being processed
    pub fn match_whole(&self, input: &str) -> bool {
        let mut current_state = self.start_state.expect("No start state set for dfa");
        if input.is_empty() {
            return self.accepting_states.contains(&current_state);
        }

        for current_atom in input.chars() {
            match self.traverse_from(current_state, current_atom) {
                Some(next_state) => current_state = next_state,
                None => return false,
            }
        }
        self.accepting_states.contains(&current_state)
    }

    /// Traverses the dfa via the characters in `input` to find the first prefix that is matched by the dfa
    pub fn match_first_prefix<'a>(&self, input: &'a str) -> Option<&'a str> {
        let mut current_state = self.start_state.expect("No start state set for dfa");
        for (index, current_atom) in input.chars().enumerate() {
            if self.accepting_states.contains(&current_state) {
                return Some(&input[0..index]);
            }
            match self.traverse_from(current_state, current_atom) {
                Some(next_state) => current_state = next_state,
                None => return None,
            }
        }
        Some(input)
    }

    fn match_all_prefixes(&self, input: &str) -> Vec<Range<usize>> {
        let mut matched_prefixes = Vec::new();

        let mut current_state = self.start_state.expect("No start state set for dfa");
        if self.accepting_states.contains(&current_state) {
            matched_prefixes.push(Range { start: 0, end: 0 });
        }
        for (index, current_atom) in input.chars().enumerate() {
            match self.traverse_from(current_state, current_atom) {
                Some(next_state) => current_state = next_state,
                None => return matched_prefixes,
            }
            if self.accepting_states.contains(&current_state) {
                matched_prefixes.push(0..index + 1);
            }
        }
        matched_prefixes
    }

    pub fn match_substrings(&self, input: &str) -> Vec<Range<usize>> {
        let mut matched_substrings = Vec::new();

        for (index, _) in input.chars().enumerate() {
            let matched_prefixes = self.match_all_prefixes(&input[index..]);
            for range in matched_prefixes {
                matched_substrings.push(range.start + index..range.end + index);
            }
        }
        matched_substrings
    }

    pub fn match_longest_prefix(&self, input: &str) -> Option<Range<usize>> {
        let mut longest_match = None;

        let mut current_state = self.start_state.expect("No start state set for dfa");
        for (index, current_atom) in input.chars().enumerate() {
            if self.accepting_states.contains(&current_state) {
                longest_match = Some(0..index);
            }
            match self.traverse_from(current_state, current_atom) {
                Some(next_state) => current_state = next_state,
                None => break,
            }
        }
        longest_match
    }

    pub fn match_longest_substring(&self, input: &str) -> Option<Range<usize>> {
        let mut longest_substring = None;

        for (index, _) in input.chars().enumerate() {
            if let Some(prefix) = self.match_longest_prefix(&input[index..]) {
                if prefix.len() > longest_substring.as_ref().unwrap_or(&(0..0)).len() {
                    longest_substring = Some(prefix.start + index..prefix.end + index)
                }
            }
        }
        longest_substring
    }

    pub fn traverse_from(&self, from_state: u32, atom: char) -> Option<u32> {
        if let Some(transitions) = self.from_transitions.get(&from_state) {
            for (to_state, atoms_set) in transitions {
                if atoms_set.contains(&Some(atom)) {
                    return Some(*to_state);
                }
            }
        }
        None
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

    pub fn add_state(&mut self) -> u32 {
        let states_before_adding = self.states;
        self.states += 1;
        self.kind = AutomatonKind::Nfa;
        states_before_adding
    }

    pub fn add_transition(&mut self, from_state: u32, to_state: u32, atom: Option<char>) {
        if let Some(c) = atom {
            self.alphabet.insert(c);
        }
        self.add_from_transition(from_state, to_state, atom);
        self.add_to_transition(from_state, to_state, atom);
        self.kind = AutomatonKind::Nfa;
    }

    pub fn clear_accepting(&mut self) {
        self.accepting_states.clear();
        self.kind = AutomatonKind::Nfa;
    }

    pub fn set_accepting(&mut self, state: u32, accepting: bool) {
        if state < self.states {
            self.kind = AutomatonKind::Nfa;
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
            self.kind = AutomatonKind::Nfa;
        }
    }

    pub fn intersection(&self, other: &Automaton) -> Automaton {
        // For each pair of states,
        let mut mul_dfa = Automaton::new();
        let mul_alphabet: BTreeSet<char> = self.alphabet.union(&other.alphabet).cloned().collect();
        let mut pair_to_dfa: HashMap<(u32, u32), u32> = HashMap::new();
        for self_from_state in 0..self.states {
            for other_from_state in 0..other.states {
                let mul_from_state = match pair_to_dfa.get(&(self_from_state, other_from_state)) {
                    Some(mul_state) => *mul_state,
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
                        self.traverse_from(self_from_state, *atom),
                        other.traverse_from(other_from_state, *atom),
                    ) {
                        // There is a transition from from_state to a to_state via atom for both self and other
                        let mul_to_state = match pair_to_dfa.get(&(self_to_state, other_to_state)) {
                            Some(mul_state) => *mul_state,
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
                    mul_dfa.set_start_state(*mul_start_state);
                }
            }
        }
        mul_dfa
    }

    pub fn to_dot_format(&self) -> String {
        let lines = std::iter::once("digraph g {".into())
            .chain((0..self.states).map(|state| {
                format!(
                    "{} [shape={} peripheries={}];",
                    state,
                    if Some(state) == self.start_state {
                        "box"
                    } else {
                        "circle"
                    },
                    if self.accepting_states.contains(&state) {
                        "2"
                    } else {
                        "1"
                    }
                )
            }))
            .chain(
                self.from_transitions
                    .iter()
                    .flat_map(|(from_state, to_states)| {
                        to_states.iter().map(move |(to_state, symbols)| {
                            format!(
                                "{} -> {} [label=\"{}\"];",
                                from_state,
                                to_state,
                                symbols
                                    .iter()
                                    .filter_map(|s| *s)
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>()
                                    .join(", ")
                            )
                        })
                    }),
            )
            .chain(std::iter::once("}".into()))
            .collect::<Vec<String>>();
        lines.join("\n")
    }

    fn get_equivalent_states(&self) -> HashMap<u32, BTreeSet<u32>> {
        let marked_states = self.get_marked_states_table();
        let mut equivalent_composite_states = HashMap::new();
        for s1 in 0..self.states {
            let mut equivalent_to_s1 = BTreeSet::new();
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
        let dead_state = self.states;
        let mut marked_states_table: Vec<Vec<bool>> =
            vec![vec![false; self.states as usize + 1]; self.states as usize + 1];
        for accepting_state in &self.accepting_states {
            // Dead state is never accepting
            marked_states_table[*accepting_state as usize][dead_state as usize] = true;
            marked_states_table[dead_state as usize][*accepting_state as usize] = true;

            for non_accepting_state in
                (0..self.states).filter(|e| !self.accepting_states.contains(e))
            {
                marked_states_table[non_accepting_state as usize][*accepting_state as usize] = true;
                marked_states_table[*accepting_state as usize][non_accepting_state as usize] = true;
            }
        }
        let mut marked_a_pair = true;
        while marked_a_pair {
            marked_a_pair = false;

            // Choose a pair of states
            'mark: for s1 in 0..=self.states {
                for s2 in 0..s1 {
                    if !marked_states_table[s1 as usize][s2 as usize] {
                        // Check if there is any transition from (s1, s2) to a marked pair
                        for c in &self.alphabet {
                            match (self.traverse_from(s1, *c), self.traverse_from(s2, *c)) {
                                (None, Some(s2_to_state)) => {
                                    // s1 transitions to dead state, there is a transition for s2
                                    if marked_states_table[dead_state as usize]
                                        [s2_to_state as usize]
                                    {
                                        marked_states_table[s1 as usize][s2 as usize] = true;
                                        marked_states_table[s2 as usize][s1 as usize] = true;
                                        marked_a_pair = true;
                                        break 'mark;
                                    }
                                }
                                (Some(s1_to_state), None) => {
                                    // s2 transitions to dead state, there is a transition for s1
                                    if marked_states_table[dead_state as usize]
                                        [s1_to_state as usize]
                                    {
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

    /// Returns the set of states that can be reached from a given starting state
    /// without reading any input (only traversing epsilon-transitions)
    ///
    /// # Arguments
    ///
    /// * `start_state` - The state from which traversal begins
    fn epsilon_closure(&self, start_state: u32) -> BTreeSet<u32> {
        let mut reachable_states = BTreeSet::new();
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
    fn atom_closure(&self, from_state_set: &BTreeSet<u32>, atom: char) -> BTreeSet<u32> {
        let mut atom_closure = BTreeSet::new();
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

    fn add_from_transition(&mut self, from_state: u32, to_state: u32, atom: Option<char>) {
        match self.from_transitions.get_mut(&from_state) {
            Some(to_states) => {
                // There is some transition from from_state to some other state
                if let Some(atoms) = to_states.get_mut(&to_state) {
                    // There is already some transition from from_state to to_state
                    atoms.insert(atom);
                } else {
                    // There is no transition from from_state to to_state
                    let mut atoms_set = BTreeSet::new();
                    atoms_set.insert(atom);
                    to_states.insert(to_state, atoms_set); // Create empty atoms set
                }
            }
            None => {
                // There is no transition from from_state to any other state
                let mut to_states = HashMap::new();
                let mut atoms_set = BTreeSet::new(); // atoms_set for transitions from from_state to to_state
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
                    let mut atoms_set = BTreeSet::new();
                    atoms_set.insert(atom);
                    from_states.insert(from_state, atoms_set); // Create empty atoms set
                }
            }
            None => {
                // There is no transition from any other state to to_state
                let mut from_states = HashMap::new();
                let mut atoms_set = BTreeSet::new(); // atoms_set for transitions from from_state to to_state
                atoms_set.insert(atom);
                from_states.insert(from_state, atoms_set);
                self.to_transitions.insert(to_state, from_states);
            }
        }
    }

    fn add_states(&mut self, states: u32) {
        self.states += states;
    }
}

impl Default for Automaton {
    fn default() -> Self {
        Self::new()
    }
}

fn nfa_to_dfa(automaton: &Automaton) -> Automaton {
    match automaton.start_state {
        Some(start_state) => {
            let mut minimized_dfa = Automaton::new();
            let mut comp_to_dfa = HashMap::new();

            let mut visited_comp = BTreeSet::new();
            let mut to_visit_comp = VecDeque::new();

            let comp_start_state = automaton.epsilon_closure(start_state);

            to_visit_comp.push_back(comp_start_state.clone());
            while let Some(from_comp) = to_visit_comp.pop_front() {
                visited_comp.insert(from_comp.clone());
                let from_dfa_id = match comp_to_dfa.get(&from_comp) {
                    Some(dfa_id) => *dfa_id,
                    None => {
                        let dfa_id = minimized_dfa.add_state();
                        comp_to_dfa.insert(from_comp.clone(), dfa_id);
                        minimized_dfa.set_accepting(
                            dfa_id,
                            from_comp
                                .iter()
                                .any(|s| automaton.accepting_states.contains(&s)), // state is accepting if any of the states in to_comp is accepting
                        );
                        dfa_id
                    }
                };
                for c in &automaton.alphabet {
                    let to_comp = automaton.atom_closure(&from_comp, *c);
                    if !to_comp.is_empty() {
                        if let Some(to_dfa_id) = comp_to_dfa.get(&to_comp) {
                            // Composite state is already in the minimized dfa
                            minimized_dfa.add_transition(from_dfa_id, *to_dfa_id, Some(*c));
                        } else {
                            // Composite state is not in the minimized dfa
                            let to_dfa_id = minimized_dfa.add_state();
                            minimized_dfa.set_accepting(
                                to_dfa_id,
                                to_comp
                                    .iter()
                                    .any(|s| automaton.accepting_states.contains(&s)), // state is accepting if any of the states in to_comp is accepting
                            );
                            to_visit_comp.push_back(to_comp.clone());
                            comp_to_dfa.insert(to_comp, to_dfa_id);
                            minimized_dfa.add_transition(from_dfa_id, to_dfa_id, Some(*c));
                        }
                    }
                }
            }

            minimized_dfa.set_start_state(comp_to_dfa[&comp_start_state]);
            minimized_dfa.kind = AutomatonKind::Dfa;
            minimized_dfa
        }
        None => {
            panic!("No starting state");
        }
    }
}

fn dfa_to_minimized_dfa(automaton: &Automaton) -> Automaton {
    let equivalent_states = automaton.get_equivalent_states();
    let mut comp_state_to_dfa = HashMap::new();
    let mut min_dfa = Automaton::new();

    // First add all states to minimized dfa
    for (s1, equivalent_to_s1) in &equivalent_states {
        if let Some(dfa_state_id) = comp_state_to_dfa.get(s1) {
            // s1 has already been added to min_dfa
            let dfa_state_id = *dfa_state_id;
            comp_state_to_dfa.insert(s1, dfa_state_id);
        } else {
            let dfa_state_id = min_dfa.add_state();
            for equivalent_state in equivalent_to_s1 {
                comp_state_to_dfa.insert(equivalent_state, dfa_state_id);
            }

            if let Some(comp_start_state) = automaton.start_state {
                if equivalent_to_s1.contains(&comp_start_state) {
                    min_dfa.set_start_state(dfa_state_id);
                }
            }

            // Set accepting if all states in comp is accepting
            min_dfa.set_accepting(
                dfa_state_id,
                equivalent_to_s1
                    .iter()
                    .all(|&state| automaton.accepting_states.contains(&state)),
            );
        }
    }

    // Then add all transitions
    for from_state in equivalent_states.keys() {
        for c in &automaton.alphabet {
            if let Some(to_state) = automaton.traverse_from(*from_state, *c) {
                if let (Some(dfa_from_state), Some(dfa_to_state)) = (
                    comp_state_to_dfa.get(&from_state),
                    comp_state_to_dfa.get(&to_state),
                ) {
                    min_dfa.add_transition(*dfa_from_state, *dfa_to_state, Some(*c));
                }
            }
        }
    }

    min_dfa.kind = AutomatonKind::MinDfa;
    min_dfa
}
