//! # Schema Dependent Index
//!
//! This module defines the general interface for basic inverse indexes, through `DocIndex` and
//! `Index`. It also implements a `SchemaDependentIndex` which is able to index documents.
//!
//! ## Schema dependent
//!
//! This means that each new document is indexed on its own, and term position is relative to the 
//! document it appears in. A schemaless index all documents would be considered as one long
//! document.
use std::collections::HashMap;

/// Trait representing an indexed document.
pub trait DocIndex {
    /// Method used to inizialize struct by the inverse index.
    fn create_bucket(doc_id: &str) -> Self;
    /// Indexes a document in this bucket. The inverse index determines that the document belongs
    /// in this bucket.
    ///
    /// # Arguments
    ///
    ///  * `index_term` - a tuple containing the term and position that is being indexed in this
    ///  bucket.
    fn index(&mut self, index_term: (&String,usize));
    /// Should return the document id of the document that this bucket represents
    fn get_doc_id(&self) -> &String;
}

/// Trait representing an inverse index
pub trait Index {
    fn index_doc(&mut self, doc_id: &str, term_index: (&String,usize));
    // TODO Returns list of doc ids matching search
    //fn search(&self, term: &String) -> [&String];
}

#[derive(Debug)]
/// Schema dependent inverse index.
pub struct SchemaDependentIndex<T: DocIndex> {
    total_documents: usize,
    doc_table: HashMap<String,Vec<T>>
}

impl<T: DocIndex> SchemaDependentIndex<T> {
    pub fn init() -> Self {
        SchemaDependentIndex {
            total_documents: 0,
            doc_table: HashMap::new()
        }
    }
}

impl<T: DocIndex> Index for SchemaDependentIndex<T> {
    fn index_doc(&mut self,doc_id: &str,(term,position): (&String,usize)) {
        self.total_documents += 1;
        let doc_indexes = self.doc_table.entry(term.clone()).or_insert({
            Vec::new()
        });
        let mut i = 0;
        let mut update = false;
        while i <= doc_indexes.len() {
            update = update || match doc_indexes.get_mut(i) {
                Some(doc_index) => {
                    if doc_index.get_doc_id().eq(doc_id) {
                        doc_index.index((term,position));
                        true
                    } else {
                        false
                    }
                },
                None => false
            };
            if update {break};
            i += 1;
        }
        if !update {
            doc_indexes.push(T::create_bucket(doc_id))
        }
    }
}
