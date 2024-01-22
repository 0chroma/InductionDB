use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::CRDT;

#[derive(Serialize, Deserialize, Debug)]
pub struct Document<T> where T: CRDT {
    id: Uuid,
    crdt: T,
}

impl<T> Document<T> {
    pub fn new() -> Document<T> {
        Document {
            id: Uuid::new_v4(),
            crdt: T::new(),
        }
    }
    pub fn sync(&self, doc: &Document<T>) -> &Document<T> {
        self
    }
    pub fn serialize(&self) -> () {

    }
    pub fn deserialize(&self) -> () {
        
    }
}
