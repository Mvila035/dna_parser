
use pyo3::prelude::*;
use rayon::prelude::*;
use pyo3::types::PyList;
use crate::utils;


/// Returns a string with white spaces inserted every k characters.
#[pyfunction]
pub fn insert_white_spaces(seq: String, k: i64) -> String {

    let mut new_str= String::from("");
    let k_usize= k as usize;
    for (i,c) in seq.chars().enumerate() {

        new_str.push(c);

        if (i+1)%k_usize == 0 {

            new_str.push(' ');
    
        }
        
    }   

    new_str
}

fn kmerize(seq: &[u8], window_size: usize, stride: usize, drop_remainder: bool) -> Vec<&[u8]> {

    if window_size == 0 || stride == 0 || seq.len() < window_size {
        panic!("window_size is smaller or equal to 0,
                stride is smaller or equal to 0, or sequence is smaller than window_size")
    }

    let estimated_chunks = (seq.len() - window_size) / stride + 1;
    let mut result = Vec::with_capacity(estimated_chunks);

    let mut start = 0;
    while start + window_size <= seq.len() {
        // Zero-allocation slicing (O(1) fat pointer creation)
        result.push(&seq[start..start + window_size]);
        start += stride;
    }

    if !drop_remainder && start < seq.len() {
        result.push(&seq[start..]);
    }

    result
}



fn encode_parallel<'a>(sequences: &'a [Vec<u8>],
                       window_size: usize, stride: usize,
                       drop_remainder: bool, pool: &rayon::ThreadPool
                        ) -> Vec<Vec<&'a [u8]>> {
    pool.install(|| sequences.par_iter()
                             .map(|seq| kmerize(seq, window_size, stride, drop_remainder))
                             .collect())
}

#[pyfunction]
pub fn make_kmers_rust<'pyt>(
    py: Python<'pyt>,
    sequences_py: &Bound<'pyt, PyList>,
    window_size: usize,
    stride: usize,
    drop_remainder: bool,
    n_jobs: i16,
) -> PyResult<Bound<'pyt, PyList>> {


    let sequences = utils::extract_all_sequences(sequences_py)?;
    let cpu_to_use = utils::check_nb_cpus(n_jobs);
    
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(cpu_to_use)
        .build()
        .expect("Failed to build rayon thread pool");

    let kmer_vecs=
        py.detach(|| encode_parallel(&sequences, window_size, stride, drop_remainder, &pool));

    let mut outer_vec = Vec::with_capacity(kmer_vecs.len());
    for inner in kmer_vecs {
        let inner_list = PyList::new(
            py,
            inner.into_iter().map(|kmer| str::from_utf8(kmer).unwrap_or("")),
        )?;
        outer_vec.push(inner_list);
    }

    PyList::new(py, outer_vec)

}
