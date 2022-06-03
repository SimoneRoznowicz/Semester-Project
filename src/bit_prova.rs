use talk::crypto::primitives::hash::Hash;
use talk::crypto::primitives::hash::hash;

#[inline]
pub fn bit(arr: &[u8; 32], index: u8) -> bool {
    let byte = arr[(index / 8) as usize];
    let sub_index: u8 = 1 << (7 - (index % 8));
    (byte & sub_index) > 0
}
fn main(){
    let my_hash = hash(String::from("ciao come va?"));
    print!({},my_hash);
}