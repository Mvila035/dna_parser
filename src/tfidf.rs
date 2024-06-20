
use numpy::ndarray::{Array1, Array2};
use numpy::array::PyArray2;
use numpy::IntoPyArray;
use pyo3::prelude::*;
use std::collections::{HashMap, HashSet};


// ######################################################################################

#[pyfunction] 
pub fn map_vocabulary_rust(py: Python, corpus: Vec<&str>, mut vocabulary: HashMap<String, usize>,kmer_size: usize) -> HashMap<String,usize> {

    for seq in corpus {

        let mut word= String::new();
        for (i,c) in seq.chars().enumerate() {
            
            word.push(c);
            
            if ((i+1) % kmer_size == 0) && !vocabulary.contains_key(&word) {
            
                vocabulary.insert(word, vocabulary.len());
                word= String::new();
                
            }

            else if (i+1) % kmer_size == 0 {
                word= String::new();
            }
        }
    }
    vocabulary 
}

fn update_idf_values (idf: Array1<i32>  ) {

    
    
}

// check that word is in voc!!!!
// reset word like in map_vocabulary!!!
fn compute_df(corpus: &Vec<&str>, mut df: HashMap<String, usize>, kmer_size: usize ) -> HashMap<String, usize>  {

    for seq in corpus {
        
        let mut words_in_seq= HashSet::new();
        let mut word= String::new();

        for (i,c) in seq.chars().enumerate() {
            
            word.push(c);
            
            if ((i+1) % kmer_size == 0 ) && !words_in_seq.contains(&word) {

                words_in_seq.insert(word.clone());
                df.insert(word.clone(), df[&word]+1); 
                word= String::new();
                
            }

            else if ((i+1) % kmer_size == 0) {
                word= String::new();
            }
        }   
    }
    df
}

fn compute_idf(df: &HashMap<String, usize>, nb_document: usize)-> HashMap<String, f64>{

    let mut idf: HashMap<String, f64>= HashMap::new();

    for key in df.keys(){

        let idf_value= ( nb_document as f64 / (df[key]+1) as f64 ).ln();
        idf.insert(key.clone(), idf_value);
    }

    idf

}

#[pyfunction]
pub fn transform_idf_rust<'pyt>(py:  Python<'pyt>, corpus: Vec<&str>, vocabulary: HashMap<String,usize>, idf: HashMap<String, f64>, kmer_size: usize) -> &'pyt PyArray2<f64> {

    let mut count_array= Array2::<f64>::zeros((corpus.len(), vocabulary.len()));

    for (row_index,seq) in corpus.iter().enumerate() {
        
        let mut word= String::new();

        for (i,c) in seq.chars().enumerate() {
            
            word.push(c);
            
            if ((i+1) % kmer_size == 0 ) && vocabulary.contains_key(&word) {

                let col_index= vocabulary[&word];
                count_array.column_mut(col_index)[row_index] += 1.0;
                word= String::new();
            }

            else if (i+1) % kmer_size == 0 {
                word= String::new();
            }
        }   
    }

    
    for (key,val) in vocabulary.iter() {
        
        for mut row in count_array.rows_mut(){

            row[*val]= row[*val] * idf[key]; 
            
        }
    }

    count_array.into_pyarray(py)

}

#[pyfunction] 
pub fn fit_idf_rust( corpus: Vec<&str>, vocabulary: HashMap<String,usize>, kmer_size: usize )
 -> (HashMap<String, usize> , HashMap<String, f64>){
    
    let n_documents= corpus.len();
    let mut df: HashMap<String,usize> = HashMap::new();
    
    for key in vocabulary.keys() {
        df.insert(key.clone(), 0);
    }

    df= compute_df(&corpus, df, kmer_size);
    let idf= compute_idf(&df, n_documents);

    (df,idf)
    
}

