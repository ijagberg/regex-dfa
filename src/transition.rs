use super::state::State;

#[derive(Debug)]
pub struct Transition {
    atom: char,
    from_state: State,
    to_state: State,
}