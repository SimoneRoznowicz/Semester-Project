use std::collections::HashMap;
use std::option::Option;
use serde::{Serialize, Deserialize};
use serde_json::json;

mod MerkleTree;
mod Node;
mod Transaction;
mod Value;
mod Proof;

//#[derive(Serialize, Deserialize, Debug)]
fn main(){
    print!("\nHey I'm main.rs file!\n\n");
    let node = Node::Node::originate(None);
    //let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&node).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}\n", serialized);
    let deserialized: Node::Node = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}



