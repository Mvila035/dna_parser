use numpy::ndarray::{Array1, Array2, Array3, Axis};
use numpy::IntoPyArray;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3::exceptions::{PyTypeError, PyValueError};
use rayon::prelude::*;
use crate::utils;

// --- Scalar Mapping Workers ---

fn encode_scalar_parallel(
    sequences: &[Vec<u8>],
    pad_type: &str,
    length: usize,
    lut: &[Vec<f64>],
    pad_val: f64,
    pool: &rayon::ThreadPool,
) -> Array2<f64> {
    // Pre-fill the array with the padding value instead of zeros
    let mut final_array = Array2::from_elem((sequences.len(), length), pad_val);
    
    pool.install(|| {
        sequences
            .par_iter()
            .zip(final_array.axis_iter_mut(Axis(0)).into_par_iter())
            .for_each(|(seq, mut row)| match pad_type {
                "after" => {
                    for (col, &b) in row.iter_mut().zip(seq.iter()) {
                        *col = lut[b as usize][0];
                    }
                }
                "before" => {
                    for (col, &b) in row.iter_mut().rev().zip(seq.iter().rev()) {
                        *col = lut[b as usize][0];
                    }
                }
                _ => panic!("The only 2 options for the type of padding are 'before' and 'after'."),
            })
    });
    final_array
}

fn encode_scalar_no_pad(
    sequences: &[Vec<u8>],
    lut: &[Vec<f64>],
    pool: &rayon::ThreadPool,
) -> Vec<Array1<f64>> {
    pool.install(|| {
        sequences
            .par_iter()
            .map(|seq| seq.iter().map(|&b| lut[b as usize][0]).collect::<Array1<f64>>())
            .collect()
    })
}

// --- Vector Mapping Workers ---

fn encode_vector_parallel(
    sequences: &[Vec<u8>],
    pad_type: &str,
    length: usize,
    feat_dim: usize,
    lut: &[Vec<f64>],
    pad_vec: &[f64],
    pool: &rayon::ThreadPool,
) -> Array3<f64> {
    // Pre-fill each feature dimension of the array with the padding vector
    let mut final_array = Array3::from_shape_fn((sequences.len(), length, feat_dim), |(_, _, k)| pad_vec[k]);
    
    pool.install(|| {
        sequences
            .par_iter()
            .zip(final_array.axis_iter_mut(Axis(0)).into_par_iter())
            .for_each(|(seq, mut row)| match pad_type {
                "after" => {
                    for (i, &b) in seq.iter().take(length).enumerate() {
                        let feats = &lut[b as usize];
                        for j in 0..feat_dim {
                            row[[i, j]] = feats[j];
                        }
                    }
                }
                "before" => {
                    // Handles proper pre-padding offsets and sequence truncation for length mismatches
                    let take_len = std::cmp::min(seq.len(), length);
                    let start_idx = length - take_len;
                    let seq_start = seq.len() - take_len;
                    
                    for (i, &b) in seq[seq_start..].iter().enumerate() {
                        let feats = &lut[b as usize];
                        for j in 0..feat_dim {
                            row[[start_idx + i, j]] = feats[j];
                        }
                    }
                }
                _ => panic!("The only 2 options for the type of padding are 'before' and 'after'."),
            })
    });
    final_array
}

fn encode_vector_no_pad(
    sequences: &[Vec<u8>],
    feat_dim: usize,
    lut: &[Vec<f64>],
    pool: &rayon::ThreadPool,
) -> Vec<Array2<f64>> {
    pool.install(|| {
        sequences
            .par_iter()
            .map(|seq| {
                let mut arr = Array2::<f64>::zeros((seq.len(), feat_dim));
                for (i, &b) in seq.iter().enumerate() {
                    let feats = &lut[b as usize];
                    for j in 0..feat_dim {
                        arr[[i, j]] = feats[j];
                    }
                }
                arr
            })
            .collect()
    })
}

// --- Main Python Export ---

/// Encodes sequences based on a user-provided dictionary.
/// 
/// # Arguments
/// * `py` - Python GIL token
/// * `sequences` - list of sequences
/// * `mapping` - Python dictionary mapping string characters to a float or list of floats
/// * `default_value` - float or list of floats for unknown characters
/// * `padding_value` - float or list of floats for padding (defaults to `default_value` if None)
/// * `pad_type` - "before" or "after"
/// * `pad_length` - -2 pad to longest, -1 trim to shortest, 0 = no padding, >0 for fixed length
/// * `n_jobs` - number of threads to use, 0 to use every cpu
#[pyfunction]
#[pyo3(signature = (sequences, mapping, default_value, padding_value=None, pad_type="after", pad_length=0, n_jobs=1))]
pub fn encode<'pyt>(
    py: Python<'pyt>,
    sequences: &Bound<'pyt, PyAny>,
    mapping: &Bound<'pyt, PyDict>,
    default_value: &Bound<'pyt, PyAny>,
    padding_value: Option<&Bound<'pyt, PyAny>>,
    pad_type: &str,
    pad_length: i128,
    n_jobs: i16,
) -> PyResult<Py<PyAny>> {
    
    // 1. Extract the default value and determine the feature dimension (K)
    let default_vec: Vec<f64> = if let Ok(v_list) = default_value.extract::<Vec<f64>>() {
        v_list
    } else if let Ok(v_float) = default_value.extract::<f64>() {
        vec![v_float]
    } else {
        return Err(PyTypeError::new_err("default_value must be a float or a list of floats."));
    };
    let feat_dim = default_vec.len();

    // 2. Extract the padding value (or fallback to default_value)
    let pad_vec: Vec<f64> = match padding_value {
        Some(val) => {
            let v: Vec<f64> = if let Ok(v_list) = val.extract::<Vec<f64>>() {
                v_list
            } else if let Ok(v_float) = val.extract::<f64>() {
                vec![v_float]
            } else {
                return Err(PyTypeError::new_err("padding_value must be a float or a list of floats."));
            };
            if v.len() != feat_dim {
                return Err(PyValueError::new_err("padding_value must have the same shape as default_value."));
            }
            v
        },
        None => default_vec.clone(),
    };

    // 3. Build the 256-element Lookup Table (LUT)
    let mut lut = vec![default_vec; 256];
    for (k, v) in mapping.iter() {
        let key_str = k.extract::<String>().map_err(|_| PyTypeError::new_err("Dictionary keys must be strings."))?;
        if key_str.is_empty() { continue; }
        
        let val_vec: Vec<f64> = if let Ok(v_list) = v.extract::<Vec<f64>>() {
            v_list
        } else if let Ok(v_float) = v.extract::<f64>() {
            vec![v_float]
        } else {
            return Err(PyTypeError::new_err("Dictionary values must be floats or lists of floats."));
        };

        if val_vec.len() != feat_dim {
            return Err(PyValueError::new_err("All values in the dictionary must have the same shape as the default_value."));
        }

        let key_byte = key_str.as_bytes()[0];
        lut[key_byte.to_ascii_uppercase() as usize] = val_vec.clone();
        lut[key_byte.to_ascii_lowercase() as usize] = val_vec;
    }

    // 4. Process the sequences using utility functions
    let sequences_rust = utils::extract_all_sequences(sequences)?;
    let cpu_to_use = utils::check_nb_cpus(n_jobs);
    
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(cpu_to_use)
        .build()
        .expect("Failed to build rayon thread pool");

    // 5. Branch execution based on mapping dimensions
    if feat_dim == 1 {
        if pad_length == 0 {
            let results = py.detach(|| encode_scalar_no_pad(&sequences_rust, &lut, &pool));
            let py_list = PyList::empty(py);
            for arr in results {
                py_list.append(arr.into_pyarray(py))?;
            }
            return Ok(py_list.unbind().into());
        } else {
            let vec_length = utils::get_length_vec(&sequences_rust, pad_length);
            let final_array = py.detach(|| encode_scalar_parallel(&sequences_rust, pad_type, vec_length, &lut, pad_vec[0], &pool));
            return Ok(final_array.into_pyarray(py).unbind().into());
        }
    } else {
        if pad_length == 0 {
            let results = py.detach(|| encode_vector_no_pad(&sequences_rust, feat_dim, &lut, &pool));
            let py_list = PyList::empty(py);
            for arr in results {
                py_list.append(arr.into_pyarray(py))?;
            }
            return Ok(py_list.unbind().into());
        } else {
            let vec_length = utils::get_length_vec(&sequences_rust, pad_length);
            let final_array = py.detach(|| encode_vector_parallel(&sequences_rust, pad_type, vec_length, feat_dim, &lut, &pad_vec, &pool));
            return Ok(final_array.into_pyarray(py).unbind().into());
        }
    }
}