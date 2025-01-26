use numpy::array::PyArray1;
use pyo3::prelude::*;
use ahash::HashMap;
use ahash::HashMapExt;
use rayon::prelude::*;
use crate::utils::*;
use indexmap::IndexMap;

type CsrMatrix= (Py<PyArray1<usize>>, Py<PyArray1<usize>>, Py<PyArray1<usize>>);
type VocabAndCsrMatrix= (HashMap<String,usize>,Py<PyArray1<usize>>, Py<PyArray1<usize>>, Py<PyArray1<usize>>);

// transform a sequence into a vec of kmer 
fn make_kmers(sequence: &str, kmer_size: usize) -> Vec<String> {
    let mut kmers= Vec::new();

    let chars: Vec<char>= sequence.chars().collect();
    for kmer in chars.chunks(kmer_size) {
        kmers.push(kmer.iter().collect::<String>().to_lowercase())
    }
    
    kmers
}

fn count_kmers(kmers: Vec<String>) -> IndexMap<String, usize> {
    let mut count= IndexMap::new();

    for kmer in kmers{
        *count.entry(kmer).or_insert(0) += 1;
    }
    count
}

fn get_counts(sequences: Vec<String>, kmer_size: usize, n_jobs:usize) -> Vec<IndexMap<String,usize>> {
    
    create_pool(n_jobs).expect("Error Building the threadpool.").install(|| {

        sequences.par_iter()
                .map(|sequence| count_kmers(make_kmers(sequence, kmer_size)))
                .collect()
   
    })
}

fn map_vocabulary(sequence_counts: &Vec<IndexMap<String,usize>>) -> HashMap<String,usize> {
    let mut vocabulary= HashMap::new();
    let mut col_index= 0;

    for sequence in sequence_counts {
        for kmer in sequence.keys() {
            if !vocabulary.contains_key(kmer) {
                vocabulary.insert(kmer.clone(), col_index);
                col_index +=1;
            }
        }
    }
    vocabulary
}


#[pyfunction]
pub fn fit_rust(sequences: Vec<String>, kmer_size: usize, n_jobs: usize) -> HashMap<String,usize> {
    
    let sequences_counts= get_counts(sequences, kmer_size, n_jobs);
    map_vocabulary(&sequences_counts)
}

#[pyfunction]
pub fn transform_rust(py: Python, sequences: Vec<String>, vocabulary: HashMap<String,usize>, kmer_size: usize, n_jobs: usize)
 -> CsrMatrix {

    let mut val= Vec::new();
    let mut row_indices= Vec::new();
    let mut col_indices= Vec::new();

    let sequences_counts= get_counts(sequences, kmer_size, n_jobs);

    for (row,sequence) in sequences_counts.iter().enumerate() {

        for (kmer, count) in sequence.iter() {
            if let Some(&col) = vocabulary.get(kmer) {
                val.push(*count);
                row_indices.push(row);
                col_indices.push(col);
            }
        };

    };

    ( PyArray1::from_vec(py, val).into(),
      PyArray1::from_vec(py, row_indices).into(),
      PyArray1::from_vec(py, col_indices).into(),
    )
    
}

#[pyfunction]
pub fn fit_transform_rust(py: Python, sequences: Vec<String>, kmer_size: usize, n_jobs: usize)-> VocabAndCsrMatrix {

    let sequences_counts= get_counts(sequences, kmer_size, n_jobs);
    let vocabulary= map_vocabulary(&sequences_counts);

    let mut val= Vec::new();
    let mut row_indices= Vec::new();
    let mut col_indices= Vec::new();

    for (row,sequence) in sequences_counts.iter().enumerate() {

        for (kmer, count) in sequence.iter() {
            if let Some(&col) = vocabulary.get(kmer) {
                val.push(*count);
                row_indices.push(row);
                col_indices.push(col);
            }
        };

    };

    (vocabulary,
      PyArray1::from_vec(py, val).into(),
      PyArray1::from_vec(py, row_indices).into(),
      PyArray1::from_vec(py, col_indices).into(),
    )

}