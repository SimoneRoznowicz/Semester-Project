use crate::get_bit_direction;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, boxed::Box, vec::Vec};
use talk::crypto::primitives::hash::{hash, Hash};
#[derive(Serialize, Deserialize, Debug, PartialEq)]

pub enum Direction {
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Left {}
impl Left {
    fn get_val() -> bool {
        false
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Right {}

impl Right {
    fn get_val() -> bool {
        true
    }
}

impl From<Left> for Direction {
    fn from(left: Left) -> Self {
        Direction::Left
    }
}

impl From<Right> for Direction {
    fn from(left: Right) -> Self {
        Direction::Right
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Id<K>
where
    K: Serialize,
{
    key: K,
}

impl<K> Id<K>
where
    K: Serialize,
{
    pub fn new(k: K) -> Self {
        Id { key: k }
    }

    pub fn get_key(&self) -> &K {
        &self.key
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sibling {
    hash: Hash,
    direction: Direction,
}
impl Sibling {
    pub fn new(h: Hash, d: Direction) -> Sibling {
        Sibling {
            hash: h,
            direction: d,
        }
    }

    pub fn get_direction(&self) -> &Direction {
        &self.direction
    }

    pub fn get_hash(&self) -> &Hash {
        &self.hash
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Proof {
    siblings: Vec<Sibling>,
}

impl Proof {
    pub fn new(s: Vec<Sibling>) -> Proof {
        Proof { siblings: s }
    }
    pub fn get_siblings(&self) -> &Vec<Sibling> {
        &self.siblings
    }
}
