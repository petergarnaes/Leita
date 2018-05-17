use std::collections::HashMap;

pub trait DocIndex {
    fn create_bucket(doc_id: &String) -> Self;
    // Index the document in this bucket
    fn index(&mut self, index_term: (&String,usize));

    fn get_doc_id(&self) -> &String;
}

pub trait Index {
    fn index_doc(&mut self, doc_id: &String, term_index: (&String,usize));
}

#[derive(Debug)]
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
    fn index_doc(&mut self,doc_id: &String,(term,position): (&String,usize)) {
        self.total_documents = self.total_documents + 1;
        let doc_indexes = self.doc_table.entry(term.clone()).or_insert({
            Vec::new()
        });
        // TODO optimize indexing, in beginning we do static indexing so a little time lost here is fine
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
            i = i + 1;
        }
        if !update {
            doc_indexes.push(T::create_bucket(doc_id))
        }
    }
}
