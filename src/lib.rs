pub mod atom;
pub mod residue;
pub mod chain;
pub mod model;
pub mod structure;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::collections::HashMap;
use atom::Atom;
use residue::Residue;
use chain::Chain;
use model::Model;
use structure::Structure;


pub struct PDBIO {
    pub filename: String,
}

fn parse_u32(number : &String) -> u32 {
    let number = number.trim().parse::<u32>();
    let number = match number {
        Ok(number) => number,
        Err(e) => {
            println!("Can not parse ({})", e);
            0
        }
    };
    number
}

fn parse_f32(number : &String) -> f32 {
    let number = number.trim().parse::<f32>();
    let number = match number {
        Ok(number) => number,
        Err(e) => {
            println!("Can not parse ({})", e);
            0.0
        }
    };
    number
}

impl PDBIO {
    pub fn parse(filename: &String) -> Structure {
        let mut structure: Structure = Default::default();
        let mut current_residue : Residue = Default::default();
        let mut current_model : Model = Default::default();
        let mut chain_id : String = String::from("");
        let mut previous_chain_id : String = String::from("");

        let file = File::open(filename).unwrap();
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();
            if line.starts_with("ATOM  ") || line.starts_with("HETATM") {
                // Atom name
                let name = line[12..17].trim().to_string();
                // Atom number
                let atom_number = parse_u32(&line[6..11].to_string());
                
                // Coord X
                let x = parse_f32(&line[30..39].to_string());
                
                // Coord Y
                let y = parse_f32(&line[38..47].to_string());

                // Coord Z
                let z = parse_f32(&line[46..55].to_string());
                
                // Occupancy
                let occ = parse_f32(&line[54..60].to_string());
                
                // B-factor
                let bfactor = parse_f32(&line[60..66].to_string());

                // Residue name
                let residue_name = line[17..20].trim().to_string();
                
                // Residue number
                let residue_number = parse_u32(&line[22..26].to_string());
                
                // Chain id
                chain_id = line[21..22].trim().to_string().to_uppercase();

                // Atom type
                let is_hetatom = line.starts_with("HETATM");

                // Chain logic
                if !current_model.chains.contains_key(&chain_id) {
                    current_model.chains.insert(chain_id.clone(), Chain::new(chain_id.clone(), Vec::new()));
                }
                
                if current_residue.is_empty() {
                    // First parsed residue
                    current_residue = Residue::new(residue_name.clone(), residue_number.clone(), Vec::new());
                    previous_chain_id = chain_id.clone();
                } else if current_residue.name == residue_name && current_residue.number == residue_number {
                    // Add atoms to the current residue
                    current_residue.atoms.push(Atom::new(name, atom_number, x, y, z, occ, bfactor, is_hetatom));
                } else {
                    // Next residue
                    if previous_chain_id != chain_id {
                        if let Some(chain) = current_model.chains.get_mut(&previous_chain_id) {
                            chain.residues.push(current_residue);
                        }
                        previous_chain_id = chain_id.clone();
                    } else {
                        if let Some(chain) = current_model.chains.get_mut(&chain_id) {
                            chain.residues.push(current_residue);
                        }
                    }
                    current_residue = Residue::new(residue_name, residue_number, Vec::new());
                    current_residue.atoms.push(Atom::new(name, atom_number, x, y, z, occ, bfactor, is_hetatom));
                }
            }

            // Model logic
            if line.starts_with("MODEL ") {
                let model_number = parse_u32(&line[5..26].to_string());
                if current_model.id != model_number {
                    structure.models.push(current_model);
                    current_model = Model::new(model_number, HashMap::new());
                }
            }
        }

        // Final residue
        let cid = if previous_chain_id != chain_id { previous_chain_id.clone() } else{ chain_id.clone() };
        if let Some(chain) = current_model.chains.get_mut(&cid) {
            chain.residues.push(current_residue);
        }

        structure.models.push(current_model);
        structure
    }
}
