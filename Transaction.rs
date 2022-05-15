use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction{
    hash_transaction: String,
    source: String,
    destination: String,
    amount: f32
}
impl Transaction{
    pub fn originate(s: String, d: String, a: f32)->Transaction{
        
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