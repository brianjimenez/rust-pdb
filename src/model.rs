use chain::Chain;

#[derive(Debug)]
pub struct Model {
    pub id: u32,
    pub chains: Vec<Chain>,
}

impl Model {
    pub fn new(id: u32, chains: Vec<Chain>) -> Model {
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
            chains: Vec::new(),
        }
    }
}