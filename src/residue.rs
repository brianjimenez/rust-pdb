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
