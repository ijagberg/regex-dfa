pub mod parse_tree;
pub mod automaton;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::parse_tree::ParseTree;
    use super::automaton::Automaton;

    #[test]
    fn test_from_atom() {
        let atom_tree = ParseTree::Atom('a');
        let atom_dfa = Automaton::from(&atom_tree);
        println!("{:#?}", atom_tree);
        println!("{:#?}", atom_dfa);
    }
}