use std::io::{BufReader,BufRead};
use std::fs::File;

#[derive(Debug)]
pub struct Atom {
    pub name: String,
    pub number: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub occupancy: f32,
    pub bfactor: f32,
}

impl Atom {
    fn new(name: String, number: u32, x: f32, y: f32, z: f32, occupancy: f32, bfactor: f32) -> Atom {
        Atom {
            name,
            number,
            x,
            y,
            z,
            occupancy,
            bfactor,
        }
    }
}


#[derive(Debug)]
pub struct Residue {
    pub name: String,
    pub number: u32,
    pub atoms: Vec<Atom>,
}

impl Residue {
    fn new(name: String, number: u32, atoms: Vec<Atom>) -> Residue {
        Residue {
            name,
            number,
            atoms,
        }
    }
}




pub struct PDBIO {
    pub filename: String,
}

impl PDBIO {
    pub fn parse(filename: &String) -> Vec<Atom> {
        let mut atoms: Vec<Atom> = Vec::new();
        let mut residues: Vec<Residue> = Vec::new();

        let file = File::open(filename).unwrap();
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();
            if line.starts_with("ATOM  ") {
                // Atom name
                let name = line[12..16].trim().to_string();
                // Atom number
                let atom_number = line[6..11].trim().parse::<u32>();
                let atom_number = match atom_number {
                    Ok(atom_number) => atom_number,
                    Err(e) => {
                        println!("Can not parse atom number ({})", e);
                        0
                    }
                };
                // Coord X
                let x_coord = line[30..39].trim().parse::<f32>();
                let x = match x_coord {
                    Ok(x) => x,
                    Err(e) => {
                        println!("Can not parse x coordinate ({})", e);
                        0.0
                    }
                };
                // Coord Y
                let y_coord = line[38..47].trim().parse::<f32>();
                let y = match y_coord {
                    Ok(y) => y,
                    Err(e) => {
                        println!("Can not parse y coordinate ({})", e);
                        0.0
                    }
                };
                // Coord Z
                let z_coord = line[46..55].trim().parse::<f32>();
                let z = match z_coord {
                    Ok(z) => z,
                    Err(e) => {
                        println!("Can not parse z coordinate ({})", e);
                        0.0
                    }
                };
                // Occupancy
                let occ = line[54..60].trim().parse::<f32>();
                let occ = match occ {
                    Ok(occ) => occ,
                    Err(e) => {
                        0.0
                    }
                };
                // B-factor
                // Occupancy
                let bfactor = line[60..66].trim().parse::<f32>();
                let bfactor = match bfactor {
                    Ok(bfactor) => bfactor,
                    Err(e) => {
                        0.0
                    }
                };

                atoms.push(Atom::new(name, atom_number, x, y, z, occ, bfactor));
            }
        }
        atoms
    }
}
