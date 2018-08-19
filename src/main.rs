extern crate pdb;

use std::env;
use pdb::PDBIO;
use pdb::chain::Chain;


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("Parsing PDB file {}", filename);

    let chains : Vec<Chain> = PDBIO::parse(filename);

    println!("{} chains found", chains.len());
    for chain in chains {
        println!("{}: {}", chain.id, chain.residues.len());
    }
}

