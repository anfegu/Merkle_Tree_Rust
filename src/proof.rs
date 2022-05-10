use super::*;

/// Which side to put Hash on when concatinating proof hashes
pub enum HashDirection<T> {
    Left(T),
    Right(T),
}

pub struct Proof<'a> {
    /// The hashes to use when verifying the proof
    /// The first element of the tuple is which side the hash should be on when concatinating
    hashes: Vec<(Path, &'a Hash)>,
    value: u8,
}   

impl<'a> Proof<'a> {
    pub fn new(value: u8, root_hash: &'a Hash, path_item: Path) -> Self {
        Proof {
            hashes: vec!((path_item, root_hash)),
            value, 
        }
    }

     /// Validates the proof path
     pub fn validate(&self, root_hash: &Hash) -> bool {
        let (path, rhash) = &self.hashes[0];

        if &root_hash != rhash || &path.hash != root_hash {
            return false;
        }
        // recursive check the proof path
        self.validate_rec(&path)
    }

    fn validate_rec(&self, path_item: &Path) -> bool {
        match path_item.sub_item {
            Some(ref child) => {
                match path_item.resulting_hash {
                    Some(HashDirection::Left(ref hash)) => {
                        // Validating the node hash according to the fact that no_leaf hash  
                        // should be the 1st parameter when concatenating since it's positioned on the left
                        let calculated_hash = hash_concat(&hash, &child.hash);
                        // it should match the node's hash
                        (calculated_hash == path_item.hash) && self.validate_rec(child)
                    }
                    Some(HashDirection::Right(ref hash)) => {
                        // validating the node hash according to the fact that no_leaf hash 
                        // should be the 2st parameter when concatenating since it's positioned on the right
                        let calculated_hash = hash_concat( &child.hash, &hash);
                        // it should match the node's hash
                        (calculated_hash == path_item.hash) && self.validate_rec(child)
                    }
                    None => false,
                }
            }
            None => path_item.resulting_hash.is_none() && path_item.hash == hash_data(&(vec!(self.value))),
        }
    }

}

pub struct Path {
    hash: Hash,
    resulting_hash: Option<HashDirection<Hash>>,
    sub_item: Option<Box<Path>>,
}

impl Path {
    /// Recursively creates path of proof until element is found.
    /// None in case element hasn't been found in the Merkletree.
    pub fn create_path(node: &TreeElement, hash_to_find: &Hash) -> Option<Path> {
        match *node {
            TreeElement::Node {
                ref left,
                ref right,
                ref hash,
            } => Path::new_node_proof(hash, hash_to_find, left, right),
            TreeElement::Leaf {ref hash, ..} => Path::new_leaf_proof(hash, hash_to_find),
        }
    }

    /// Creates an item in the  proof path for a leaf
    fn new_leaf_proof(hash: &Hash, hash_to_find: &Hash) -> Option<Path> {
        if *hash == *hash_to_find {
            Some(Path {
                hash: hash.to_vec(),
                resulting_hash: None,
                sub_item: None,
            })
        } else {
            None
        }
    }

    /// Creates an item in the proof path for an internal node
    fn new_node_proof(
        hash: &Hash,
        hash_to_find: &Hash,
        left: &TreeElement,
        right: &TreeElement,
    ) -> Option<Path> {
    // Recursively go to the left node or to the right node
    Path::create_path(left, hash_to_find)
        .map(|item| {
            (item, Some(HashDirection::Right(right.hash().to_vec())))
        }).or_else(|| {
            let child_item = Path::create_path(right, hash_to_find);
            child_item.map(|item| {
                (item, Some(HashDirection::Left(left.hash().to_vec())))
            })
        }).map(|(path_item, res_hash)| {
      
            Path {
                hash: hash.to_vec(),
                resulting_hash: res_hash,
                sub_item: Some(Box::new(path_item)),
            }
        })
    }
}
