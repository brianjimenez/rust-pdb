extern crate pdb;

use std::env;
use pdb::PDBIO;
use pdb::structure::Structure;


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("Parsing PDB file {}", filename);

    let protein : Structure = PDBIO::parse(filename);
    for model in protein.models.iter() {
        println!("Model {} - {} chains found", model.id, model.chains.len());
        for chain in model.chains.iter() {
            if chain.id == "I" {
                println!("{}: {}", chain.id, chain.residues.len());
                for residue in chain.residues.iter() {
                    println!("{}.{}", residue.name, residue.number);
                    println!("{:?}", residue);
                }
            }
        }
    }
}

