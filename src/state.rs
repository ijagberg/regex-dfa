use super::transition::Transition;

#[derive(Debug)]
pub struct State<'a> {
    accepting: bool,
    from_transitions: Vec<&'a Transition>,
    to_transitions: Vec<&'a Transition>,
}

impl<'a> State<'a> {
    pub fn new(accepting: bool) -> State<'a> {
        State {
            accepting,
            from_transitions: Vec::new(),
            to_transitions: Vec::new(),
        }
    }

    fn add_from_transition(&mut self, from_transition: &'a Transition) {
        self.from_transitions.push(from_transition);
    }
}
