use ndarray::prelude::*;
use numpy::ndarray::{Array1, Axis};
use numpy::array::PyArray1;
use numpy::IntoPyArray;
use pyo3::prelude::*;
use rayon::prelude::*;
use pyo3::exceptions::PyValueError;
use ahash::HashMap;
use ahash::HashMapExt;
use crate::utils::*;



type CsrMatrix = (Py<PyArray1<f64>>, Py<PyArray1<i32>>, Py<PyArray1<i32>>);
type CsrAndIDF = (Py<PyArray1<f64>>, Py<PyArray1<i32>>, Py<PyArray1<i32>>, Py<PyArray1<f64>> );
type FitTransResults = (Vocabulary, Py<PyArray1<f64>>, Py<PyArray1<i32>>, Py<PyArray1<i32>>,  Py<PyArray1<f64>>);


#[pyclass]
pub struct Vocabulary {
    vocab: HashMap<Vec<u8>, usize>,
}

#[pymethods]
impl Vocabulary {
    #[new]
    #[pyo3(signature = (py_vocab=None))]
    pub fn new(py_vocab: Option<HashMap<String, usize>>) -> Self {
        match py_vocab {
            Some(d) => Self {
                vocab: d.into_iter().map(|(k, v)| (k.into_bytes(), v)).collect(),
            },
            None => Self {
                vocab: HashMap::new(),
            },
        }
    }

    /// Insert a byte key and ID into the internal map
    pub fn insert(&mut self, key: Vec<u8>, id: usize) {
        self.vocab.insert(key, id);
    }

    pub fn __len__(&self) -> usize {
        self.vocab.len()
    }
    /// Retrieve the internal count/size
    pub fn len(&self) -> usize {
        self.vocab.len()
    }

    pub fn to_string_dict(&self) -> PyResult<HashMap<String, usize>> {
        self.vocab
            .iter()
            .map(|(k, &v)| {
                String::from_utf8(k.clone())
                    .map(|s| (s, v))
                    .map_err(|e| PyValueError::new_err(format!("Invalid UTF-8 sequence: {e}")))
            })
            .collect()
    }

}

impl Vocabulary {
        pub fn from_bytes(rust_vocab: Option<HashMap<Vec<u8>, usize>>) -> Self {
        match rust_vocab {
            Some(d) => Self {
                vocab: d,
            },
            None => Self {
                vocab: HashMap::new(),
            },
        }
    }
}

struct KmerBuffer {
    counts: Vec<f64>,     // count array
    touched: Vec<usize>,  // indices with non-zero counts for the current sequence -> column indices
}

impl KmerBuffer {
    fn new(vocab_len: usize) -> Self {
        Self {
            counts: vec![0.0; vocab_len],
            touched: Vec::new(),
        }
    }
}

fn inplace_csr_l2_normalize(vals: &mut Array1<f64>, indptr: &Array1<i32>) {

    for (r_start, r_end) in indptr.iter().zip(indptr.iter().skip(1)) {

        let mut row= vals.slice_mut(s![*r_start as usize..*r_end as usize]);
        let norm= row.mapv(|x| x.powi(2)).sum().sqrt();

        row.map_inplace(|x| *x = *x/norm);

    }
}

fn tfidf_inplace(tf: &mut Array1<f64>, idf: &Array1<f64>, col_idx: &Array1<i32>, n_jobs: i16) {
    
    tf.axis_chunks_iter_mut(Axis(0), tf.len()/n_jobs as usize)
    .into_par_iter()
    .enumerate()
    .for_each(|(chunk_idx, chunk_view)|{

        let chunk_size= chunk_view.len();
        let mut global_index= chunk_idx*chunk_size;
        for val in chunk_view{
            *val= *val * idf[col_idx[global_index] as usize];
            global_index+=1;
        }

    });
    
    
}

fn count_kmers( mut sequence: Vec<u8>, kmer_size: usize, vocabulary: &HashMap<Vec<u8>, usize>, buf: &mut KmerBuffer) -> (Vec<f64>, Vec<i32>) {
   
    sequence.make_ascii_lowercase();

    for kmer in sequence.chunks(kmer_size) {
        if let Some(&idx) = vocabulary.get(kmer) {
            if buf.counts[idx] == 0.0 {
                buf.touched.push(idx);
            }
            buf.counts[idx] += 1.0;
        }
    }

    // sort column index for csr here -> because one sequence is a row
    buf.touched.sort_unstable();

    let mut values = Vec::with_capacity(buf.touched.len());
    let mut cols = Vec::with_capacity(buf.touched.len());

    // Emit and reset only what we touched
    for idx in buf.touched.drain(..) {
        values.push(buf.counts[idx]);
        cols.push(idx as i32);
        buf.counts[idx] = 0.0;
    }

    (values, cols)
}

// build vecs for a csr matrix
fn get_counts_csr(sequences: Vec<Vec<u8>>, vocabulary: &HashMap<Vec<u8>, usize>, kmer_size: usize, n_jobs: usize,) -> (Vec<f64>, Vec<i32>, Vec<i32>) {
    assert!(kmer_size > 0, "kmer_size must be > 0");

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(n_jobs)
        .build()
        .expect("Error building the thread pool.");

    let vocab_len = vocabulary.len();

    // Indexed parallel iterator => collect() preserves sequence order
    let rows: Vec<(Vec<f64>, Vec<i32>)> = pool.install(|| {
        sequences
            .into_par_iter()
            .map_init(
                || KmerBuffer::new(vocab_len),
                |buf, seq| count_kmers(seq, kmer_size, vocabulary, buf),
            )
            .collect()
    });

    // Build indptr and flatten
    let nnz: usize = rows.iter().map(|(v, _)| v.len()).sum();
    assert!(nnz <= i32::MAX as usize, "nnz exceeds i32 range; use i64 indices");

    let mut data = Vec::with_capacity(nnz);
    let mut indices = Vec::with_capacity(nnz);
    let mut indptr = Vec::with_capacity(rows.len() + 1);
    indptr.push(0i32);

    for (vals, cols) in rows {
        data.extend_from_slice(&vals);
        indices.extend_from_slice(&cols);
        indptr.push(data.len() as i32);
    }

    (data, indices, indptr)
}





fn map_vocabulary(sequences: &mut [Vec<u8>], kmer_size: usize) -> HashMap<Vec<u8>, usize> {
    let mut vocabulary = HashMap::new();
    let mut col_index = 0;

    for sequence in sequences {
        sequence.make_ascii_lowercase();
        for kmer_bytes in sequence.chunks(kmer_size) {
            if !vocabulary.contains_key(kmer_bytes) {
                vocabulary.insert(kmer_bytes.to_owned(), col_index); // only allocate on actual insert
                col_index += 1;
            }
        }
    }
    vocabulary
}

fn document_frequency(indices: &Array1<i32>, vocab_len: usize) -> Array1<f64> {
    let mut df: Array1<f64> = Array1::zeros(vocab_len);
    for &col in indices {
        df[col as usize] += 1.0;
    }
    df
}

fn df_to_idf(arr: &mut Array1<f64>, n_seq: f64, smooth_idf: bool, original: bool){
    
    if !original {

        if smooth_idf {
            arr.map_inplace(|x| *x = ( (1.0+n_seq) / (1.0+ *x)).ln());
        }

        else {
            arr.map_inplace(|x| *x =  (n_seq / *x).ln());
        }
        
        *arr= &*arr + 1.0;

    }

    else{
        arr.map_inplace(|x| *x =  (n_seq / *x).ln());
    }
   
}

#[pyfunction]
#[pyo3(signature = (sequences, kmer_size))]
pub fn get_vocab<'py>(
     py: Python<'py>, 
    sequences: &Bound<'py, PyAny>, 
    kmer_size: usize, 
) -> PyResult<Vocabulary> {
    
    let mut sequences_rust = extract_all_sequences(sequences)?;
    let vocabulary= py.detach(|| Vocabulary::from_bytes( Some(map_vocabulary(&mut sequences_rust, kmer_size))));

    Ok(vocabulary)
    
}



#[pyfunction]
#[pyo3(signature = (sequences, vocabulary, kmer_size, smooth_idf=true, original=false, n_jobs=1))]
pub fn fit_rust<'py>(
    py: Python<'py>, 
    sequences: &Bound<'py, PyAny>,
    vocabulary: &Vocabulary, 
    kmer_size: usize,
    smooth_idf: bool,
    original: bool,
    n_jobs: i16
) -> PyResult<Py<PyArray1<f64>>> {
    
    let sequences_rust = extract_all_sequences(sequences)?;
    
    let idf= py.detach(|| {
        let cpu_to_use = check_nb_cpus(n_jobs);
        
        let (_, col_idx, indptr) = get_counts_csr(sequences_rust, &vocabulary.vocab, kmer_size, cpu_to_use);
        let col_idx= Array1::from_vec(col_idx);
        let indptr= Array1::from_vec(indptr);
        
        let n_seq= (indptr.len()-1) as f64;

        let mut idf= document_frequency(&col_idx, vocabulary.len());
        df_to_idf(&mut idf, n_seq, smooth_idf, original);

        idf
    });

    Ok(idf.into_pyarray(py).into())
    
}

#[pyfunction]
#[pyo3(signature = (sequences, vocabulary, kmer_size, l2_normalize=true, smooth_idf=true, original=false, n_jobs=1))]
pub fn transform_rust<'py>(
    py: Python<'py>, 
    sequences: &Bound<'py, PyAny>, 
    vocabulary: &Vocabulary, 
    kmer_size: usize, 
    l2_normalize: bool,
    smooth_idf: bool,
    original: bool,
    n_jobs: i16,
) -> PyResult<CsrMatrix> {

    let sequences_rust = extract_all_sequences(sequences)?;
    
    let (counts, col_idx, indptr)= py.detach(|| {
        let cpu_to_use = check_nb_cpus(n_jobs);

        let (counts, col_idx, indptr) = get_counts_csr(sequences_rust, &vocabulary.vocab, kmer_size, cpu_to_use);
        let mut counts= Array1::from_vec(counts);
        let col_idx= Array1::from_vec(col_idx);
        let indptr=Array1::from_vec(indptr);
        
        let n_seq= (indptr.len()-1) as f64;

        let mut idf= document_frequency(&col_idx, vocabulary.len());
        df_to_idf(&mut idf, n_seq, smooth_idf, original);

        tfidf_inplace(&mut counts, &idf, &col_idx, n_jobs);

        if l2_normalize {
            inplace_csr_l2_normalize(&mut counts, &indptr);
        }

        (counts, col_idx, indptr)
    });

    Ok((
        counts.into_pyarray(py).into(),
        col_idx.into_pyarray(py).into(),
        indptr.into_pyarray(py).into(),
    ))
}


#[pyfunction]
#[pyo3(signature = (sequences, vocabulary, kmer_size, l2_normalize=true, smooth_idf=true, original=false, n_jobs=1))]
pub fn fit_transform_with_voc<'py>(
    py: Python<'py>, 
    sequences: &Bound<'py, PyAny>,
    vocabulary: &Vocabulary,
    kmer_size: usize,
    l2_normalize: bool,
    smooth_idf: bool,
    original: bool,
    n_jobs: i16
) -> PyResult<CsrAndIDF> {

    let sequences_rust = extract_all_sequences(sequences)?;
    
        let (counts, col_idx, indptr, idf) = py.detach(||{
        let cpu_to_use = check_nb_cpus(n_jobs);
        
        let (counts, col_idx, indptr) = get_counts_csr(sequences_rust, &vocabulary.vocab, kmer_size, cpu_to_use);
        let mut counts= Array1::from_vec(counts);
        let col_idx= Array1::from_vec(col_idx);
        let indptr=Array1::from_vec(indptr);
        
        let n_seq= (indptr.len()-1) as f64;

        let mut idf= document_frequency(&col_idx, vocabulary.len());
        df_to_idf(&mut idf, n_seq, smooth_idf, original);

        tfidf_inplace(&mut counts, &idf, &col_idx, n_jobs);

        if l2_normalize {
            inplace_csr_l2_normalize(&mut counts, &indptr);
        }
        
        (counts, col_idx, indptr, idf)
    });

    Ok((
      counts.into_pyarray(py).into(),
      col_idx.into_pyarray(py).into(),
      indptr.into_pyarray(py).into(),
      idf.into_pyarray(py).into(),
    ))
}


#[pyfunction]
#[pyo3(signature = (sequences, kmer_size, l2_normalize=true, smooth_idf=true, original=false, n_jobs=1))]
pub fn fit_transform_rust<'py>(
    py: Python<'py>, 
    sequences: &Bound<'py, PyAny>,
    kmer_size: usize,
    l2_normalize: bool,
    smooth_idf: bool,
    original: bool,
    n_jobs: i16
) -> PyResult<FitTransResults> {

    let mut sequences_rust = extract_all_sequences(sequences)?;

   let(vocabulary, counts, col_idx, indptr, idf)= py.detach( || {
        let cpu_to_use = check_nb_cpus(n_jobs);
        let vocabulary= Vocabulary::from_bytes( Some(map_vocabulary(&mut sequences_rust, kmer_size)));
        
        let (counts, col_idx, indptr) = get_counts_csr(sequences_rust, &vocabulary.vocab, kmer_size, cpu_to_use);
        let mut counts= Array1::from_vec(counts);
        let col_idx= Array1::from_vec(col_idx);
        let indptr=Array1::from_vec(indptr);
        
        let n_seq= (indptr.len()-1) as f64;

        let mut idf= document_frequency(&col_idx, vocabulary.len());
        df_to_idf(&mut idf, n_seq, smooth_idf, original);

        tfidf_inplace(&mut counts, &idf, &col_idx, n_jobs);

        if l2_normalize {
            inplace_csr_l2_normalize(&mut counts, &indptr);
        }
        (vocabulary, counts, col_idx, indptr, idf)
   });

    Ok((
      vocabulary,
      counts.into_pyarray(py).into(),
      col_idx.into_pyarray(py).into(),
      indptr.into_pyarray(py).into(),
      idf.into_pyarray(py).into(),
    ))
}