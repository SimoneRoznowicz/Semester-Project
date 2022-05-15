use crate::Node::Node;
use crate::Transaction::Transaction;
use std::option::Option;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};
//use talk::crypto::primitives::hash::Hash;
use talk::{crypto::Identity, net::SessionConnector};

#[derive(Serialize, Deserialize, Debug)]
pub struct Value{
    node: Node,
    hash: String
}
impl Value{
    fn originate_all(n: &mut Node, h: String)->Value{
        return Value{node: n, hash: h}; 
    }
    fn originate(n: &mut Node)->Value{
        return Value{node: n, hash: String::from("")}; 
    }
    pub fn get_node(&mut self)->&mut Node{
        return &mut self.node;      
    }
    pub fn get_hash(&mut self)->&mut String{
        return &mut self.hash;      
    }
    pub fn set_node(&mut self, n: Node){
        self.node = n;
    }
    pub fn set_hash(&mut self, h: String){
        self.hash = h;
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MerkleTree{
    root: Node,
    map: HashMap<String, Value>  //big HashMap containing all the serialized nodes of the MPT as values, all the hashes of the nodes as key
    //HashMap: key = id (String)    value = Node reference (&Node) and hash of the node (String)
}
impl MerkleTree {
    fn originate()->MerkleTree{
        let key_root: String = String::from("0");
        let root = Node::originate(None);
        return MerkleTree{root, map: HashMap::new()}; //map_children presenta in ogni elemento come chiave un carattere, come valore un nodo 
        //si poteva scrivere equivalentemente:  return MerkleTree{root: Node{key: "DDD", children: HashMap::new()}}; 
    }
    fn add(&mut self, transaction: Option<Transaction>){
        let my_source = transaction.as_ref().unwrap().get_source().clone();
        let mut node = &mut self.root;
        let mut counter = 0;
        let mut path = String::from("");
        //let character = &id;  //suppose key is a string (Should be the hash)
        for ch in my_source.chars(){
            //let ch_string = id.chars().nth(counter).unwrap().to_string();
            path.push_str(&ch.to_string());
            if self.map.get_mut(&path).is_none() {
                let new_node = Node::originate(None);     //poi ci si dovrebbe
                self.map.insert(path, Value::originate(new_node));  //VALUE FITTIZIO!!!!!!!!!!!!!!
            }
            node = self.map.get_mut(&path).unwrap().get_node();
            node = node.get_children().as_mut().unwrap().get_mut(&ch_string).unwrap();
        }
        //qui anche se è gia presente una Transaction, la sovrascrivo!
        if !node.get_children().as_mut().unwrap().is_empty(){
            return;
        }
        //let tran = Some(Transaction::originate(transaction.unwrap().get_source(), transaction.unwrap().get_destination(), transaction.unwrap().get_amount()));//FAKE TRANSACTION: BISOGNA CONVERTIRE RETURN TYPE IN OPTION E METTERE NULL!!!
        let leaf: Node = Node::originate(String::from("DATA"), true, transaction);
        node.add_child(String::from("DATA"), leaf);
    }

    fn get(&mut self, transaction: Option<Transaction>, id: String)->&Option<Transaction>{
        let mut node = &mut self.root;
        let mut counter = 0;
        let character = &id;  //suppose key is a string (Should be the hash)
        for i in character.chars(){
            let ch_string = id.chars().nth(counter).unwrap().to_string();
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
    fn prove(/*path to check (can be simply the id of the source whose transactions have to be checked), root(?), list of transactions(?) */){//prove existance and exclusion for a specific source: check if the path corresponds

    }
}
