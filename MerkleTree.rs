use crate::Node::Node;
use crate::Transaction::Transaction;
use std::option::Option;
use std::collections::HashMap;

//use talk::crypto::primitives::hash::Hash;
use talk::{crypto::Identity, net::SessionConnector};

//use std::vec::Vec;
/* COSA DEVO FARE:
- Creare un Merkle Tree a partire da una lista di items
- Avere il riferimento della root
- 
.........................................................

MerkleTree: */ 
//quindi ogni Nodo contiene key, value e un riferimento a una lista di nodi figli

/*pub(in crate::vector) enum Nodee {
    Internal(Hash, Hash),
    Item(Hash),
}*/


//quindi ogni Merkle Tree contiene il riferimento a un nodo root
struct MerkleTree{
    root: Node,
    map: HashMap<String, String>  //big HashMap containing all the nodes of the MPT as values, all the hashes of the nodes as key
}

impl MerkleTree {
    fn originate(k: String)->MerkleTree{
        let root = Node::originate(k, false, None);
        return MerkleTree{root, map: HashMap::new()}; //map_children presenta in ogni elemento come chiave un carattere, come valore un nodo 
        //si poteva scrivere equivalentemente:  return MerkleTree{root: Node{key: "DDD", children: HashMap::new()}}; 
    }
    fn add(&mut self, transaction: Option<Transaction>, hash: String){
        let mut node = &mut self.root;
        let mut counter = 0;
        let character = &hash;  //suppose key is a string (Should be the hash)
        for i in character.chars(){
            let ch_string = hash.chars().nth(counter).unwrap().to_string();
            if node.get_children().as_ref().unwrap().get(&ch_string).is_none() {
                let new_node = Node::originate(ch_string.clone(), false, None);     //poi ci si dovrebbe
                node.add_child(ch_string.clone(), new_node);
            }
            node = node.get_children().as_mut().unwrap().get_mut(&ch_string).unwrap();
            counter+=1;
        }
        //qui anche se è gia presente una Transaction, la sovrascrivo!
        if !node.get_children().as_mut().unwrap().is_empty(){
            return;
        }
        //let tran = Some(Transaction::originate(transaction.unwrap().get_source(), transaction.unwrap().get_destination(), transaction.unwrap().get_amount()));//FAKE TRANSACTION: BISOGNA CONVERTIRE RETURN TYPE IN OPTION E METTERE NULL!!!
        let leaf: Node = Node::originate(String::from("DATA"), true, transaction);
        node.add_child(String::from("DATA"), leaf);
    }

    fn get(&mut self, transaction: Option<Transaction>, hash: String)->&Option<Transaction>{
        let mut node = &mut self.root;
        let mut counter = 0;
        let character = &hash;  //suppose key is a string (Should be the hash)
        for i in character.chars(){
            let ch_string = hash.chars().nth(counter).unwrap().to_string();
            if node.get_children().as_ref().unwrap().get(&ch_string).is_none(){
                return &None;
            }
            else{
                node = node.get_children().as_mut().unwrap().get_mut(&ch_string).unwrap();
            }
            counter+=1;
        }
        return node.get_children().as_mut().unwrap().get_mut(&String::from("DATA")).unwrap().get_transaction();    //se non trova l'elemento non so cosa restituisce qui...
    }

    fn remove(){
        //per fare la proof puoi partire da un vetttore di tramsactions che supponi di avere. Ogni nodo 
    }
}
