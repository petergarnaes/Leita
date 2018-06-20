//! # Frequency Index
//!
//! This inverse index type defines a `FreqIndex` which is a bucket type that only registers the
//! number of times a term occurs in a document

use super::schema_index::DocIndex;

#[derive(Debug)]
pub struct FreqIndex {
    doc_id: String,
    doc_freq: usize
}

impl FreqIndex {
    pub fn from_doc_id(doc_id: &str) -> FreqIndex {
        FreqIndex {
            doc_id: doc_id.to_string(),
            doc_freq: 1
        }
    }
    pub fn freq_inc(&mut self){
        self.doc_freq += 1;
    }
}

impl DocIndex for FreqIndex {
    fn create_bucket(doc_id: &str) -> Self {
        FreqIndex::from_doc_id(doc_id)
    }
    fn index(&mut self, _: (&String,usize)){
        self.freq_inc();
    }
    fn get_doc_id(&self) -> &String {
        &self.doc_id
    }
}
