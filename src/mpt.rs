use crate::get_bit_direction;
use crate::node_generic::*;
use crate::structs::*;
use serde::{Deserialize, Serialize};
use talk::crypto::primitives::hash::Hash;

/**
 * Introduction to the project:
 *
 * In this representation of the Merkle Patricia Tree,
 * • true <--> 1 <--> Right
 * • false <--> 0 <--> Left
 *
**/

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
pub struct MerkleTree<K, V>
where
    K: Serialize,
    V: Serialize,
{
    root: Box<NodeGeneric<K, V>>,
}

impl<K, V> MerkleTree<K, V>
where
    K: Serialize + Clone + Eq,
    V: Serialize + Clone,
{
    pub fn new() -> MerkleTree<K, V> {
        let n = Box::new(NodeGeneric::new_internal_default());
        MerkleTree { root: n }
    }

    pub fn get_root(&self) -> &NodeGeneric<K, V> {
        &self.root
    }

    pub fn get_mut_root(&mut self) -> &mut NodeGeneric<K, V> {
        &mut self.root
    }

    pub fn insert(&mut self, key_to_add: K, value_to_add: V) -> NodeGeneric<K, V> {
        self.root.insert(key_to_add, value_to_add, 0)
    }

    pub fn get_node(&self, key: K) -> Result<&NodeGeneric<K, V>, ()> {
        self.root.find_path(&key, 0)
    }

    pub fn get_value(&self, key: K) -> V {
        match self.get_node(key).unwrap() {
            NodeGeneric::Leaf(n) => n.get_value().clone(),
            _ => panic!(),
        }
    }
}

impl<K, V> MerkleTree<K, V>
where
    K: Serialize + Clone + Eq,
    V: Serialize + Clone,
{
    /// Returns a Proof for a specific leaf (== specific source of payment)
    pub fn prove(&mut self, key: K) -> Proof {
        let mut siblings = Vec::<Sibling>::new();
        let node_err = self.get_node(key.clone());

        let mut_node_err = match node_err {
            Ok(n) => {}
            Err(e) => return Proof::new(siblings),
        };

        self.root.get_siblings(&key, 0, &mut siblings);
        siblings.reverse();
        Proof::new(siblings)
    }

    pub fn compute_hashes(&mut self) -> Hash {
        self.root.compute_hashes()
    }

    pub fn compute_hashes_prove(&mut self, key: K) -> Proof {
        self.compute_hashes();
        self.prove(key)
    }
}
