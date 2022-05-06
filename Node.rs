use std::collections::HashMap;
use std::option::Option;
use serde::Serialize;
use crate::Transaction::Transaction;



pub struct Node{
    key: String,
    children: Option<HashMap<String, Node>>,
    transaction: Option<Transaction>
}

impl Node{
    pub fn originate(k: String, is_leaf: bool, t: Option<Transaction>)->Node{
        //return Node{key: k, children: HashMap::new(), transaction: t};
        if is_leaf{
            return Node{key: k, children: None, transaction: t};
        }
        return Node{key: k, children: Some(HashMap::<String,Node>::new()), transaction: None};
    }
    pub fn get_transaction(& self)->&Option<Transaction>{
        return &self.transaction;
    }
    pub fn get_string_transaction(& self)->String{
        if self.key == (String::from("DATA")){
            let res = self.transaction.as_ref().unwrap().get_source();
            //res.push_str(self.transaction.get_destination());
            //res.push_str(&self.transaction.get_amount().to_string());            
            return res.to_string()                  //String::from(self.transaction.get_source() + self.transaction.get_destination() + self.transaction.get_amount()); 
        }
        return String::from("This node is not a leaf: it doesn't contain a transaction");
    }
    pub fn get_children(&mut self)->&mut Option<HashMap<String, Node>>{
       return &mut self.children;
    }
    pub fn add_child(&mut self, k: String, new_node: Node){
        self.children.as_mut().unwrap().insert(k, new_node);
    }
}
