use pyo3::prelude::*;
use pyo3::types::PyList;
use numpy::{PyArray2, ndarray::Array2};
use crate::utils;

#[pyfunction]
pub fn insert_white_spaces(seq: String, k: i64) -> String {
    let mut new_str = String::from("");
    let k_usize = k as usize;
    for (i, c) in seq.chars().enumerate() {
        new_str.push(c);
        if (i + 1) % k_usize == 0 {
            new_str.push(' ');
        }
    }
    new_str
}

fn kmerize_into_buf(
    seq: &[u8],
    window_size: usize,
    stride: usize,
    drop_remainder: bool,
) -> Vec<u8> {
    let n_full = (seq.len() - window_size) / stride + 1;
    let last_start = n_full * stride;           // next window's start position
    let remainder_len = seq.len() - last_start; 

    let has_remainder = !drop_remainder && (seq.len() - window_size) % stride != 0; // check if divides perfectly
    let n_rows = n_full + if has_remainder { 1 } else { 0 };

    let mut buf = vec![0u8; n_rows * window_size]; // zero-init -> padding bytes are 0x00

    let mut start = 0;
    let mut out_off = 0;
    for _ in 0..n_full {
        buf[out_off..out_off + window_size].copy_from_slice(&seq[start..start + window_size]);
        start += stride;
        out_off += window_size;
    }

    if has_remainder {
        buf[out_off..out_off + remainder_len].copy_from_slice(&seq[last_start..]);
        // remaining bytes in this row stay 0x00 (padding)
    }

    buf
}

#[pyfunction]
pub fn make_kmers_rust<'pyt>(
    py: Python<'pyt>,
    sequences_py: &Bound<'pyt, PyAny>,
    window_size: usize,
    stride: usize,
    drop_remainder: bool,
) -> PyResult<Bound<'pyt, PyList>> {
    let sequences = utils::extract_all_sequences(sequences_py)?;

    let buffers: Vec<Vec<u8>> = sequences
        .iter()
        .map(|seq| kmerize_into_buf(seq, window_size, stride, drop_remainder))
        .collect();

    let out = PyList::empty(py);
    for buf in buffers {
        let n_kmers = buf.len() / window_size;
        let arr = Array2::from_shape_vec((n_kmers, window_size), buf).unwrap();
        let py_arr = PyArray2::from_owned_array(py, arr);
        out.append(py_arr)?;
    }

    Ok(out)
}