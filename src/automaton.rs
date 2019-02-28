use super::parse_tree::ParseTree;

#[derive(Debug)]
pub struct Automaton {
    states: usize,
    transitions: Vec<Vec<Option<char>>>,
}

impl Automaton {
    pub fn from(parse_tree: ParseTree) {
        Automaton::from_tree(&parse_tree);
    }

    fn from_tree(parse_tree: &ParseTree) {
        let mut dfa = Automaton {
            states: 0_usize,
            transitions: Vec::new(),
        };
        match parse_tree {
            ParseTree::Concatenation { left, right } => {
                let left_dfa = Automaton::from_tree(left);
                let right_dfa = Automaton::from_tree(right);
                dfa.add_state();
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
            ParseTree::Atom(c) => {}
            ParseTree::Empty => {}
        }
    }

    fn add_state(&mut self) {
        self.states += 1;
        self.transitions.push(vec![None; self.states]);
    }
}
