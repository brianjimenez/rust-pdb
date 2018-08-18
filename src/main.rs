extern crate pdb;

use std::env;
use pdb::PDBIO;
use pdb::Atom;


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("PDB file {}", filename);

    let atoms : Vec<Atom> = PDBIO::parse(filename);

    println!("{:?}", &atoms[0]);
}

