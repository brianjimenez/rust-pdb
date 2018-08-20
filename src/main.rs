extern crate pdb;

use std::env;
use pdb::PDBIO;
use pdb::model::Model;


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("Parsing PDB file {}", filename);

    let models : Vec<Model> = PDBIO::parse(filename);
    for model in models.iter() {
        println!("Model {} - {} chains found", model.id, model.chains.len());
        for chain in model.chains.iter() {
            println!("{}: {}", chain.id, chain.residues.len());
        }
    }
}

