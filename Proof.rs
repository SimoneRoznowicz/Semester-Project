use crate::Node::Node;
use crate::Transaction::Transaction;
use serde::{Serialize,Deserialize};
use std::collections::HashSet;
use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]
pub struct Proof{     //PROOF FOR A SPECIFIC SOURCE
    path: String,       //represents the expected id used to get to the transaction 
    //transaction: Vec<Transaction>,
    vec: Vec<String>      //serialized nodes containing the list of transactions OF THE SAME SPECIFIED SOURCE //lista data dal BROKER
}
impl Proof{
    pub fn originate(p: String, v: Vec<String>)->Proof{
        return Proof{path: p, vec: v};
    }
    pub fn prove(&mut self,id: String, client_set: HashSet<String>)->bool{       //Questo è l'id del client giusto (viene fornito infatti dal client)
        if !self.path.eq(&id) {
            return false;
        }
        for elem in &self.vec {
            if !client_set.contains(elem){
                return false;
            }   
        }
        return true;
        
        //so the paths are the same: now I need to check the proof for every transaction for the given source id
        //ASSUMPION: 
        //immagina che il client abbia a disposizione un hashSet contentente il valore di hash di ogni transazione che vuole performare.
        //poi immagina che la variabile set, contenuta in Proof, contenga i hash delle transazioni.
        /*Quindi: controlla che ogni hash del set di transazioni, passato dal broker, sia contenuto nel sset di transazioni da performare del client.  */
    }
}