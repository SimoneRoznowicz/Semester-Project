use std::option::Option;
use serde::{Serialize,Deserialize};
use crate::Transaction::Transaction;


#[derive(Serialize, Deserialize, Debug)]
pub struct Node{
    //key: String,
    //value: Option<String>, //nodo serialized e reso stringa (forse posso togliere?)
    transactions: Option<Vec<Transaction>>,
    //left: Option<char>,       non mi serve left e right perchè in base all'id che devo raggiungere, prendo l'id (key) del nodo che vglio raggiungere e ci aggiungo un carattere
    //ottengo la key del nuovo nodo che voglio raggiungere. Quindi interpello la hashmap con la key ottenuta. In realtà anche key mi sembra inutile perchè l'informazione è gia
    //contenuta nella mappa.
    //right: Option<char>
}

impl Node{
    pub fn originate(ts: Option<Vec<Transaction>>)->Node{
        return Node{transactions: ts};
    }
    pub fn get_transactions(& self)->&Option<Vec<Transaction>>{
        return &self.transactions;
    }
    pub fn get_string_transaction(t: Option<Transaction>)->String{
        if t.is_none(){
            return String::from("Transaction is None");
        }
        let mut s = t.as_ref().unwrap().get_source().clone();
        let mut d = t.as_ref().unwrap().get_destination();
        let mut a = t.as_ref().unwrap().get_amount().to_string();

        s.push_str(d);
        s.push_str(&a);
        return s.to_string();  //return a string containing s+d+a (source+destination+amount)
    }
    pub fn add_transaction(){

    }
}
