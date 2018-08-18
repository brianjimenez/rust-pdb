use std::io::{BufReader,BufRead};
use std::fs::File;

#[derive(Debug)]
pub struct Atom {
    pub name: String,
    pub number: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Atom {
    fn new(name: String, number: u32, x: f32, y: f32, z: f32) -> Atom {
        Atom {
            name,
            number,
            x,
            y,
            z,
        }
    }
}


pub struct PDBIO {
    pub filename: String,
}

impl PDBIO {
    pub fn parse(filename: &String) -> Vec<Atom> {
        let mut atoms: Vec<Atom> = Vec::new();

        let file = File::open(filename).unwrap();
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();
            if line.starts_with("ATOM  ") {
                let name = line[12..16].trim().to_string();
                let atom_number = line[6..11].trim().parse::<u32>();
                let atom_number = match atom_number {
                    Ok(atom_number) => atom_number,
                    Err(e) => {
                        println!("Can not parse atom number ({})", e);
                        0
                    }
                };
                let x_coord = line[30..39].trim().parse::<f32>();
                let x = match x_coord {
                    Ok(x) => x,
                    Err(e) => {
                        println!("Can not parse x coordinate ({})", e);
                        0.0
                    }
                };
                let y_coord = line[38..47].trim().parse::<f32>();
                let y = match y_coord {
                    Ok(y) => y,
                    Err(e) => {
                        println!("Can not parse y coordinate ({})", e);
                        0.0
                    }
                };
                let z_coord = line[46..55].trim().parse::<f32>();
                let z = match z_coord {
                    Ok(z) => z,
                    Err(e) => {
                        println!("Can not parse z coordinate ({})", e);
                        0.0
                    }
                };

                atoms.push(Atom::new(name, atom_number, x, y, z));
            }
        }
        atoms
    }
}
