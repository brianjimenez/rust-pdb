use chain::Chain;
use std::collections::HashMap;


#[derive(Debug)]
pub struct Model {
    pub id: u32,
    pub chains: HashMap<String, Chain>,
}

impl Model {
    pub fn new(id: u32, chains: HashMap<String, Chain>) -> Model {
        Model {
            id,
            chains,
        }
    }
}

impl Default for Model {
    fn default() -> Model {
        Model {
            id: 1,
            chains: HashMap::new(),
        }
    }
}