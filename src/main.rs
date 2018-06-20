//! # Leita - search engine in Rust
//!
//! Pet project for learning Rust and search engine algorithms and implementation optimizations.

use std::fs::{File,read_dir};
use std::io::prelude::*;
pub mod freq_index;
use freq_index::FreqIndex;
pub mod schema_index;
use schema_index::{SchemaDependentIndex,Index};

/// Main function testing basic functionality
fn main() {
    //FreqIndex::from_doc_id(&String::from("bob.txt"));
    let removes = [";",",",".","\n","\"","-",":","!","?","(",")","–"];
    let paths = read_dir("hc_crawler/eventyr").unwrap();
    let mut schema_index: SchemaDependentIndex<FreqIndex> = SchemaDependentIndex::init();
    for path in paths {
        let filename = path.unwrap().path().display().to_string();
        println!("Name: {}",filename);
        let mut f = File::open(&filename).expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        for p in &removes {
            contents = contents.replace(p," ");
        }
        let result: Vec<_> = contents.split_whitespace().collect();
        for r in result {
            //println!("{}",r);
            schema_index.index_doc(&filename,(&r.to_lowercase(),0));
        }

    }
    println!("{:#?}",schema_index);
}
