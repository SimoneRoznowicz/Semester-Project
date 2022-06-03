use std::collections::HashMap;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
//use std::hash;
//use std::collections::hash_map::DefaultHasher;
//use std::hash::Hash;


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn calculate_hash(t: &String) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    return s.finish();
}

//testare classi e funzioni della libreria collections
fn main(){
    let mut s1 = String::from("ciao");
    let mut s2 = String::from("ciaoo");
    print!("{}\n\n", &calculate_hash(&s1));
    print!("{}\n\n", &calculate_hash(&s2));

    let mut map = HashMap::new();
    map.insert("key1", "va?");
    map.insert("Key2", "come");
    map.insert("Key3", "ciao");
    //let mut str: String = map.get("key3");
    let mut vera_stringa = "ciao come va?";
    //print_type_of(&str);
    print_type_of(&vera_stringa);

    
    println!("La sringa è {:?}",  map.get("key3").unwrap());
    let mut map2 = HashMap::new();
    map2.insert("key1", "ccc");
    //children: Option<HashMap<String, Node>>,

}