use super::state::State;

#[derive(Debug)]
pub struct Transition<'a> {
    atom: char,
    from_state: &'a State<'a>,
    to_state: &'a State<'a>,
}

impl<'a> Transition<'a> {
    pub fn new(atom: char, from_state: &'a State, to_state: &'a State) -> Transition<'a> {
        Transition {
            atom,
            from_state,
            to_state,
        }
    }
}
