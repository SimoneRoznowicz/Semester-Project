use std::collections::HashMap;
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
    fn originate(&mut self, s: String, d: String, a: f32)->Transaction{
        
        //aggiungi String hash_transaction = hash(source+destination+amount);
        return Transaction{source: s, destination: d, amount: a, hash_transaction: String::from("ABCDE")};
    }
}

struct Node{
    key: String,
    //value: String,    //Transaction: per ora supponiamo sia un value
    children: HashMap<String, Node>
}
impl Node{
    pub fn originate(k: String)->Node{
        return Node{key: k, children: HashMap::new()};
    }
    fn to_String(){
        print!("ciaoo");
    }
    pub fn get_children(&mut self)->&mut HashMap<String,Node>{
       return &mut self.children;
    }
}
struct LeafNode{
    key: String,
    data_val: Transaction
}
impl LeafNode{
    fn originate(t: Transaction)->LeafNode{
        return LeafNode{key: "DATA".into(), data_val: t};      //the label key of every LeafNode is "DATA"
    }
}

//quindi ogni Merkle Tree contiene il riferimento a un nodo root
struct MerkleTree{
    root: Node
}

impl MerkleTree {
    fn originate(k: String)->MerkleTree{
        //let mut map_children = HashMap::new();
        //root::k = k;
        //root::ch = v;
        
        //root = Node::originate(k);
        let mut root = Node::originate("DDDD".into());
        return MerkleTree{root}; //map_children presenta in ogni elemento come chiave un carattere, come valore un nodo 
        //si poteva scrivere equivalentemente:  return MerkleTree{root: Node{key: "DDD", children: HashMap::new()}}; 
    }
    fn add(&mut self, hash: String){
        //rroott= Node::originate(k);
        let mut node = &mut self.root;
        let mut character = &hash;  //suppose key is a string (Should be the hash)
        let mut counter = 0;
        for i in character.chars(){
            let ch = hash.chars().nth(counter).unwrap();
            let ch_String = ch.clone().to_string();
            if node.get_children().get(&ch_String).is_none() {
                let mut nnode = Node::originate(ch_String.clone());
            }
            else{
                //
            }
            node = node.get_children().get_mut(&ch_String.clone()).unwrap();
            counter+=counter;
        }
        
    }

    fn get(){
        
    }

    fn remove(){
        
    }
}

fn main(){
}