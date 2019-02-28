use super::parse_tree;

#[derive(Debug)]
pub struct Automaton {
    pub id:String
}

impl Automaton {
    fn get_id(&self)->String {
        self.id.clone()
    }
}