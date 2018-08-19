use residue::Residue;

#[derive(Debug)]
pub struct Chain {
    pub id: String,
    pub residues: Vec<Residue>,
}

impl Chain {
    pub fn new(id: String, residues: Vec<Residue>) -> Chain {
        Chain {
            id,
            residues,
        }
    }
}

impl Default for Chain {
    fn default() -> Chain {
        Chain {
            id: "".to_string(),
            residues: Vec::new(),
        }
    }
}