
use pyo3::prelude::*;

//implement rope

fn string_to_kmers(k: i64, sequence: &str) -> String {

    let mut new_str= String::from("");
    let k_usize= k as usize;
    for (i,c) in sequence.chars().enumerate() {

        new_str.push(c);

        if (i+1)%k_usize == 0 {

            new_str.push_str(" ");
    
        }
        
    }   
    return new_str
}

#[pyfunction]
pub fn make_kmers(seq: String, k: i64) -> String {

    let return_seq= string_to_kmers(k,&seq);

    return return_seq
}