
use std::ops::{DivAssign, MulAssign};
use std::str::from_utf8;
use numpy::ndarray::{Array1, Array2};
use numpy::array::{PyArray1,PyArray2};
use numpy::IntoPyArray;
use numpy::PyArrayMethods;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use ahash::{AHasher, RandomState};
use std::collections::{HashMap, HashSet};


// ######################################################################################

#[pyfunction] 
pub fn map_vocabulary_rust<'pyt>(corpus_list: &Bound<'pyt, PyList>, vocabulary_py: &Bound< PyDict >, kmer_size: usize) -> HashMap<String,usize, RandomState> {

    let corpus: Vec<String> = corpus_list.extract().expect("Error unpacking Python object to Rust");
    let mut vocabulary:HashMap<String,usize, RandomState>= vocabulary_py.extract().expect("Error unpacking Python object to Rust");

    for seq in corpus {

        for word_in_bytes in seq.as_bytes().chunks(kmer_size) {

            let word= from_utf8(word_in_bytes).unwrap().to_ascii_lowercase();

            if !vocabulary.contains_key(&word) {

                vocabulary.insert(word.to_string(), vocabulary.len());
            }
            
        }
    }
    vocabulary 
}



fn compute_df<'a>(corpus: &Vec<String>, vocabulary: &HashMap<String, usize, RandomState> , kmer_size: usize ) -> Array1<i32>  {

    let mut df= Array1::<i32>::zeros(vocabulary.len());

    for seq in corpus {
        
        let mut words_in_seq:HashSet<String, RandomState>= HashSet::default();

        for word_in_bytes in seq.as_bytes().chunks(kmer_size) {

            let word= from_utf8(word_in_bytes).unwrap().to_ascii_lowercase();

            if !words_in_seq.contains(&word) {
                
                df[vocabulary[&word]] +=1;
                words_in_seq.insert(word);
            }

        }
    }
    df
}

fn compute_idf<'a>(df: &Array1<i32>, nb_document: usize)-> Array1<f64>{

    let mut idf= Array1::<f64>::zeros(df.len());

    for (index,_val) in df.into_iter().enumerate(){

        idf[index]= ( ( nb_document) as f64 / (df[index]+1) as f64 ).ln();

    }

    idf

}

fn get_number_of_kmer(sequence: &str, kmer_size: usize) -> usize {

    let seq_len= sequence.len();

    if seq_len%kmer_size == 0 { seq_len/kmer_size }

    else { (seq_len/kmer_size) +1 }

}

#[pyfunction]
pub fn transform_idf_rust<'pyt>(py:  Python<'pyt>, corpus_list: &Bound<'pyt, PyList>,
 vocabulary_py: &Bound< PyDict > , idf: Bound< PyArray1<f64> >, kmer_size: usize) -> (Vec<f64>,Vec<usize>,Vec<usize>, (usize,usize)){


    let corpus: Vec<String> = corpus_list.extract().expect("Error unpacking Python object to Rust");
    let vocabulary:HashMap<String,usize, RandomState>= vocabulary_py.extract().expect("Error unpacking Python object to Rust");

    let mut return_data= Vec::<f64>::new();
    let mut return_rows= Vec::<usize>::new();
    let mut return_cols= Vec::<usize>::new();

    let rust_idf= idf.to_owned_array();

    for (row_index,seq) in corpus.iter().enumerate() {
        
        let mut count_dict= HashMap::<usize,usize, RandomState>::default();
        let nb_words= get_number_of_kmer(seq, kmer_size);

        for word_in_bytes in seq.as_bytes().chunks(kmer_size) {

            let word= from_utf8(word_in_bytes).unwrap().to_ascii_lowercase();
            
            if vocabulary.contains_key(&word) {

                let col_index= vocabulary[&word];
                *count_dict.entry(col_index).or_insert(0) += 1;
                
            }
        }

        for (col_index, count) in count_dict.iter() {

            let tf= *count as f64 / nb_words as f64;
            let idf= rust_idf[*col_index];
            let tfidf= tf  * idf;

            if tfidf != 0.0 {
                return_data.push(tfidf);
                return_cols.push(*col_index);
                return_rows.push(row_index);
            }

        }
       
    }

    (return_data, return_rows, return_cols, (corpus.len(), vocabulary.len()))

}

#[pyfunction] 
pub fn fit_idf_rust<'a,'pyt>(py: Python<'pyt>, corpus_list: &Bound<'pyt, PyList> , vocabulary_py: &Bound< PyDict >, kmer_size: usize )
 -> (Bound<'pyt, PyArray1<i32> >, Bound<'pyt, PyArray1<f64> >){
    
    let vocabulary:HashMap<String,usize, RandomState>= vocabulary_py.extract().expect("Error unpacking Python object to Rust");

    let corpus: Vec<String> = corpus_list.extract().expect("Error unpacking Python object to Rust");

    let n_documents= corpus.len();

    let df= compute_df(&corpus, &vocabulary, kmer_size);
    let idf= compute_idf(&df, n_documents);

    (df.into_pyarray_bound(py),idf.into_pyarray_bound(py))
    
}



