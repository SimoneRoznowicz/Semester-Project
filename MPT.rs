use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, boxed::Box};

use talk::crypto::primitives::hash::hash;
use talk::crypto::primitives::hash::Hash;

pub fn get_bit_direction() -> bool {
    todo!()
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
    my_hash: Option<Hash>, //initialized to Node, change to the value at the end (the leaves can be added immediately)
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Empty {}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Leaf<K, V>
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
    V: Serialize + Clone + Copy,
{
    fn compute_hashes(&mut self) -> Hash {
        match self {
            NodeGeneric::Empty(n) => n.get_hash(),
            NodeGeneric::Leaf(n) => n.my_hash,
            NodeGeneric::Internal(n) => n.compute_hashes(),
            //posso pensare anche di non mandare il vettore con tutti i hash dei nodi empty,
            //(posso ad esempio sostituire quei valori con None)
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

    fn find_path<Q>(&self, key: &Q, value: V, index: u8) -> Result<&NodeGeneric<K, V>, ()>
    where
        K: Borrow<Q>,
        Q: Serialize + Eq,
    {
        match &self {
            n if &n.get_hash() == &hash(&key).unwrap() => Ok(&self),
            NodeGeneric::Internal(n) => n.find_path(key, value, index),
            //NodeGeneric::Leaf(n) => n.find_path(key),
            _ => Err(()),
        }
    }

    fn insert(self, key_to_add: K, value_to_add: V, index: u8) -> NodeGeneric<K, V> {
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
            NodeGeneric::Empty(n) => n.get_hash(),
        }
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
    V: Serialize + Clone + Copy,
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
    /*fn prove_left(&self, &mut siblings: Vec<Hash>) {
        siblings.push(self.right);
    }
    fn prove_right(&self, &mut siblings: Vec<Hash>) {
        siblings.push(self.left);
    }*/
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

    fn find_path<Q>(&self, key: &Q, value: V, index: u8) -> Result<&NodeGeneric<K, V>, ()>
    where
        K: Borrow<Q>,
        Q: Serialize + Eq,
    {
        let direction = get_bit_direction(); //semplicemente la funzione bit per una determinata profondità (== per un determinato indice del hash di 256 elementi (from 0---> 255))
        if direction == true {
            self.get_right().find_path(key, value, index)
        } else {
            self.get_left().find_path(key, value, index)
        }
    }

    fn insert(mut self, key_to_add: K, value_to_add: V, index: u8) -> NodeGeneric<K, V> {
        let next_index = index + 1;
        if get_bit_direction() == true {
            //correspond to a 1 found at this index: right, true, 1
            self.right.insert(key_to_add, value_to_add, next_index)
        } else {
            //correspond to a 0 found at this index: left, false, 0
            self.left.insert(key_to_add, value_to_add, next_index)
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

impl<K, V> Leaf<K, V>
where
    K: Serialize + Clone + Eq,
    V: Serialize + Clone + Copy,
{
    fn new(key: K, value: V) -> Self {
        let my_h = Leaf::create_leaf_hash(&key, &value);
        Leaf {
            k: key,
            v: value,
            my_hash: my_h,
        }
    }

    fn compute_hashes(&self) -> Hash {
        self.my_hash
    }
    fn create_leaf_hash(key: K, value: V) -> Hash {
        let h1: Hash = hash(&key).unwrap();
        let h2: Hash = hash(&value).unwrap();
        hash(&(h1, h2)).unwrap()
    }

    fn get_hash(&self) -> Hash {
        Leaf::create_leaf_hash(&self.k, &self.v)
    }

    fn insert(mut self, key_to_add: K, value_to_add: V, index: u8) -> NodeGeneric<K, V> {
        //let hash_key_to_add = hash(&key_to_add);
        if key_to_add == self.k {
            //substitute the value of this Leaf node
            self.v = value_to_add;
        } else if index == 255 {
            //collision: due chiavi diverse ma con stesso hash. Alla fine confronto le chiavi e sono diverse (ma sono giunto allo stesso nodo finale e quindi sfortunatamente hanno lo stesso hash)
            panic!("followed the same path: different keys but same hash ---> Collision")
        }
        //this Leaf node is at depth<255, so insert an Internal node and two children: one is the current node, the other is a new empty Leaf node
        let new_internal;
        if get_bit_direction(/*self.k---> devi solo spostare la foglia già formata che appartiene a un'altra source e contiene altre transactions */)
            == true
        {
            new_internal = Internal::new(Empty::new().into(), self.into(), None);
        } else {
            new_internal = Internal::new(self.into(), Empty::new().into(), None);
        }
        new_internal.insert(key_to_add, value_to_add, index + 1)
    }
}

impl<K, V> From<Empty> for NodeGeneric<K, V>
where
    K: Serialize + Clone + Eq,
    V: Serialize + Clone,
{
    fn from(empty: Empty) -> Self {
        NodeGeneric::Empty(empty)
    }
}
impl Empty {
    fn new() -> Self {
        Empty {}
    }
    fn get_hash(&self) -> Hash {
        hash(&()).unwrap()
    }

    fn insert<K, V>(mut self, key_to_add: K, value_to_add: V, index: u8) -> NodeGeneric<K, V>
    where
        K: Serialize + Clone + Eq,
        V: Serialize + Clone + Copy,
    {
        if index == 255 {
            panic!("followed the same path: different keys but same hash ---> Collision");
        }
        Leaf::new(key_to_add, value_to_add).into()
    }
}

#[derive(Serialize, Deserialize)]
struct Proof {
    siblings: Vec<Sibling>,
}

#[derive(Serialize, Deserialize)]
struct Sibling {
    hash: Hash,
    direction: Direction,
}

#[derive(Serialize, Deserialize)]
enum Direction {
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
struct MerkleTree {
    //map: HashMap<Hash, NodeGeneric<K, V>>,
    root: Hash,
}
/*
impl<K, V> MerkleTree
where K: Serialize + Clone + Eq, V: Serialize + Clone{
    fn insert(&mut self, key: K, value: V) {
        todo!()
        /*let path: Vec<Direction> = convert_to_path(hash(key));
        let leaf = Leaf::new(key, value);

        let root_node = self.map.get(&root).unwrap();

        self.map.insert(leaf.hash(), leaf);
        let hash = root_node.insert(path, leaf);

        self.root = hash;*/
    }
}*/
/*
impl<K, V> MerkleTree<K, V> {
    pub fn prove(&self, key: &K) {
        let path = hash(key);

        let node: NodeGeneric = self.map.get(&self.root);

        let hash = hash(&key);

        let vector: Vec<Hash> = Vec::new();

        let result: Vec<Hash> = node.prove(key, vector);
    }
}

fn get_root_so_i_can_sign_it_later(proof: Proof, my_transaction: T, my_id: Id) -> Hash {
    let path = proof.path;
    let siblings = proof.siblings;

    let leaf = Leaf::from(my_id, my_transaction);
    let hash = leaf.hash();

    for sibling in siblings {
        hash = if sibling.1 == Direction::Left {
            let internal = Internal { left: sibling.0, right: hash };
            internal.hash()
        } else {
            let internal = Internal { left: sibling.0, right: hash };
            internal.hash()
        }
    }
    return hash;
}
*/
