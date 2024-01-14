use uuid::Uuid;
use yrs::Doc;

pub struct Document {
    id: Uuid,
    crdt: Doc,
}

impl Document {
    pub fn new() -> Document {
        Document {
            id: Uuid::new_v4(),
            crdt: Doc::new(), //todo make this into a trait so we can swap between crdts
        }
    }
    pub fn sync(&self, doc: &Document) -> &Document {
        self
    }
    pub fn serialize(&self) -> () {}
    pub fn deserialize(&self) -> () {}
}
