
use pyo3::prelude::*;

//implement rope?


#[pyfunction]
pub fn make_kmers(seq: String, k: i64) -> String {
    
    /// Returns a string with white spaces inserted every k characters.

    let mut new_str= String::from("");
    let k_usize= k as usize;
    for (i,c) in seq.chars().enumerate() {

        new_str.push(c);

        if (i+1)%k_usize == 0 {

            new_str.push_str(" ");
    
        }
        
    }   

    new_str
}