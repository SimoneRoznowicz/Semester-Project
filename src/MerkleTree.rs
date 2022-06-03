use crate::Node::Node;
use crate::Transaction::Transaction;
use crate::Value::Value;

use std::option::Option;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};
//use talk::crypto::primitives::hash::Hash;
use talk::{crypto::Identity, net::SessionConnector};

/*da fare: - usare serialize e deserialize per mandare le transactions nella rete e riceverle in proof
           - fare classe Proof che effettua una prove per una source: dato root
           - in add devi aggiungerci una risalita dell'albero riscorsiva in modo tale che ogni nodo presenti in Value il hash derivato da tutti i nodi figli 
*/


#[derive(Serialize, Deserialize, Debug)]
struct MerkleTree{
    root: Node,
    map: HashMap<String, Value>  //big HashMap containing all the serialized nodes of the MPT as values, all the hashes of the nodes as key
    //HashMap: key = id (String)    value = {Node reference (&Node)     ,     hash of the node (String)}
}
impl MerkleTree {
    fn originate()->MerkleTree{
        let key_root: String = String::from("0");
        let root = Node::originate(None);
        return MerkleTree{root, map: HashMap::new()}; //map_children presenta in ogni elemento come chiave un carattere, come valore un nodo 
        //si poteva scrivere equivalentemente:  return MerkleTree{root: Node{key: "DDD", children: HashMap::new()}}; 
    }
    fn add(&mut self, ts: Option<Vec<Transaction>>){
        let my_source = ts.as_ref().unwrap().get(0).unwrap().get_source().clone();
        let mut node = &mut self.root;
        let mut path = String::from("");

        for ch in my_source.chars(){
            path.push_str(&ch.to_string());
            if self.map.get_mut(&path).is_none() {
                let mut new_node;
                if path.eq(&my_source){
                    new_node = Node::originate(ts);     //poi ci si dovrebbe
                    break;
                }
                else{
                    new_node = Node::originate(None);     //poi ci si dovrebbe
                }
                self.map.insert(path.clone(), Value::originate(new_node));  
            }
            node = self.map.get_mut(&path).unwrap().get_node();
        }
    }

    fn get(&mut self, transaction: Option<Transaction>, id: String)->&Option<Transaction>{
        return &None;
        /*let mut node = &mut self.root;
        let mut counter = 0;
        //let character = &id;  //suppose key is a string (Should be the hash)
        for i in character.chars(){
            //let ch_string = id.chars().nth(counter).unwrap().to_string();
            if node.get_children().as_ref().unwrap().get(&ch_string).is_none(){
                return &None;
            }
            else{
                node = node.get_children().as_mut().unwrap().get_mut(&ch_string).unwrap();
            }
            counter+=1;
        }
        return node.get_children().as_mut().unwrap().get_mut(&String::from("DATA")).unwrap().get_transaction();*/    //se non trova l'elemento non so cosa restituisce qui...
    }

    fn remove(){
        //per fare la proof puoi partire da un vetttore di tramsactions che supponi di avere. Ogni nodo 
    }
}
