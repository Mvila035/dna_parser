use numpy::array::PyArray1;
use pyo3::prelude::*;
use ahash::HashMap;
use ahash::HashMapExt;
use rayon::prelude::*;
use crate::utils::*;
use indexmap::IndexMap;

type CsrMatrix = (Py<PyArray1<usize>>, Py<PyArray1<usize>>, Py<PyArray1<usize>>);
type VocabAndCsrMatrix = (HashMap<String, usize>, Py<PyArray1<usize>>, Py<PyArray1<usize>>, Py<PyArray1<usize>>);


fn count_kmers(mut sequence: Vec<u8>, kmer_size: usize) -> IndexMap<Vec<u8>, usize> {
    let mut count = IndexMap::new();

    // Lowercase the entire sequence in-place using ASCII rules
    sequence.make_ascii_lowercase();

    
    for kmer in sequence.chunks(kmer_size) {
        *count.entry(kmer.to_vec()).or_insert(0) += 1;
    }
    count
}

fn get_counts(sequences: Vec<Vec<u8>>, kmer_size: usize, n_jobs: usize) -> Vec<IndexMap<Vec<u8>, usize>> {
    create_pool(n_jobs).expect("Error Building the threadpool.").install(|| {
        sequences.into_par_iter()
                .map(|sequence| count_kmers(sequence, kmer_size))
                .collect()
    })
}


fn map_vocabulary(sequence_counts: &[IndexMap<Vec<u8>, usize>]) -> HashMap<String, usize> {
    let mut vocabulary = HashMap::new();
    let mut col_index = 0;

    for sequence in sequence_counts {
        for kmer_bytes in sequence.keys() {
            
            let kmer_str = unsafe { String::from_utf8_unchecked(kmer_bytes.clone()) };
            
            if !vocabulary.contains_key(&kmer_str) {
                vocabulary.insert(kmer_str, col_index);
                col_index += 1;
            }
        }
    }
    vocabulary
}

#[pyfunction]
#[pyo3(signature = (sequences, kmer_size, n_jobs=1))]
pub fn fit_rust<'py>(
    sequences: &Bound<'py, PyAny>, 
    kmer_size: usize, 
    n_jobs: i16
) -> PyResult<HashMap<String, usize>> {
    
    let sequences_rust = extract_all_sequences(sequences)?;
    let cpu_to_use = check_nb_cpus(n_jobs);
    
    let sequences_counts = get_counts(sequences_rust, kmer_size, cpu_to_use);
    Ok(map_vocabulary(&sequences_counts))
}

#[pyfunction]
#[pyo3(signature = (sequences, vocabulary, kmer_size, n_jobs=1))]
pub fn transform_rust<'py>(
    py: Python<'py>, 
    sequences: &Bound<'py, PyAny>, 
    vocabulary: HashMap<String, usize>, 
    kmer_size: usize, 
    n_jobs: i16
) -> PyResult<CsrMatrix> {

    let sequences_rust = extract_all_sequences(sequences)?;
    let cpu_to_use = check_nb_cpus(n_jobs);

    let mut val = Vec::new();
    let mut row_indices = Vec::new();
    let mut col_indices = Vec::new();

    let sequences_counts = get_counts(sequences_rust, kmer_size, cpu_to_use);

    for (row, sequence) in sequences_counts.iter().enumerate() {
        for (kmer_bytes, count) in sequence.iter() {
            
            let kmer_str = unsafe { std::str::from_utf8_unchecked(kmer_bytes) };
            if let Some(&col) = vocabulary.get(kmer_str) {
                val.push(*count);
                row_indices.push(row);
                col_indices.push(col);
            }
        };
    };

    Ok(( 
      PyArray1::from_vec(py, val).into(),
      PyArray1::from_vec(py, row_indices).into(),
      PyArray1::from_vec(py, col_indices).into(),
    ))
}

#[pyfunction]
#[pyo3(signature = (sequences, kmer_size, n_jobs=1))]
pub fn fit_transform_rust<'py>(
    py: Python<'py>, 
    sequences: &Bound<'py, PyAny>, 
    kmer_size: usize, 
    n_jobs: i16
) -> PyResult<VocabAndCsrMatrix> {

    let sequences_rust = extract_all_sequences(sequences)?;
    let cpu_to_use = check_nb_cpus(n_jobs);

    let sequences_counts = get_counts(sequences_rust, kmer_size, cpu_to_use);
    let vocabulary = map_vocabulary(&sequences_counts);

    let mut val = Vec::new();
    let mut row_indices = Vec::new();
    let mut col_indices = Vec::new();

    for (row, sequence) in sequences_counts.iter().enumerate() {
        for (kmer_bytes, count) in sequence.iter() {
            let kmer_str = unsafe { std::str::from_utf8_unchecked(kmer_bytes) };
            if let Some(&col) = vocabulary.get(kmer_str) {
                val.push(*count);
                row_indices.push(row);
                col_indices.push(col);
            }
        };
    };

    Ok((
      vocabulary,
      PyArray1::from_vec(py, val).into(),
      PyArray1::from_vec(py, row_indices).into(),
      PyArray1::from_vec(py, col_indices).into(),
    ))
}