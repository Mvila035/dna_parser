
use numpy::ndarray::ArrayView;
use numpy::ndarray::Array2;
use numpy::{IntoPyArray, PyArray2};
use pyo3::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;


fn map_vocabulary(corpus: &Vec<String>) -> (HashMap<&str,f64>, Vec<&str>) {

    let mut map= HashMap::new();
    let mut word_order= Vec::new();
    let mut words_set= HashSet::new();

    for seq in corpus.iter() {

        let mut current_words= HashSet::new();

        for word in seq.split_whitespace(){
            
            if !words_set.contains(word) {
                word_order.push(word);
                words_set.insert(word);
            }
            
            if !current_words.contains(word){
                map.entry(word).and_modify(|counter| *counter += 1.0).or_insert(1.0);
                current_words.insert(word);
            }
        }

    }

    (map,word_order)
}

fn word_counts(sequence: &String) -> HashMap<&str,f64> {

    let mut counts= HashMap::new();

    for word in sequence.split_whitespace(){

        counts.entry(word).and_modify(|counter| *counter += 1.0).or_insert(1.0);
    }

    counts
}

fn compute_tfidf(length:f64, counts: HashMap<&str, f64>, map: &(HashMap<&str,f64>, Vec<&str>)) -> Vec<f64>{

    let mut tfidf_vec= Vec::new();

    for word in map.1.iter(){

        //match to see if none in case a word isn't in a sequence
        let tf= match counts.get(word){
            Some(x) => x/length,
            None => 0.0,
        };
        
        //here use match in case we use the mapping of the corpus to transforme an sequence that is not 
        //part of it.
        //if a word of this seq not in corpus x= 0 (never appearred in corpus ) instead of None
        let idf= match map.0.get(word){
            Some(x) => (map.0.len() as f64/ (x+1.0)).ln(),
            None=> (map.0.len() as f64).ln(),
        };

        let tfidf= tf*idf;

        tfidf_vec.push(tfidf);

    }

    tfidf_vec
}

#[pyfunction]
pub fn tfidf_encoding<'pyt>(py: Python <'pyt>, corpus: Vec<String>) -> &'pyt PyArray2<f64> {

    let word_map= map_vocabulary(&corpus);
    let nrows= corpus.len();
    let ncols= word_map.0.len();

    let mut matrix =Array2::<f64>::zeros((nrows,ncols));

    for elements in corpus.iter().zip(matrix.rows_mut()) {
        let (seq, mut current_row)= elements;
        let seq_len= seq.split_whitespace().count() as f64;
        let counts= word_counts(seq);

        let tfidf_vec= compute_tfidf(seq_len, counts, &word_map);
        current_row.assign(&ArrayView::from(&tfidf_vec));
        
    }

    
    let py_array= matrix.into_pyarray(py);
    py_array

    
}