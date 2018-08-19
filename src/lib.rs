pub mod atom;
pub mod residue;
pub mod chain;
use std::io::{BufReader,BufRead};
use std::fs::File;
use atom::Atom;
use residue::Residue;
use chain::Chain;


pub struct PDBIO {
    pub filename: String,
}

impl PDBIO {
    pub fn parse(filename: &String) -> Vec<Chain> {
        let mut chains: Vec<Chain> = Vec::new();

        let mut current_residue : Residue = Default::default();
        let mut current_chain : Chain = Default::default();

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
                let bfactor = line[60..66].trim().parse::<f32>();
                let bfactor = match bfactor {
                    Ok(bfactor) => bfactor,
                    Err(e) => {
                        0.0
                    }
                };
                // Residue
                let residue_name = line[17..20].trim().to_string();
                // Residue number
                let residue_number = line[22..26].trim().parse::<u32>();
                let residue_number = match residue_number {
                    Ok(residue_number) => residue_number,
                    Err(e) => {
                        println!("Can not parse residue number ({})", e);
                        0
                    }
                };
                // Chain id
                let chain_id = line[21..22].trim().to_string().to_uppercase();
                
                // Residue logic
                if current_residue.name == "" && current_residue.number == 0 {
                    current_residue.name = residue_name.clone();
                    current_residue.number = residue_number.clone();
                }
                if current_residue.name == residue_name && current_residue.number == residue_number {
                    current_residue.atoms.push(Atom::new(name, atom_number, x, y, z, occ, bfactor));
                }
                else {
                    current_chain.residues.push(current_residue);
                    current_residue = Residue::new(residue_name, residue_number, Vec::new());
                    current_residue.atoms.push(Atom::new(name, atom_number, x, y, z, occ, bfactor));
                }

                // Chain logic
                if current_chain.id == "" {
                    current_chain.id = chain_id.clone();
                }
                if chain_id != current_chain.id {
                    chains.push(current_chain);
                    current_chain = Chain::new(chain_id, Vec::new());
                }
            }
        }
        chains.push(current_chain);
        chains
    }
}
