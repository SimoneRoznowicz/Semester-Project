use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, boxed::Box, vec::Vec};
use talk::crypto::primitives::hash::hash;
use talk::crypto::primitives::hash::Hash;

/**
 * Introduction to the project:
 * 
 * In this representation of the Merkle Patricia Tree,
 * • true <--> 1 <--> Right
 * • false <--> 0 <--> Left
 *
**/

///Given an index representing the depth of a node in the Merkle Patricia Tree, returns true if the bit is 1,
///false if the bit is 0 (the array of u8 contains 256 total bits).
pub fn get_bit_direction(arr: &[u8; 32], index: u8) -> bool {
    let byte = arr[(index / 8) as usize];
    let sub_index: u8 = 1 << (7 - (index % 8));
    (byte & sub_index) > 0
}

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
    fn new() -> Self {
        Self::Empty(Empty::new())
    }
    fn new_internal_default() -> Self {
        Self::Internal(Internal::new(NodeGeneric::new(), NodeGeneric::new(), None))
    }

    fn compute_hashes(&mut self) -> Hash {
        match self {
            NodeGeneric::Empty(n) => Empty::get_hash(),
            NodeGeneric::Leaf(n) => n.my_hash,
            NodeGeneric::Internal(n) => n.compute_hashes(),
        }
    }

    ///returns a Leaf node
    fn to_leaf(self) -> Leaf<K, V> {
        match self {
            NodeGeneric::Leaf(n) => n,
            _ => panic!("Node which is not a Leaf!"),
        }
    }

    ///returns an Internal node
    fn to_internal(self) -> Internal<K, V> {
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

    fn get_siblings<Q>(&self, key: &Q, index: u8, siblings: &mut Vec<Sibling>)
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

    fn find_path<Q: ?Sized>(&self, key: &Q, index: u8) -> Result<&NodeGeneric<K, V>, ()>
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

    fn insert(&mut self, key_to_add: K, value_to_add: V, index: u8) -> NodeGeneric<K, V> {
        match self {
            NodeGeneric::Internal(n) => n.insert(key_to_add, value_to_add, index),
            NodeGeneric::Leaf(n) => n.insert(key_to_add, value_to_add, index),
            NodeGeneric::Empty(n) => n.insert(key_to_add, value_to_add, index),
        }
    }

    fn get_hash(&self) -> Hash {
        match self {
            NodeGeneric::Internal(n) => n.get_hash(),
            NodeGeneric::Leaf(n) => n.get_hash(),
            NodeGeneric::Empty(_) => Empty::get_hash(),
        }
    }

    fn hash(&self) -> Hash {
        match self {
            NodeGeneric::Internal(n) => n.get_hash(),
            NodeGeneric::Leaf(n) => n.get_hash(),
            NodeGeneric::Empty(_) => Empty::get_hash(),
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
    fn new(l: NodeGeneric<K, V>, r: NodeGeneric<K, V>, h: Option<Hash>) -> Self {
        Internal {
            left: Box::new(l),
            right: Box::new(r),
            my_hash: h,
        }
    }

    fn set_hash(&mut self, h: Option<Hash>) -> Hash {
        self.my_hash = h;
        h.unwrap()
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

    fn get_mut_right(&mut self) -> &mut NodeGeneric<K, V> {
        &mut self.right
    }

    fn get_mut_left(&mut self) -> &mut NodeGeneric<K, V> {
        &mut self.left
    }

    fn get_right(&self) -> &NodeGeneric<K, V> {
        &self.right
    }

    fn get_left(&self) -> &NodeGeneric<K, V> {
        &self.left
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

    fn create_hash(l_hash: Hash, r_hash: Hash) -> Hash {
        hash(&(l_hash, r_hash)).unwrap()
    }

    fn get_hash(&self) -> Hash {
        Internal::<K, V>::create_hash(self.left.get_hash(), self.right.get_hash())
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
    fn new(key: K, value: V) -> Self {
        let my_h = Leaf::create_leaf_hash(key.clone(), value.clone());
        Leaf {
            k: key,
            v: value,
            my_hash: my_h,
        }
    }

    fn set_hash(&mut self, h: Hash) -> Hash {
        self.my_hash = h;
        h
    }

    fn create_leaf_hash(key: K, value: V) -> Hash {
        let h1: Hash = hash(&key).unwrap();
        let h2: Hash = hash(&value).unwrap();
        hash(&(h1, h2)).unwrap()
    }

    fn get_hash(&self) -> Hash {
        Leaf::create_leaf_hash(&self.k, &self.v)
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
    fn new() -> Self {
        Empty {}
    }

    fn get_hash() -> Hash {
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Left {}
impl Left {
    fn get_val() -> bool {
        false
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Right {}

impl Right {
    fn get_val() -> bool {
        true
    }
}

impl From<Left> for Direction {
    fn from(left: Left) -> Self {
        Direction::Left
    }
}

impl From<Right> for Direction {
    fn from(left: Right) -> Self {
        Direction::Right
    }
}
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
struct MerkleTree<K, V>
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
    fn new() -> MerkleTree<K, V> {
        let n = Box::new(NodeGeneric::new_internal_default());
        MerkleTree { root: n }
    }

    fn insert(&mut self, key_to_add: K, value_to_add: V) -> NodeGeneric<K, V> {
        self.root.insert(key_to_add, value_to_add, 0)
    }

    fn get_node(&self, key: K) -> Result<&NodeGeneric<K, V>, ()> {
        self.root.find_path(&key, 0)
    }

    fn get_value(&self, key: K) -> V {
        match self.get_node(key).unwrap() {
            NodeGeneric::Leaf(n) => n.v.clone(),
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

        let mut node_err = match node_err {
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

#[derive(Serialize, Deserialize, Clone)]
struct Id<K>
where
    K: Serialize,
{
    key: K,
}

impl<K> Id<K>
where
    K: Serialize,
{
    fn new(k: K) -> Self {
        Id { key: k }
    }

    fn get_key(&self) -> &K {
        &self.key
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Sibling {
    hash: Hash,
    direction: Direction,
}
impl Sibling {
    fn new(h: Hash, d: Direction) -> Sibling {
        Sibling {
            hash: h,
            direction: d,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Proof {
    siblings: Vec<Sibling>,
}

impl Proof {
    pub fn new(s: Vec<Sibling>) -> Proof {
        Proof { siblings: s }
    }
    pub fn get_siblings(&self) -> &Vec<Sibling> {
        &self.siblings
    }
}

/// Returns the hash of the root
fn get_root_hash<T, K>(proof: Proof, my_transactions: T, id: Id<K>) -> Hash
where
    T: Serialize + Clone,
    K: Serialize + Eq + Clone,
{
    let siblings = proof.get_siblings();
    let my_leaf = Leaf::<K, T>::new(id.key, my_transactions);

    let mut hash_final = my_leaf.get_hash();

    for sibling in siblings {
        match sibling.direction {
            Direction::Left => hash_final = hash(&(sibling.hash, hash_final)).unwrap(),
            Direction::Right => hash_final = hash(&(hash_final, sibling.hash)).unwrap(),
        }
    }
    hash_final
}

/*******************************************************************************************************/

#[cfg(test)]
mod tests {
    use std::path;

    use super::*;
    // CONSTRUCTOR TESTS

    #[test]
    fn leaf_new() {
        let l = Leaf::<&str, u8>::new("key", 5);
        assert_eq!(l.k, "key");
        assert_eq!(l.v, 5);
    }

    #[test]
    fn Internal_new() {
        let left_child = Leaf::<&str, u8>::new("left", 5).into();
        let right_child = Leaf::<&str, u8>::new("right", 7).into();
        let i = Internal::new(left_child, right_child, None);

        match (i.get_left(), i.get_right()) {
            (NodeGeneric::Leaf(l), NodeGeneric::Leaf(r)) => {
                assert_eq!(l.k, "left");
                assert_eq!(l.v, 5);
                assert_eq!(r.k, "right");
                assert_eq!(r.v, 7);
            }
            _ => panic!("Error"),
        };
    }

    #[test]
    fn Empty_get_hash() {
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
            NodeGeneric::Leaf(n) => assert_eq!(n.v, 55),
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
            NodeGeneric::Leaf(n) => assert_eq!(n.v, 55),
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
            NodeGeneric::Leaf(n) => assert_eq!(n.v, 123),
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
            NodeGeneric::Leaf(n) => assert_eq!(n.v, 1),
            _ => assert!(false),
        }
        match root.find_path("AAAAA", 0).unwrap() {
            NodeGeneric::Leaf(n) => assert_eq!(n.v, 2),
            _ => assert!(false),
        }
        match root.find_path("EEEEE", 0).unwrap() {
            NodeGeneric::Leaf(n) => assert_eq!(n.v, 6),
            _ => assert!(false),
        }
        match root.find_path("CCCCC", 0).unwrap() {
            NodeGeneric::Leaf(n) => assert_eq!(n.v, 4),
            _ => assert!(false),
        }
        match root.find_path("GGGGG", 0).unwrap() {
            NodeGeneric::Leaf(n) => assert_eq!(n.v, 8),
            _ => assert!(false),
        }
    }

    #[test]

    fn MerkleTree_new() {
        //a new Merkle Patricia Tree has a root which matches an Internal node, whose left
        //and right children match Empty nodes
        let mpt: MerkleTree<&str, u8> = MerkleTree::new();
        match *mpt.root {
            NodeGeneric::Internal(n) => {
                match *n.left {
                    NodeGeneric::Empty(_) => (),
                    _ => assert!(false),
                }
                match *n.right {
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
            NodeGeneric::Leaf(n) => assert_eq!(n.v, 2),
            _ => assert!(false),
        }
        assert_eq!(mpt.get_value("AAAAA"), 2);
        match mpt.get_node("EEEEE").unwrap() {
            NodeGeneric::Leaf(n) => assert_eq!(n.v, 6),
            _ => assert!(false),
        }
        assert_eq!(mpt.get_value("EEEEE"), 6);
        assert_eq!(mpt.get_value("HHHHH"), 1);
        assert_eq!(mpt.get_value("BBBBB"), 3);
        assert_eq!(mpt.get_value("FFFFF"), 7);
        assert_eq!(mpt.get_value("CCCCC"), 4);
        assert_eq!(mpt.get_value("GGGGG"), 8);
        assert_eq!(mpt.get_value("DDDDD"), 5);

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

        assert_eq!(mpt.root.compute_hashes(), hash_root);

        let proofHello = mpt.prove("Hello");
        let dir0 = &proofHello.siblings.get(0).unwrap().direction;
        let dir1 = &proofHello.siblings.get(1).unwrap().direction;

        match dir0 {
            Direction::Left => assert!(true),
            Direction::Right => assert!(false),
        }
        match dir1 {
            Direction::Left => assert!(false),
            Direction::Right => assert!(true),
        }
        assert_eq!(proofHello.siblings.len(), 2);

        let proofciao = mpt.prove("ciao");
        let dir0 = &proofciao.siblings.get(0).unwrap().direction;
        let dir1 = &proofciao.siblings.get(1).unwrap().direction;

        match dir0 {
            Direction::Left => assert!(false),
            Direction::Right => assert!(true),
        }
        match dir1 {
            Direction::Left => assert!(false),
            Direction::Right => assert!(true),
        }
        assert_eq!(proofciao.siblings.len(), 2);
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
        for sib in &proofHHHHH.siblings {
            print!("\n\nsib == {:?}", sib);
        }
        print!("\n\n\nAAAAA");
        let proofAAAAA = mpt.prove("AAAAA");
        for sib in &proofAAAAA.siblings {
            print!("\n\nsib == {:?}", sib);
        }
        print!("\n\n\nBBBBB");

        let proofBBBBB = mpt.prove("BBBBB");
        for sib in &proofBBBBB.siblings {
            print!("\n\nsib == {:?}", sib);
        }
        print!("\n\n\nCCCCC");

        let proofCCCCC = mpt.prove("CCCCC");
        for sib in &proofCCCCC.siblings {
            print!("\n\nsib == {:?}", sib);
        }
        print!("\n\n\nDDDDD");

        let proofDDDDD = mpt.prove("DDDDD");
        for sib in &proofDDDDD.siblings {
            print!("\n\nsib == {:?}", sib);
        }
        print!("\n\n\nEEEEE");

        let proofEEEEE = mpt.prove("EEEEE");
        for sib in &proofEEEEE.siblings {
            print!("\n\nsib == {:?}", sib);
        }
        print!("\n\n\nFFFFF");

        let proofFFFFF = mpt.prove("FFFFF");
        for sib in &proofFFFFF.siblings {
            print!("\n\nsib == {:?}", sib);
        }
        print!("\n\n\nGGGGG");

        let proofGGGGG = mpt.prove("GGGGG");
        for sib in &proofGGGGG.siblings {
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

        let mut hash_root = mpt.root.get_hash();
        mpt.compute_hashes();

        //"Hello" key does not exist in the mpt so expect an empty vector siblings
        let mut proof_nothing = mpt.prove("Hello"); 
        assert_eq!(proof_nothing.siblings.len(), 0);

        let mut proof = mpt.prove("GGGGG");
        let reconstructed_hash_root = get_root_hash(proof, 8u8, Id::new("GGGGG"));

        match *mpt.root {
            NodeGeneric::Internal(n) => {
                assert_eq!(n.my_hash.unwrap(), hash_root);
                print!("FIRST ASSERT PASSATO\n\n");
                assert_eq!(hash_root, reconstructed_hash_root);
                print!("SECOND ASSERT PASSED\n\n");
            }
            _ => assert!(false),
        }
    }
}
