use serde::{Serialize, Deserialize};
mod yrs;
pub use crate::crdt::yrs::Yrs;

pub trait CRDT<'a>: Deserialize<'a> + Serialize<'a> {
   fn new() -> CRDT;
}
