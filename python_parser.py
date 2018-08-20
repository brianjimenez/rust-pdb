from Bio.PDB.PDBParser import PDBParser
import sys

print("Parsing PDB file {}".format(sys.argv[1]))
pdb_parser = PDBParser(PERMISSIVE=True, QUIET=True)
structure = pdb_parser.get_structure(sys.argv[1], sys.argv[1])

print("{} chains found".format(len(structure[0])))
for chain in structure[0]:
    print("{}: {}".format(chain.id, len(chain)))

