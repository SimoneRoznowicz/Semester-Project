use crate::get_bit_direction;
use crate::structs::*;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, boxed::Box, vec::Vec};
use talk::crypto::primitives::hash::{hash, Hash};


#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
pub enum NodeGeneric<K, V>
where
    K: Serialize,
    V: Serialize,
{
    Internal(Internal<K, V>),
    Leaf(Leaf<K, V>),
    Empty(Empty),
}


#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
pub struct Internal<K, V>
where
    K: Serialize,
    V: Serialize,
{
    left: Box<NodeGeneric<K, V>>,
    right: Box<NodeGeneric<K, V>>,
    my_hash: Option<Hash>,
}


#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Empty {}


#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Leaf<K, V>
where
    K: Serialize,
    V: Serialize,
{
    k: K,
    v: V,
    my_hash: Hash,
}


impl<K, V> NodeGeneric<K, V>
where
    K: Serialize + Clone + Eq,
    V: Serialize + Clone,
{
    pub fn new() -> Self {
        Self::Empty(Empty::new())
    }

    pub fn new_internal_default() -> Self {
        Self::Internal(Internal::new(NodeGeneric::new(), NodeGeneric::new(), None))
    }

    pub fn compute_hashes(&mut self) -> Hash {
        match self {
            NodeGeneric::Empty(n) => Empty::get_hash(),
            NodeGeneric::Leaf(n) => n.my_hash,
            NodeGeneric::Internal(n) => n.compute_hashes(),
        }
    }

    pub fn get_hash(&self) -> Hash {
        match self {
            NodeGeneric::Internal(n) => n.get_hash(),
            NodeGeneric::Leaf(n) => n.get_hash(),
            NodeGeneric::Empty(_) => Empty::get_hash(),
        }
    }

    pub fn find_path<Q: ?Sized>(&self, key: &Q, index: u8) -> Result<&NodeGeneric<K, V>, ()>
    where
        K: Borrow<Q>,
        Q: Serialize + Eq,
    {
        let my_hash = self.get_hash();
        let hash_of_given_key = hash(&key).unwrap();

        match self {
            NodeGeneric::Internal(n) => n.find_path(key, index),
            NodeGeneric::Leaf(n) => {
                if hash(&n.k).unwrap() == hash(&key).unwrap() {
                    Ok(&self)
                } else {
                    Err(())
                }
            }
            _ => Err(()),
        }
    }

    pub fn insert(&mut self, key_to_add: K, value_to_add: V, index: u8) -> NodeGeneric<K, V> {
        match self {
            NodeGeneric::Internal(n) => n.insert(key_to_add, value_to_add, index),
            NodeGeneric::Leaf(n) => n.insert(key_to_add, value_to_add, index),
            NodeGeneric::Empty(n) => n.insert(key_to_add, value_to_add, index),
        }
    }

    pub fn get_siblings<Q>(&self, key: &Q, index: u8, siblings: &mut Vec<Sibling>)
    where
        K: Borrow<Q>,
        Q: Serialize + Eq,
    {
        match &self {
            NodeGeneric::Internal(n) => n.get_siblings(key, index, siblings),
            NodeGeneric::Leaf(n) => (),
            NodeGeneric::Empty(n) => (),
            _ => panic!(),
        }
    }
    
    ///returns a Leaf node
    pub fn to_leaf(self) -> Leaf<K, V> {
        match self {
            NodeGeneric::Leaf(n) => n,
            _ => panic!("Node which is not a Leaf!"),
        }
    }

    ///returns an Internal node
    pub fn to_internal(self) -> Internal<K, V> {
        match self {
            NodeGeneric::Internal(n) => n,
            _ => panic!("Node which is not a Internal!"),
        }
    }

    ///returns an Empty node
    fn to_empty(self) -> Empty {
        match self {
            NodeGeneric::Empty(n) => n,
            _ => panic!("Node which is not a Empty!"),
        }
    }

}

impl<K, V> From<&mut Internal<K, V>> for NodeGeneric<K, V>
where
    K: Serialize + Clone + Eq,
    V: Serialize + Clone,
{
    fn from(i: &mut Internal<K, V>) -> Self {
        NodeGeneric::Internal(i.clone())
    }
}

impl<K, V> From<Internal<K, V>> for NodeGeneric<K, V>
where
    K: Serialize + Clone + Eq,
    V: Serialize + Clone,
{
    fn from(i: Internal<K, V>) -> Self {
        NodeGeneric::Internal(i)
    }
}

impl<K, V> Internal<K, V>
where
    K: Serialize + Clone + Eq,
    V: Serialize + Clone,
{
    pub fn new(l: NodeGeneric<K, V>, r: NodeGeneric<K, V>, h: Option<Hash>) -> Self {
        Internal {
            left: Box::new(l),
            right: Box::new(r),
            my_hash: h,
        }
    }

    ///Returns the hash of the node calling the method and asssigns a hash value to every Internal node.
    fn compute_hashes<Q>(&mut self) -> Hash
    where
        K: Borrow<Q>,
        Q: Serialize + Eq,
    {
        let this_hash = Internal::<K, V>::create_hash(
            self.get_mut_left().compute_hashes(),
            self.get_mut_right().compute_hashes(),
        );
        self.set_hash(Some(this_hash))
    }

    pub fn create_hash(l_hash: Hash, r_hash: Hash) -> Hash {
        hash(&(l_hash, r_hash)).unwrap()
    }

    pub fn get_current_hash(&self) -> &Option<Hash> {
        &self.my_hash
    }

    pub fn get_hash(&self) -> Hash {
        Internal::<K, V>::create_hash(self.left.get_hash(), self.right.get_hash())
    }

    fn set_hash(&mut self, h: Option<Hash>) -> Hash {
        self.my_hash = h;
        h.unwrap()
    }

    fn find_path<Q: ?Sized>(&self, key: &Q, index: u8) -> Result<&NodeGeneric<K, V>, ()>
    where
        K: Borrow<Q>,
        Q: Serialize + Eq,
    {
        let key_hash = hash(&key).unwrap();
        let direction = get_bit_direction(&key_hash.to_bytes(), index);

        if direction == true {
            self.get_right().find_path(key, index + 1)
        } else {
            self.get_left().find_path(key, index + 1)
        }
    }

    fn insert(&mut self, key_to_add: K, value_to_add: V, index: u8) -> NodeGeneric<K, V> {
        let key_hash = hash(&key_to_add).unwrap();
        let direction = get_bit_direction(&key_hash.to_bytes(), index);

        let mut side;
        if direction == true {
            side = &mut self.right;
        } else {
            side = &mut self.left;
        }

        let mut n = NodeGeneric::new();
        std::mem::swap(&mut n, side);

        match n.insert(key_to_add, value_to_add, index + 1) {
            n @ _ => {
                *side = Box::new(n);
                self.into()
            }
        }
    }

    fn get_siblings<Q>(&self, key: &Q, index: u8, siblings: &mut Vec<Sibling>)
    where
        K: Borrow<Q>,
        Q: Serialize + Eq,
    {
        let key_hash = hash(key).unwrap();
        let direction = get_bit_direction(&key_hash.to_bytes(), index);
        if direction == true {
            let l_node = self.get_left();
            match l_node {
                NodeGeneric::Internal(n) => {
                    siblings.push(Sibling::new(n.my_hash.unwrap(), Left {}.into()))
                }
                NodeGeneric::Leaf(n) => siblings.push(Sibling::new(n.my_hash, Left {}.into())),
                NodeGeneric::Empty(n) => {
                    siblings.push(Sibling::new(Empty::get_hash(), Left {}.into()))
                }
            }
            self.get_right().get_siblings(key, index + 1, siblings)
        } else {
            let r_node = self.get_right();
            match r_node {
                NodeGeneric::Internal(n) => {
                    siblings.push(Sibling::new(n.my_hash.unwrap(), Right {}.into()))
                }
                NodeGeneric::Leaf(n) => siblings.push(Sibling::new(n.my_hash, Right {}.into())),
                NodeGeneric::Empty(n) => {
                    siblings.push(Sibling::new(Empty::get_hash(), Right {}.into()))
                }
            }
            self.get_left().get_siblings(key, index + 1, siblings)
        }
    }

    pub fn get_mut_right(&mut self) -> &mut NodeGeneric<K, V> {
        &mut self.right
    }

    pub fn get_mut_left(&mut self) -> &mut NodeGeneric<K, V> {
        &mut self.left
    }

    pub fn get_right(&self) -> &NodeGeneric<K, V> {
        &self.right
    }

    pub fn get_left(&self) -> &NodeGeneric<K, V> {
        &self.left
    }

}

impl<K, V> From<Leaf<K, V>> for NodeGeneric<K, V>
where
    K: Serialize + Clone + Eq,
    V: Serialize + Clone,
{
    fn from(leaf: Leaf<K, V>) -> Self {
        NodeGeneric::Leaf(leaf)
    }
}


impl<K, V> From<&mut Leaf<K, V>> for NodeGeneric<K, V>
where
    K: Serialize + Clone + Eq,
    V: Serialize + Clone,
{
    fn from(leaf: &mut Leaf<K, V>) -> Self {
        NodeGeneric::Leaf(leaf.clone())
    }
}

impl<K, V> Leaf<K, V>
where
    K: Serialize + Clone + Eq,
    V: Serialize + Clone,
{
    pub fn new(key: K, value: V) -> Self {
        let my_h = Leaf::create_leaf_hash(key.clone(), value.clone());
        Leaf {
            k: key,
            v: value,
            my_hash: my_h,
        }
    }

    pub fn create_leaf_hash(key: K, value: V) -> Hash {
        let h1: Hash = hash(&key).unwrap();
        let h2: Hash = hash(&value).unwrap();
        hash(&(h1, h2)).unwrap()
    }

    pub fn get_hash(&self) -> Hash {
        Leaf::create_leaf_hash(&self.k, &self.v)
    }

    fn set_hash(&mut self, h: Hash) -> Hash {
        self.my_hash = h;
        h
    }

    fn insert(&mut self, key_to_add: K, value_to_add: V, index: u8) -> NodeGeneric<K, V> {
        if hash(&key_to_add).unwrap() == hash(&self.k).unwrap() {
            self.v = value_to_add.clone();
            self.set_hash(self.get_hash());
            return self.into();
        } else if index == 255 {
            panic!("followed the same path: different keys but same hash ---> Collision")
        }
        // this Leaf node is at depth < 255 but the key_to_add != self.k, so create a branch Internal node,
        // move the precedent Leaf node more into depth and create the Empty sibling
        let mut new_internal;
        let key_hash = hash(&self.k).unwrap();
        let direction = get_bit_direction(&key_hash.to_bytes(), index);

        if direction == true {
            new_internal = Internal::new(Empty::new().into(), self.into(), None);
        } else {
            new_internal = Internal::new(self.into(), Empty::new().into(), None);
        }
        new_internal.insert(key_to_add, value_to_add, index)
    }

    pub fn get_key(&self) -> &K {
        &self.k
    }

    pub fn get_value(&self) -> &V {
        &self.v
    }
}


impl<K, V> From<Empty> for NodeGeneric<K, V>
where
    K: Serialize + Clone + Eq,
    V: Serialize + Clone,
{
    fn from(e: Empty) -> Self {
        NodeGeneric::Empty(e)
    }
}

impl Empty {
    pub fn new() -> Self {
        Empty {}
    }

    pub fn get_hash() -> Hash {
        hash(&()).unwrap()
    }

    fn insert<K, V>(&mut self, key_to_add: K, value_to_add: V, index: u8) -> NodeGeneric<K, V>
    where
        K: Serialize + Clone + Eq,
        V: Serialize + Clone,
    {
        if index == 255 {
            panic!("followed the same path: different keys but same hash ---> Collision");
        }
        Leaf::new(key_to_add, value_to_add).into()
    }
}
