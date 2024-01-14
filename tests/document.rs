use inductiondb::*;

#[test]
/// make sure two documents can sync basic data structures
fn sync_two_documents() {
    let doc1 = Document::new();
    let doc2 = Document::new();
    //todo make change to doc2
    let synced_doc = Document::sync(&doc1, &doc2);
    assert_eq!(Document::serialize(&synced_doc), Document::serialize(&doc2));
}

//todo test serialization
