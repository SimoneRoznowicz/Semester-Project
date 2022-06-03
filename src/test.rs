use crate::client_verify::*;
use crate::{mpt::*, node_generic::*, structs::*};
use talk::crypto::primitives::hash::hash;
use talk::crypto::primitives::hash::Hash;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaf_new() {
        let l = Leaf::<&str, u8>::new("key", 5);
        assert_eq!(l.get_key(), &"key");
        assert_eq!(l.get_value(), &5);
    }

    #[test]
    fn Internal_new() {
        let left_child = Leaf::<&str, u8>::new("left", 5).into();
        let right_child = Leaf::<&str, u8>::new("right", 7).into();
        let i = Internal::new(left_child, right_child, None);

        match (i.get_left(), i.get_right()) {
            (NodeGeneric::Leaf(l), NodeGeneric::Leaf(r)) => {
                assert_eq!(l.get_key(), &"left");
                assert_eq!(l.get_value(), &5);
                assert_eq!(r.get_key(), &"right");
                assert_eq!(r.get_value(), &7);
            }
            _ => panic!("Error"),
        };
    }

    #[test]
    fn empty_get_hash() {
        let e = Empty::new();
        assert_eq!(hash(&()).unwrap(), Empty::get_hash());
    }

    #[test]
    fn find_path_insert_test1() {
        //insert when I find an empty node (so I move the leaf)
        let left_empty = Empty::new().into();
        let right_empty = Empty::new().into();
        let mut root: NodeGeneric<&str, i32> = Internal::new(left_empty, right_empty, None).into();

        root.insert("ciao", 55, 0);

        match root.find_path("ciao", 0).unwrap() {
            NodeGeneric::Leaf(n) => assert_eq!(n.get_value(), &55),
            _ => assert!(false),
        }
    }

    #[test]
    fn find_path_insert_test2() {
        //insert when I find a leaf (move the leaf, and then put the key-value in an empty node)
        let left = Leaf::<&str, u8>::new("Hello", 55).into();
        let right_empty = Empty::new().into();
        let mut root: NodeGeneric<&str, u8> = Internal::new(left, right_empty, None).into();
        //First 8 bits (there are 256 bit):
        //124: 01111100 --> hash(&"Hello")[0] == 124
        //32:  00100000 --> hash(&"ciao")[0] == 32
        //0 == FALSE AND 1 == TRUE

        root.insert("ciao", 55, 0);

        match root.find_path("ciao", 0).unwrap() {
            NodeGeneric::Leaf(n) => assert_eq!(n.get_value(), &55),
            _ => assert!(false),
        }
    }

    #[test]
    fn find_path_insert_test3() {
        //substitute the value of a Leaf (the key is already existing)
        let left = Leaf::<&str, u8>::new("Hello", 55).into();
        let right_empty = Empty::new().into();

        let mut root: NodeGeneric<&str, u8> = Internal::new(left, right_empty, None).into();

        root.insert("Hello", 123, 0);

        match root.find_path("Hello", 0).unwrap() {
            NodeGeneric::Leaf(n) => assert_eq!(n.get_value(), &123),
            _ => assert!(false),
        }
    }

    #[test]
    fn find_path_insert_test4() {
        //insert some nodes and then go through the tree to find each value
        let left_empty = Empty::new().into();
        let right_empty = Empty::new().into();

        let mut root: NodeGeneric<&str, u8> = Internal::new(left_empty, right_empty, None).into();

        root.insert("Hello", 1, 0);
        root.insert("AAAAA", 2, 0);
        root.insert("BBBBB", 3, 0);
        root.insert("CCCCC", 4, 0);
        root.insert("DDDDD", 5, 0);
        root.insert("EEEEE", 66, 0);
        root.insert("FFFFF", 7, 0);
        root.insert("GGGGG", 8, 0);
        root.insert("EEEEE", 6, 0);

        //124: 01111100 --> hash(&"Hello")[0] == 124
        //32:  00100000 --> hash(&"ciao")[0] == 32
        //0 == FALSE AND 1 == TRUE
        match root.find_path("Hello", 0).unwrap() {
            NodeGeneric::Leaf(n) => assert_eq!(n.get_value(), &1),
            _ => assert!(false),
        }
        match root.find_path("AAAAA", 0).unwrap() {
            NodeGeneric::Leaf(n) => assert_eq!(n.get_value(), &2),
            _ => assert!(false),
        }
        match root.find_path("EEEEE", 0).unwrap() {
            NodeGeneric::Leaf(n) => assert_eq!(n.get_value(), &6),
            _ => assert!(false),
        }
        match root.find_path("CCCCC", 0).unwrap() {
            NodeGeneric::Leaf(n) => assert_eq!(n.get_value(), &4),
            _ => assert!(false),
        }
        match root.find_path("GGGGG", 0).unwrap() {
            NodeGeneric::Leaf(n) => assert_eq!(n.get_value(), &8),
            _ => assert!(false),
        }
    }

    #[test]
    fn MerkleTree_new() {
        //a new Merkle Patricia Tree has a root which matches an Internal node, whose left
        //and right children match Empty nodes
        let mpt: MerkleTree<&str, u8> = MerkleTree::new();
        match mpt.get_root() {
            NodeGeneric::Internal(n) => {
                match n.get_left() {
                    NodeGeneric::Empty(_) => (),
                    _ => assert!(false),
                }
                match n.get_right() {
                    NodeGeneric::Empty(_) => (),
                    _ => assert!(false),
                }
                assert!(true)
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn MerkleTree_find_path_insert() {
        let mut mpt: MerkleTree<&str, u8> = MerkleTree::new();
        mpt.insert("HHHHH", 1);
        mpt.insert("AAAAA", 2);
        mpt.insert("BBBBB", 3);
        mpt.insert("CCCCC", 4);
        mpt.insert("DDDDD", 5);
        mpt.insert("EEEEE", 66);
        mpt.insert("FFFFF", 7);
        mpt.insert("GGGGG", 8);
        mpt.insert("EEEEE", 6);

        match mpt.get_node("AAAAA").unwrap() {
            NodeGeneric::Leaf(n) => assert_eq!(n.get_value(), &2),
            _ => assert!(false),
        }
        assert_eq!(mpt.get_value("AAAAA"), &2);
        match mpt.get_node("EEEEE").unwrap() {
            NodeGeneric::Leaf(n) => assert_eq!(n.get_value(), &6),
            _ => assert!(false),
        }
        assert_eq!(mpt.get_value("EEEEE"), &6);
        assert_eq!(mpt.get_value("HHHHH"), &1);
        assert_eq!(mpt.get_value("BBBBB"), &3);
        assert_eq!(mpt.get_value("FFFFF"), &7);
        assert_eq!(mpt.get_value("CCCCC"), &4);
        assert_eq!(mpt.get_value("GGGGG"), &8);
        assert_eq!(mpt.get_value("DDDDD"), &5);
    }

    #[test]
    fn root_compute_hashes_prove1() {
        let mut mpt: MerkleTree<&str, u8> = MerkleTree::new();
        mpt.insert("Hello", 1);
        mpt.insert("ciao", 2);

        let hash_Hello = hash(&(hash(&"Hello").unwrap(), hash(&1u8).unwrap())).unwrap();
        let hash_ciao = hash(&(hash(&"ciao").unwrap(), hash(&2u8).unwrap())).unwrap();

        let hash_empty = hash(&()).unwrap(); //hash(&"Hello").unwrap();
        let hash_internal = hash(&(hash_ciao, hash_Hello)).unwrap();
        let hash_root = hash(&(hash_internal, Empty::get_hash())).unwrap();

        assert_eq!(mpt.get_mut_root().compute_hashes(), hash_root);

        let proofHello = mpt.prove("Hello");
        let dir0 = &proofHello.get_siblings().get(0).unwrap().get_direction();
        let dir1 = &proofHello.get_siblings().get(1).unwrap().get_direction();

        match dir0 {
            Direction::Left => assert!(true),
            Direction::Right => assert!(false),
        }
        match dir1 {
            Direction::Left => assert!(false),
            Direction::Right => assert!(true),
        }
        assert_eq!(proofHello.get_siblings().len(), 2);

        let proofciao = mpt.prove("ciao");
        let dir0 = &proofciao.get_siblings().get(0).unwrap().get_direction();
        let dir1 = &proofciao.get_siblings().get(1).unwrap().get_direction();

        match dir0 {
            Direction::Left => assert!(false),
            Direction::Right => assert!(true),
        }
        match dir1 {
            Direction::Left => assert!(false),
            Direction::Right => assert!(true),
        }
        assert_eq!(proofciao.get_siblings().len(), 2);
    }

    #[test]
    fn root_compute_hashes_prove2() {
        let mut mpt: MerkleTree<&str, u8> = MerkleTree::new();
        mpt.insert("HHHHH", 1);
        mpt.insert("AAAAA", 2);
        mpt.insert("BBBBB", 3);
        mpt.insert("CCCCC", 4);
        mpt.insert("DDDDD", 5);
        mpt.insert("EEEEE", 66);
        mpt.insert("FFFFF", 7);
        mpt.insert("GGGGG", 8);
        mpt.insert("EEEEE", 6);
        //First 8 bits (there are 256 bit):
        //113: 01110001 --> HHHHH  //68:  01000100 --> AAAAA  //201: 11001001 --> BBBBB
        //157: 10011101 --> CCCCC  //12:  00001100 --> DDDDD  //100: 01100100 --> EEEEE
        //104: 01101000 --> FFFFF  //183: 10110111 --> GGGGG
        mpt.compute_hashes();
        print!("HHHHH\n\n");
        let proofHHHHH = mpt.prove("HHHHH");
        for sib in proofHHHHH.get_siblings() {
            print!("\n\nsib == {:?}", sib);
        }
        print!("\n\n\nAAAAA");
        let proofAAAAA = mpt.prove("AAAAA");
        for sib in proofAAAAA.get_siblings() {
            print!("\n\nsib == {:?}", sib);
        }
        print!("\n\n\nBBBBB");

        let proofBBBBB = mpt.prove("BBBBB");
        for sib in proofBBBBB.get_siblings() {
            print!("\n\nsib == {:?}", sib);
        }
        print!("\n\n\nCCCCC");

        let proofCCCCC = mpt.prove("CCCCC");
        for sib in proofCCCCC.get_siblings() {
            print!("\n\nsib == {:?}", sib);
        }
        print!("\n\n\nDDDDD");

        let proofDDDDD = mpt.prove("DDDDD");
        for sib in proofDDDDD.get_siblings() {
            print!("\n\nsib == {:?}", sib);
        }
        print!("\n\n\nEEEEE");

        let proofEEEEE = mpt.prove("EEEEE");
        for sib in proofEEEEE.get_siblings() {
            print!("\n\nsib == {:?}", sib);
        }
        print!("\n\n\nFFFFF");

        let proofFFFFF = mpt.prove("FFFFF");
        for sib in proofFFFFF.get_siblings() {
            print!("\n\nsib == {:?}", sib);
        }
        print!("\n\n\nGGGGG");

        let proofGGGGG = mpt.prove("GGGGG");
        for sib in proofGGGGG.get_siblings() {
            print!("\n\nsib == {:?}", sib);
        }
        print!("\n\n\n");
    }

    #[test]
    fn get_root_hash_test() {
        let mut mpt: MerkleTree<&str, u8> = MerkleTree::new();
        mpt.insert("HHHHH", 1);

        mpt.insert("AAAAA", 5);
        mpt.insert("AAAAA", 2);
        mpt.insert("BBBBB", 3);
        mpt.insert("CCCCC", 4);
        mpt.insert("DDDDD", 5);
        mpt.insert("EEEEE", 66);
        mpt.insert("FFFFF", 7);
        mpt.insert("GGGGG", 8);
        mpt.insert("EEEEE", 6);

        let mut hash_root = mpt.get_root().get_hash();
        mpt.compute_hashes();

        //"Hello" key does not exist in the mpt so expect an empty vector siblings
        let mut proof_nothing = mpt.prove("Hello");
        assert_eq!(proof_nothing.get_siblings().len(), 0);

        let mut proof = mpt.prove("GGGGG");
        let reconstructed_hash_root = get_root_hash(proof, 8u8, Id::new("GGGGG"));

        match mpt.get_root() {
            NodeGeneric::Internal(n) => {
                assert_eq!(n.get_current_hash().unwrap(), hash_root);
                print!("FIRST ASSERT PASSATO\n\n");
                assert_eq!(hash_root, reconstructed_hash_root);
                print!("SECOND ASSERT PASSED\n\n");
            }
            _ => assert!(false),
        }
    }
}
