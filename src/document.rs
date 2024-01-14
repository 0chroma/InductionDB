use uuid::Uuid;
use yrs::Doc;

pub struct Document {
    id: Uuid,
    yrs_doc: Doc,
}

impl Document {
    pub fn new() -> () {
        Document {
            id: Uuid::new_v4(),
            yrs_doc: Doc::new(),
        };
    }
    pub fn serialize(&self) -> () {}
    pub fn deserialize(&self) -> () {}
}
