use crate::Node::Node;
use crate::Transaction::Transaction;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Value{
    node: Node,
    hash: String
}
impl Value{
    pub fn originate_all(n: Node, h: String)->Value{
        return Value{node: n, hash: h}; 
    }
    pub fn originate(n: Node)->Value{
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