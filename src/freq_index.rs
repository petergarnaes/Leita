use super::schema_index::DocIndex;

#[derive(Debug)]
pub struct FreqIndex {
    doc_id: String,
    doc_freq: usize
}

impl FreqIndex {
    pub fn from_doc_id(doc_id: &String) -> FreqIndex {
        FreqIndex {
            doc_id: doc_id.clone(),
            doc_freq: 1
        }
    }
    pub fn freq_inc(&mut self){
        self.doc_freq = self.doc_freq + 1;
    }
}

impl DocIndex for FreqIndex {
    fn create_bucket(doc_id: &String) -> Self {
        FreqIndex::from_doc_id(doc_id)
    }
    fn index(&mut self, _: (&String,usize)){
        self.freq_inc();
    }
    fn get_doc_id(&self) -> &String {
        &self.doc_id
    }
}
