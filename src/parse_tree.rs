#[derive(Debug)]
pub enum ParseTree {
    Or {
        left: Box<ParseTree>,
        right: Box<ParseTree>,
    },
    Concatenation {
        left: Box<ParseTree>,
        right: Box<ParseTree>,
    },
    Star {
        inner: Box<ParseTree>,
    },
    Question {
        inner: Box<ParseTree>,
    },
    Plus {
        inner: Box<ParseTree>,
    },
    Atom(char),
    Empty,
}

impl ParseTree {
    pub fn from(input: &str) -> ParseTree {
        let input_mut: Vec<char> = input.chars().collect();
        let mut iter = input_mut.iter().peekable();
        ParseTree::build_tree(&mut iter)
    }

    fn build_tree(mut iter: &mut std::iter::Peekable<std::slice::Iter<'_, char>>) -> ParseTree {
        let tree = ParseTree::build_term(&mut iter);
        match iter.peek() {
            Some('|') => {
                iter.next();
                let next_term_tree = ParseTree::build_tree(&mut iter);
                let or_tree = ParseTree::Or {
                    left: Box::new(tree),
                    right: Box::new(next_term_tree),
                };
                or_tree
            }
            _ => tree,
        }
    }

    fn build_term(mut iter: &mut std::iter::Peekable<std::slice::Iter<'_, char>>) -> ParseTree {
        let mut factor_tree = ParseTree::Empty;
        while let Some(c) = iter.peek() {
            match c {
                ')' => {
                    break;
                }
                '|' => {
                    break;
                }
                _ => {
                    let next_factor_tree = ParseTree::build_factor(&mut iter);
                    factor_tree = ParseTree::Concatenation {
                        left: Box::new(factor_tree),
                        right: Box::new(next_factor_tree),
                    };
                }
            }
        }
        factor_tree
    }

    fn build_factor(mut iter: &mut std::iter::Peekable<std::slice::Iter<'_, char>>) -> ParseTree {
        let mut base_tree = ParseTree::build_base(&mut iter);
        while let Some('*') = iter.peek() {
            iter.next();
            base_tree = ParseTree::Star {
                inner: Box::new(base_tree),
            };
        }
        base_tree
    }

    fn build_base(iter: &mut std::iter::Peekable<std::slice::Iter<'_, char>>) -> ParseTree {
        match iter.next() {
            Some('(') => {
                let tree = ParseTree::build_tree(iter);
                if let Some(')') = iter.next() {
                    tree
                } else {
                    panic!("Invalid regular expression, expected a ')' to close a '('");
                }
            }
            Some('\\') => {
                let atom_tree = ParseTree::Atom('\\');
                atom_tree
            }
            Some(c) => {
                let atom_tree = ParseTree::Atom(*c);
                atom_tree
            }
            None => panic!("Invalid regular expression"),
        }
    }
}