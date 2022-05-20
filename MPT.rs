use std::collections::HashMap;
use serde::{Serialize,Deserialize};

use talk::crypto::primitives::hash::Hash;
use talk::crypto::primitives::hash::hash;


/*Per trovare il path uso una funzione bit da investigare meglio: ma in generale passo sempre da dalla funzione find_path di Node,
 da quella, in base al tipo del Nodo right o left, rieseguirò la funzione find_path, fino a che non arrivo a un empty node o a una leaf
    Big HashMap contains as key the HASH OF THE NODE (as defined for Internal, Leaf and Empty) and as Value &NodeGeneric<K, V>
 */



pub fn get_bit_direction()->bool{
    
}
pub enum NodeGeneric<K, V> /*where K: Serialize, V: Serialize*/{
    Internal(Internal<K,V>),
    Leaf(Leaf<K, V>),
    Empty(Empty),
}

struct Internal<K, V> /*where K: Serialize, V: Serialize*/{
    left: Hash,
    right: Hash
}
struct Empty<K, V> {
}
struct Leaf<K, V> /*where K: Serialize, V: Serialize*/{
    k: K,
    v: V,
}

impl<K, V> NodeGeneric<K, V>{
    ///returns a Leaf node
    fn to_leaf(self)->Leaf<K,V>{
        match self{
            NodeGeneric::Leaf(n) => n,
            _ => panic!("Node which is not a Leaf!")
        }
    }

    ///returns an Internal node
    fn to_internal(self)->Internal<K,V>{
        match self{
            NodeGeneric::Internal(n) => n,
            _ => panic!("Node which is not a Internal!")
        }
    }

    
    ///returns an Empty node
    fn to_empty(self)->Empty{
        match self{
            NodeGeneric::Empty(n) => n,
            _ => panic!("Node which is not a Empty!")
        }
    }

    fn find_path(&self, key: K, value: V, index: u8) -> Result<&NodeGeneric<K, V>,()>{
        match self{
            NodeGeneric::Internal(n) => n.find_path(key, value, index),
            NodeGeneric::Leaf(n) => n.find_path(key, value),
            NodeGeneric::Empty(n) => n.find_path(),
            _ => ()
        }
    }

    fn insert(&mut self, key_to_add: K, val_to_add: V, map: HashMap<K, V>, index: u8){
        match self{
            NodeGeneric::Internal(n) => n.insert(key_to_add, val_to_add, map, index),
            NodeGeneric::Leaf(n) => n.insert(key_to_add, val_to_add, map, index),
            NodeGeneric::Empty(n) => n.insert(),
            _=> panic!("Other non defined type!")
        }
    }

    fn get_hash(self)->Hash{
        match self{
            NodeGeneric::Internal(n) => n.get_hash(),
            NodeGeneric::Leaf(n) => n.get_hash(),
            NodeGeneric::Empty(n) => n.get_hash(),
            _=> panic!("Other non defined type!")
        }
    }
}


impl<K, V> Internal<K, V> {
    fn new(l: NodeGeneric<K, V>, r: NodeGeneric<K, V>, map: &mut HashMap<K, V>) -> Self {
        let mut i = Internal {left: l.get_hash(), right: r.get_hash()};
        i
    }
    fn prove_left(&self, &mut siblings: Vec<Hash>) {
        siblings.push(self.right);
    }
    fn prove_right(&self, &mut siblings: Vec<Hash>) {
        siblings.push(self.left);
    }
    fn get_right(&self)->&NodeGeneric<K, V>{
        &self.right
    }
    fn get_left(&self)->&NodeGeneric<K, V>{
        &self.left
    }
    
    fn find_path(&mut self, key: K, value: V, index: u8) -> Result<&NodeGeneric<K, V>,()>{
        let direction = todo!();       //semplicemente la funzione bit per una determinata profondità (==per un determinato indice del hash di 256 elementi (from 0---> 255))
        if(direction==true){
            self.get_right().find_path(key, value, index);
        }
        self.get_left().find_path(key, value, index);
    }


    fn insert(&self, &mut map: HashMap<K, V>, mut path: Vec<Direction>, leaf: Leaf<K, V>) -> Hash {
        let direction = path.pop().unwrap();

        match direction {
            Direction::Left => todo!(),
            Direction::Right => todo!(),
        }
    }
    fn get_hash(&self) -> Hash {
        let final_hash = hash(&(self.left, self.right));
        final_hash
    }
}

impl<K, V> Leaf<K, V> {
    fn get_hash(&self) -> Hash {
        let h1 = hash(&self.key);
        let h2 = hash(&self.value);
        let final_hash = hash(&(h1, h2));
        
        final_hash
    }
    fn find_path(&mut self, key: K, value: V) -> Result<&NodeGeneric<K, V>,()>{
        if key == self.get_hash(){
            Ok(&self);
        }
        ();
    }
    //Returns the hash of the key just added to the MPT
    fn insert(&self, key_to_add: K, val_to_add: V, &mut map: HashMap<K, V>, index: u8) -> Hash {
        let hash_key_to_add = hash(&key_to_add); 
        if key_to_add == self.k {   //modificare valore per questa chiave nella mappa
            map.insert(hash_key_to_add, val_to_add);
            hash_key_to_add
        }
        else if index == 255{                       //collision: due chiavi diverse ma con stesso hash. Alla fine confronto le chiavi e sono diverse (ma sono giunto allo stesso nodo finale e quindi sfortunatamente hanno lo stesso hash)
            panic!("followed the same path: different keys but same hash ---> Collision")
        }
        map.insert(hash_key_to_add, val_to_add);
        hash_key_to_add
    }
}

impl<K, V> Empty<K, V>{
    fn get_hash(&self) -> Hash {
        let final_hash = hash(&String::from("Empty"));
    }
    fn find_path(&mut self) -> Result<&NodeGeneric<K, V>,()>{
        ();
    }
}


#[derive(Serialize, Deserialize)]
struct Proof {
    siblings: Vec<Sibling>,
}

struct Sibling {
    hash: Hash,
    direction: Direction,
}

enum Direction {
    Left,
    Right,
}


struct MerkleTree<K, V> where K: Serialize, V: Serialize{
    map: HashMap<Hash, NodeGeneric<K, V>>,
    root: Hash,
}

impl<K, V> MerkleTree<K, V> {
    fn insert(&mut self, key: K, value: V) {
        let path: Vec<Direction> = convert_to_path(hash(key));
        let leaf = Leaf::new(key, value);

        let root_node = self.map.get(&root).unwrap();

        self.map.insert(leaf.hash(), leaf);
        let hash = root_node.insert(path, leaf);

        self.root = hash;
    }
}

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