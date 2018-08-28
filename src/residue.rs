use atom::Atom;

#[derive(Debug)]
pub struct Residue {
    pub name: String,
    pub number: u32,
    pub atoms: Vec<Atom>,
}

impl Residue {
    pub fn new(name: String, number: u32, atoms: Vec<Atom>) -> Residue {
        Residue {
            name,
            number,
            atoms,
        }
    }

    pub fn is_empty(&self) -> bool {
        (self.name == "" && self.number == 0)
    }
}

impl Default for Residue {
    fn default() -> Residue {
        Residue {
            name: "".to_string(),
            number: 0,
            atoms: Vec::new(),
        }
    }
}
