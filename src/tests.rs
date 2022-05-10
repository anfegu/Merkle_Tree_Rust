use super::*;

    #[test]
    fn empty_merkletree_test() {
        let data: Data = Vec::new();
        let merkletree = MerkleTree::construct(&data);
        assert!(merkletree.is_none());
    }
    
    #[test]
    fn build_merkletree_test_1() {
        let data:Data = vec!(1); // One data
        let merkletree = MerkleTree::construct(&data).unwrap();
        let test_hash = hash_data(&vec!(1));
        
        assert_eq!(merkletree.get_total_data(), 1); //Elements amount
        assert_eq!(merkletree.get_height(), 0); //No Layer
        assert!(!merkletree.is_empty()); //No empty
        assert_eq!(merkletree.root_hash(), &test_hash); //Test hash (root hash) 
    }

    #[test]
    fn build_merkletree_test_2() {
        let data:Data = vec!(1, 2); //Two Data
        let merkletree = MerkleTree::construct(&data).unwrap();
        let h1 = hash_data(&vec!(1)); //left
        let h2 = hash_data(&vec!(2)); //right
        let test_hash =hash_concat(&h1, &h2);
                
        assert_eq!(merkletree.get_total_data(), 2); //Elements amount
        assert_eq!(merkletree.get_height(), 1); //A Layer
        assert!(!merkletree.is_empty()); //No empty
        assert_eq!(merkletree.root_hash(), &test_hash); //Test hash (root hash) 
    }

    #[test]
    fn verify_hash_test() {
        let data = vec!(1,2,3,4); // Set of data
        
        //verify expected hashes like example
        let a1: Data = vec!(data[0]);
        let a2: Data = vec!(data[1]);
        let a3: Data = vec!(data[2]);
        let a4: Data = vec!(data[3]);

        let h1 = hash_data(&a1);
        let h2 = hash_data(&a2);
        let h3 = hash_data(&a3);
        let h4 = hash_data(&a4);

        let h5 = hash_concat(&h1, &h2);
        let h6 = hash_concat(&h3, &h4);

        let root_hash = hash_concat(&h5, &h6);

        //Method verify of the MerkleTree
        assert_eq!(MerkleTree::verify(&data,&root_hash), true);
    }

    #[test]
    fn verify_proof_test_fail() {
        let data = vec!(1,2,3,4);  // Set of data
        let tree = MerkleTree::construct(&data).unwrap();
        let proof = tree.get_proof(5); //5 is not found in data
    
        assert!(proof.is_none());
     }

     #[test]

     fn verify_proof_test_ok(){
        let data = &vec!(1,2,3,4);  // Set of data
        let tree = MerkleTree::construct(data).unwrap();

        for v in data{
            let proof = tree.get_proof(*v).unwrap();
            //Method verify_proof of the MerkleTree
            assert_eq!(MerkleTree::verify_proof(&data.clone(), &proof, &tree.root_hash()), true);
        }
     }