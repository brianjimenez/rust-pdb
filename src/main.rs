extern crate pdb;

use std::env;
use pdb::PDBIO;
use pdb::Residue;


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("PDB file {}", filename);

    let residues : Vec<Residue> = PDBIO::parse(filename);

    println!("{}.{}", residues[0].name, residues[0].number);
    let atom = &residues[0].atoms[0];
    println!("{:?}", atom);
}

