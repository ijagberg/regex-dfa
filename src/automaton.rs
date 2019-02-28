use super::parse_tree::ParseTree;

#[derive(Debug)]
pub struct Automaton {}

impl Automaton {
    pub fn from(parse_tree: ParseTree) {
        Automaton::from_tree(&parse_tree);
    }

    fn from_tree(parse_tree: &ParseTree) {
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
            ParseTree::Atom(c) => {}
            ParseTree::Empty => {}
        }
    }
}
