use std::collections::HashMap;
use std::option::Option;
//use std::vec::Vec;
/* COSA DEVO FARE:
- Creare un Merkle Tree a partire da una lista di items
- Avere il riferimento della root
- 


.........................................................


MerkleTree: */ 
//quindi ogni Nodo contiene key, value e un riferimento a una lista di nodi figli

struct Transaction{
    hash_transaction: String,
    source: String,
    destination: String,
    amount: f32
}
impl Transaction{
    fn originate(s: String, d: String, a: f32)->Transaction{
        
        //aggiungi String hash_transaction = hash(source+destination+amount);
        return Transaction{source: s, destination: d, amount: a, hash_transaction: String::from("ABCDE")};
    }
    pub fn get_source(& self)->&String{
        return &self.source;
    }
    pub fn get_destination(& self)->&String{
        return &self.destination;
    }
    pub fn get_amount(& self)->&f32{
        return &self.amount;
    }
}

trait GenNode {
    fn to_string();
}
struct Node{
    key: String,
    children: Option<HashMap<String, Node>>,
    transaction: Option<Transaction>
}

impl Node{
    fn originate(k: String, is_leaf: bool, t: Option<Transaction>)->Node{
        //return Node{key: k, children: HashMap::new(), transaction: t};
        if is_leaf{
            return Node{key: k, children: None, transaction: t};
        }
        return Node{key: k, children: Some(HashMap::<String,Node>::new()), transaction: None};
    }
    fn to_string(){
        print!("ciaoo");
    }
    pub fn get_transaction(& self)->String{
        if self.key == (String::from("DATA")){
            let res = self.transaction.as_ref().unwrap().get_source();
            //res.push_str(self.transaction.get_destination());
            //res.push_str(&self.transaction.get_amount().to_string());            
            return res.to_string()                  //String::from(self.transaction.get_source() + self.transaction.get_destination() + self.transaction.get_amount()); 
        }
        return String::from("This node is not a leaf: it doesn't contain a transaction");
    }
    fn get_children(&mut self)->&mut Option<HashMap<String, Node>>{
       return &mut self.children;
    }
    fn add_child(&mut self, k: String, new_node: Node){
        self.children.as_mut().unwrap().insert(k, new_node);
    }
}

/*struct LeafNode{
    key: String,
    transaction: Transaction
}
impl LeafNode{
    fn originate(t: Transaction)->LeafNode{
        return LeafNode{key: "DATA".into(), transaction: t};      //the label key of every LeafNode is "DATA"
    }
    /*pub fn add_child(&mut self, k: String, new_leaf_node: LeafNode){
        self.children.insert(k, new_leaf_node);
    }*/
    
    pub fn get_string_transaction(& self){

    }
}
*/
//quindi ogni Merkle Tree contiene il riferimento a un nodo root
struct MerkleTree{
    root: Node
}

impl MerkleTree {
    fn originate(k: String)->MerkleTree{
        let fake_trans = Some(Transaction::originate(String::from("aa"), String::from("aa"),12.4));//FAKE TRANSACTION: BISOGNA CONVERTIRE RETURN TYPE IN OPTION E METTERE NULL!!!
        let root = Node::originate(k, false, fake_trans);
        return MerkleTree{root}; //map_children presenta in ogni elemento come chiave un carattere, come valore un nodo 
        //si poteva scrivere equivalentemente:  return MerkleTree{root: Node{key: "DDD", children: HashMap::new()}}; 
    }
    fn add(&mut self, transaction: Transaction, hash: String){
        //rroott= Node::originate(k);
        let mut node = &mut self.root;
        let character = &hash;  //suppose key is a string (Should be the hash)
        let mut counter = 0;
        for i in character.chars(){
            //let ch = hash.chars().nth(counter).unwrap();
            let ch_string = hash.chars().nth(counter).unwrap().to_string();
            if node.get_children().as_ref().unwrap().get(&ch_string).is_none() {
                let fake_trans = Some(Transaction::originate(String::from("aa"), String::from("aa"),12.4));//FAKE TRANSACTION: BISOGNA CONVERTIRE RETURN TYPE IN OPTION E METTERE NULL!!!
                let new_node = Node::originate(ch_string.clone(), false, fake_trans);     //poi ci si dovrebbe
                node.add_child(ch_string.clone(), new_node);
            }
            node = node.get_children().as_mut().unwrap().get_mut(&ch_string).unwrap();
            counter+=counter;
        }
        //qui anche se è gia presente una Transaction, la sovrascrivo!
        if !node.get_children().as_mut().unwrap().is_empty(){
            return;
        }
        let fake_trans = Some(Transaction::originate(String::from("aa"), String::from("aa"),12.4));//FAKE TRANSACTION: BISOGNA CONVERTIRE RETURN TYPE IN OPTION E METTERE NULL!!!
        let leaf: Node = Node::originate(String::from("DATA"), true, fake_trans);
        //node.add_child(String::from("DATA"), leaf);
    }

    fn get(){
        
    }

    fn remove(){
        
    }
}

fn main(){
}