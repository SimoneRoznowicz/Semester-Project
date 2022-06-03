use talk::crypto::primitives::hash::{hash, Hash};

mod MPT;
mod Test;
mod client_verify;
mod mpt;
mod node_generic;
mod structs;
mod util;

pub fn get_bit_direction(arr: &[u8; 32], index: u8) -> bool {
    let byte = arr[(index / 8) as usize];
    //print!("**** Byte == {}\n\n", byte);
    let sub_index: u8 = 1 << (7 - (index % 8));
    //print!("**** Subindex == {}\n\n", sub_index);
    print!("**** Byte & sub_index == {}\n\n", (byte & sub_index) > 0);

    (byte & sub_index) > 0
}

fn main() {
    let arr: [u8; 32] = [
        32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10,
        9, 8, 7, 6, 5, 4, 3, 2, 1,
    ];
    get_bit_direction(&arr, 0);
    get_bit_direction(&arr, 1);
    get_bit_direction(&arr, 2);
    get_bit_direction(&arr, 3);
    get_bit_direction(&arr, 4);
    get_bit_direction(&arr, 5);
    get_bit_direction(&arr, 6);
    get_bit_direction(&arr, 7);
    //32:  00100000 --> ciao
    //124: 01111100 --> Hello

    //68:  01000100 --> AAAAA
    //201: 11001001 --> BBBBB
    //157: 10011101 --> CCCCC
    //12:  00001100 --> DDDDD
    //100: 01100100 --> EEEEE
    //104: 01101000 --> FFFFF
    //183: 10110111 --> GGGGG
    //113: 01110001 --> HHHHH

    //0 == FALSE AND 1 == TRUE

    let h1 = hash(&"ciao").unwrap();

    let h2 = hash(&"Hello").unwrap();
    let h3 = hash(&"AAAAA").unwrap();
    let h4 = hash(&"BBBBB").unwrap();
    let h5 = hash(&"CCCCC").unwrap();
    let h6 = hash(&"DDDDD").unwrap();
    let h7 = hash(&"EEEEE").unwrap();
    let h8 = hash(&"FFFFF").unwrap();
    let h9 = hash(&"GGGGG").unwrap();
    let h10 = hash(&"HHHHH").unwrap();

    //print!("hash(&\"ciao\") ==\n {:?}\n\n", h1.to_bytes()[0]);
    print!("hash(&\"Hello\") ==\n {:?}\n\n", h2.to_bytes()[0]);
    print!("hash(&\"AAAAA\") ==\n {:?}\n\n", h3.to_bytes()[0]);
    print!("hash(&\"BBBBB\") ==\n {:?}\n\n", h4.to_bytes()[0]);
    print!("hash(&\"CCCCC\") ==\n {:?}\n\n", h5.to_bytes()[0]);
    print!("hash(&\"DDDDD\") ==\n {:?}\n\n", h6.to_bytes()[0]);
    print!("hash(&\"EEEEE\") ==\n {:?}\n\n", h7.to_bytes()[0]);
    print!("hash(&\"FFFFF\") ==\n {:?}\n\n", h8.to_bytes()[0]);
    print!("hash(&\"GGGGG\") ==\n {:?}\n\n", h9.to_bytes()[0]);
    print!("hash(&\"HHHHH\") ==\n {:?}\n\n", h10.to_bytes()[0]);
}
