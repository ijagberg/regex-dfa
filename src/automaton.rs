use super::parse_tree::ParseTree;
use super::state::State;
use super::transition::Transition;

#[derive(Debug)]
pub struct Automaton<'a> {
    states: Vec<State<'a>>,
    transitions: Vec<Transition<'a>>,
    start_state: Option<&'a State<'a>>,
}

impl<'a> Automaton<'a> {
    fn new() -> Automaton<'a> {
        Automaton {
            states: Vec::new(),
            transitions: Vec::new(),
            start_state: None,
        }
    }

    fn add_transition(&mut self, from_state: &'a mut State, to_state: &'a mut State, atom: char) {
        let transition = Transition::new(atom, from_state, to_state);
        self.transitions.push(transition);
        from_state.add_from_transition(&transition);
    }

    pub fn from(parse_tree: ParseTree) {
        Automaton::from_tree(&parse_tree);
    }

    fn from_tree(parse_tree: &ParseTree) {
        let mut dfa = Automaton {
            states: Vec::new(),
            transitions: Vec::new(),
            start_state: None,
        };
        match parse_tree {
            ParseTree::Concatenation { left, right } => {
                let left_dfa = Automaton::from_tree(left);
                let right_dfa = Automaton::from_tree(right);
            }
            ParseTree::Or { left, right } => {
                let left_dfa = Automaton::from_tree(left);
                let right_dfa = Automaton::from_tree(right);
            }
            ParseTree::Star { inner } => {
                let inner_dfa = Automaton::from_tree(inner);
            }
            ParseTree::Question { inner } => {
                let inner_dfa = Automaton::from_tree(inner);
            }
            ParseTree::Plus { inner } => {
                let inner_dfa = Automaton::from_tree(inner);
            }
            ParseTree::Atom(c) => {
                let start_state = State::new(false);
                let end_state = State::new(true);
                let atom_dfa = Automaton::new();
            }
            ParseTree::Empty => {}
        }
    }
}
