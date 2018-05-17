use std::fs::{File,read_dir};
use std::io::prelude::*;
mod freq_index;
use freq_index::FreqIndex;
mod schema_index;
use schema_index::{SchemaDependentIndex,Index};

fn main() {
    //FreqIndex::from_doc_id(&String::from("bob.txt"));
    let removes = [";",",",".","\n","\"","-",":","!","?","(",")","–"];
    let filename = String::from("hc_crawler/eventyr/Bedstemoder.txt");
    let mut f = File::open(&filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    //println!("With text:\n{}", contents);

    for p in removes.iter() {
        contents = contents.replace(p," ");
    }
    let result: Vec<_> = contents.split_whitespace().collect();

    let mut schema_index: SchemaDependentIndex<FreqIndex> = SchemaDependentIndex::init();
    for r in result {
        println!("{}",r);
        schema_index.index_doc(&filename,(&r.to_string(),0));
    }
    println!("{:?}",schema_index);

    let paths = read_dir("hc_crawler/eventyr").unwrap();
    for path in paths {
        println!("Name: {}",path.unwrap().path().display().to_string())
    }
}
