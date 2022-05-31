use serde::de::MapAccess;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, boxed::Box, vec::Vec};
//use bitvec
use talk::crypto::primitives::hash::hash;
use talk::crypto::primitives::hash::Hash;

pub fn get_bit_direction(arr: &[u8; 32], index: u8) -> bool {
    let byte = arr[(index / 8) as usize];
    let sub_index: u8 = 1 << (7 - (index % 8));
    (byte & sub_index) > 0
}

//prova a togliere Copy dappertutto. Dove lo richiede, prova a mettere .clone()

/*fn add_sibling<Q>(vec: &mut Q, elem: &Sibling) -> &mut Vec<Sibling>
where Q: Borrow<Vec<Sibling>>{
    vec.append(elem.clone());
    vec
}*/

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

    fn compute_hashes(&mut self) -> Hash {
        match self {
            NodeGeneric::Empty(n) => Empty::get_hash(),
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

    /*fn try_create<'a>() -> &'a String {
        &String::new()
    }*/

    fn get_siblings_first<Q>(&self, key: &Q, index: u8) -> Result<Vec<Sibling>, ()>
    where
        K: Borrow<Q>,
        Q: Serialize + Eq,
    {
        let mut siblings = Vec::<Sibling>::new();
        self.get_siblings(key, index, &mut siblings);
        Ok(siblings)
    }

    fn get_siblings<Q>(&self, key: &Q, index: u8, siblings: &mut Vec<Sibling>)
    where
        K: Borrow<Q>,
        Q: Serialize + Eq,
    {
        match &self {
            NodeGeneric::Internal(n) => n.get_siblings(key, index, siblings),
            NodeGeneric::Leaf(n) => (),
            NodeGeneric::Empty(n) => panic!(),
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
        let direction = get_bit_direction(&key_hash.to_bytes(), index); //semplicemente la funzione bit per una determinata profondità (== per un determinato indice del hash di 256 elementi (from 0---> 255))
        if direction == true {
            let l_node = self.get_left();
            match l_node {
                NodeGeneric::Internal(n) => {
                    siblings.insert(0, Sibling::new(n.my_hash.unwrap(), Left {}.into()))
                }
                NodeGeneric::Leaf(n) => siblings.insert(0, Sibling::new(n.my_hash, Left {}.into())),
                NodeGeneric::Empty(n) => {
                    siblings.insert(0, Sibling::new(Empty::get_hash(), Left {}.into()))
                }
            }
            self.get_right().get_siblings(key, index + 1, siblings)
        } else {
            let r_node = self.get_right();
            match r_node {
                NodeGeneric::Internal(n) => {
                    siblings.insert(0, Sibling::new(n.my_hash.unwrap(), Left {}.into()))
                }
                NodeGeneric::Leaf(n) => siblings.insert(0, Sibling::new(n.my_hash, Left {}.into())),
                NodeGeneric::Empty(n) => {
                    siblings.insert(0, Sibling::new(Empty::get_hash(), Left {}.into()))
                }
            }
            self.get_left().get_siblings(key, index + 1, siblings)
        }
    }
    //Returns the hash of the node calling the method, asssigns the hash value to every Internal node.
    //This method is to be called when the MPT is complete.
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

    fn find_path<Q: ?Sized>(&self, key: &Q, index: u8) -> Result<&NodeGeneric<K, V>, ()>
    where
        K: Borrow<Q>,
        Q: Serialize + Eq,
    {
        let key_hash = hash(&key).unwrap();
        let direction = get_bit_direction(&key_hash.to_bytes(), index); //semplicemente la funzione bit per una determinata profondità (== per un determinato indice del hash di 256 elementi (from 0---> 255))

        if direction == true {
            self.get_right().find_path(key, index)
        } else {
            self.get_left().find_path(key, index)
        }
    }

    fn insert(&mut self, key_to_add: K, value_to_add: V, index: u8) -> NodeGeneric<K, V> {
        let key_hash = hash(&key_to_add).unwrap();
        let direction = get_bit_direction(&key_hash.to_bytes(), index); //semplicemente la funzione bit per una determinata profondità (== per un determinato indice del hash di 256 elementi (from 0---> 255))

        let mut side;
        if direction == true {
            //correspond to a 1 found at this index: right, true, 1
            side = &mut self.right;
        } else {
            //correspond to a 0 found at this index: left, false, 0.
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
    //NON SO SE SIA GIUSTO QUESTO SOTTO
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

    fn insert(&mut self, key_to_add: K, value_to_add: V, index: u8) -> NodeGeneric<K, V> {
        //let hash_key_to_add = hash(&key_to_add);
        if key_to_add == self.k {
            //substitute the value of this Leaf node
            self.v = value_to_add.clone();
        } else if index == 255 {
            //collision: due chiavi diverse ma con stesso hash. Alla fine confronto le chiavi e sono diverse (ma sono giunto allo stesso nodo finale e quindi sfortunatamente hanno lo stesso hash)
            panic!("followed the same path: different keys but same hash ---> Collision")
        }
        //this Leaf node is at depth<255, so insert an Internal node and two children: one is the current node, the other is a new empty Leaf node
        let mut new_internal;
        let key_hash = hash(&self.k).unwrap();
        let direction = get_bit_direction(&key_hash.to_bytes(), index); //semplicemente la funzione bit per una determinata profondità (== per un determinato indice del hash di 256 elementi (from 0---> 255))

        if direction == true {
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

#[derive(Serialize, Deserialize)]
enum Direction {
    Left,
    Right,
}

#[derive(Serialize, Deserialize)]
struct Left {}
impl Left {
    fn get_val() -> bool {
        false
    }
}

#[derive(Serialize, Deserialize)]
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
    fn new(r: NodeGeneric<K, V>) -> MerkleTree<K, V> {
        let r = Box::new(r);
        MerkleTree { root: r }
    }

    fn insert(&mut self, key_to_add: K, value_to_add: V) -> NodeGeneric<K, V> {
        self.root.insert(key_to_add, value_to_add, 0)
    }

    fn get_value(&self, key: &K) -> Result<&NodeGeneric<K, V>, ()> {
        self.root.find_path(key, 0)
    }
}

impl<K, V> MerkleTree<K, V>
where
    K: Serialize + Clone + Eq,
    V: Serialize + Clone,
{
    //Returns a Proof for a specific leaf (== specific source of payment)

    pub fn prove(&self, key: &K) -> Proof {
        Proof::new(self.root.get_siblings_first(key, 0).unwrap())
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
    fn get_key(&self) -> &K {
        &self.key
    }
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

//devi fare questo metodo che verifica e uno prima che appartiene alla struct MPT che genera la Proof (collezionando quindi i siblings necessari con il metodo simile a get che diceva)

//Returns the hash of the root
//For each (client, transaction) in the batch, broker creates a proof from MT for (client, transaction)
//method performed by the client

//my_transaction is a vector of the transactions
fn get_root_hash<T, K>(proof: Proof, my_transaction: T, id: Id<K>) -> Hash
where
    T: Serialize + Clone,
    K: Serialize + Eq + Clone,
{
    let siblings = proof.get_siblings();
    let my_leaf = Leaf::<K, T>::new(id.key, my_transaction);

    //se considero il vettore contenere None values per le mpty leaves, posso iterare siblings e sostituire il valore con il hash di Empty

    //immagino di avere un vector ordinato in base agli hash che vedo prima
    //initially I have just the hash of the leaf
    let mut hash_final = my_leaf.get_hash();

    for sibling in siblings {
        match sibling.direction {
            //Direction::Left indicates that the sibling is on the Left
            Direction::Left => hash_final = hash(&(sibling.hash, hash_final)).unwrap(),
            //Direction::Right indicates that the sibling is on the right
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
        let left_empty = Empty::new().into();
        //let left = Leaf::<&str, u8>::new("left", 55).into();
        let right_empty = Empty::new().into();

        //let right = Leaf::<&str, u8>::new("right", 77).into();
        let mut root: NodeGeneric<&str, i32> = Internal::new(left_empty, right_empty, None).into();

        root.insert("ciao", 55, 0);

        match root.find_path("ciao", 0).unwrap() {
            NodeGeneric::Leaf(n) => assert_eq!(n.v, 55),
            _ => assert!(false),
        }
    }

    #[test]
    fn find_path_insert_test2() {
        //let left_empty = Empty::new().into();
        let kk = "left";
        let gg = hash(&kk).unwrap();
        let left = Leaf::<&str, u8>::new("Hello", 55).into();
        let right_empty = Empty::new().into();

        //let right = Leaf::<&str, u8>::new("right", 77).into();
        let mut root: NodeGeneric<&str, u8> = Internal::new(left, right_empty, None).into();

        root.insert("ciao", 55, 0);

        match root.find_path("ciao", 0).unwrap() {
            NodeGeneric::Leaf(n) => assert_eq!(n.v, 55),
            _ => assert!(false),
        }
    }
}
