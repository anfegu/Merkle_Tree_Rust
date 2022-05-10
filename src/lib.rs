#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(in_band_lifetimes)]
use sha2::Digest;
use std::collections::VecDeque;

pub type Data = Vec<u8>;
pub type Hash = Vec<u8>;

pub enum TreeElement
{
    // Leafs contain data.
    Leaf {
        value: u8,
        hash: Vec<u8>,
    },
    // Represents the internal node that'll form the possible layers
    Node {
        left: Box<TreeElement>,
        right: Box<TreeElement>,
        hash: Vec<u8>,
    },
}

impl TreeElement {
    /// Returns the hash depending on the type of node
    pub fn hash(&self) -> &Vec<u8>{
        match *self {
            TreeElement::Leaf { ref hash, .. } => hash,
            TreeElement::Node { ref hash, .. } => hash,
        }  
    }
    /// builds new leaf from data
    pub fn new_leaf(value: u8) -> TreeElement {
        let data: Data = vec!(value);
        TreeElement::Leaf {
            hash: hash_data(&data),
            value: value,
        }
    }

    /// Builds new internal node from resulting elements
    pub fn new_node(left: TreeElement, right: TreeElement) -> TreeElement {
        TreeElement::Node {
            hash: hash_concat(left.hash(), right.hash()),
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

mod proof;
pub use proof::{Path, Proof};

pub struct MerkleTree {
    root: TreeElement, //Merkle Root
    total_data: usize, //Elements amount 
    height: usize, //MerkleTree Layers
}

impl MerkleTree {

    /// Constructs a Merkle tree from given input data 
    pub fn construct(input: &Data) -> Option<MerkleTree> {
        let count = input.len();

        match count {
            0 => None,
            _ => {
                let mut height = 0;
                let mut layer = VecDeque::with_capacity(count);

                // merkletree's first layer leaves
                for val in input {
                    layer.push_back(TreeElement::new_leaf(*val));
                }

                // build Merkletree layer by layer
                while layer.len() > 1 {
                    let mut new_layer: VecDeque<TreeElement> =
                        VecDeque::with_capacity(layer.len() / 2);

                    while !layer.is_empty() {
                        // if have one element only, it'll be the Leaf
                        if layer.len() == 1 {
                            new_layer.push_back(layer.pop_front().unwrap());
                        } else {
                            //build the Node
                            let left_node = layer.pop_front().unwrap();
                            let right_node = layer.pop_front().unwrap();
                            let node = TreeElement::new_node(left_node, right_node);
                            new_layer.push_back(node);
                        }
                    }
                    // increase the height of tree
                    height += 1;
                    // pass our prepared queue to the next iteration if any
                    layer = new_layer;
                }
                // the root node
                let root = layer.pop_back().unwrap();

                // return
                Some(MerkleTree {
                    root: root,
                    total_data: count,
                    height: height,
                })
            }
        }
    }  

    /// Verifies that the given input data produces the given root hash
    pub fn verify(input: &Data, root_hash: &Hash) -> bool {
        MerkleTree::construct(input).unwrap().root_hash() == root_hash    
    }

    /// Verifies that the given data and proof_path correctly produce the given root_hash
    pub fn verify_proof(data: &Data, proof: &Proof, root_hash: &Hash) -> bool {
        let tree = MerkleTree::construct(data).unwrap();
        proof.validate(tree.root_hash()) && proof.validate(root_hash)      
    }

    /// Get the root element in Merkletree
    pub fn root_hash(&self) -> &Vec<u8> {
        self.root.hash()
    }

    /// Get the height or number of layers in Merkletree
    pub fn get_height(&self) -> usize {
         self.height
    }

    /// Get total amount of data in Merkletree
    pub fn get_total_data(&self) -> usize {
        self.total_data
    }

    ///Handle empty Merkletree
    pub fn is_empty(&self) -> bool {
        self.total_data == 0
    }

    ///Constructor the proof
    pub fn get_proof(&self, value: u8) -> Option<Proof>{
        let data: Data = vec!(value);
        let cal_hash = hash_data(&data);
        let rhash = self.root_hash();
        Path::create_path(&self.root, &cal_hash).map(|p| {
            Proof::new(value, rhash, p)
        },
        )
    }
}

fn hash_data(data: &Data) -> Hash {
    sha2::Sha256::digest(data).to_vec()
}

fn hash_concat(h1: &Hash, h2: &Hash) -> Hash {
    let h3 = h1.iter().chain(h2).copied().collect();
    hash_data(&h3)
}

#[cfg(test)]
mod tests;

