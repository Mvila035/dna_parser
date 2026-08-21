use ndarray::ArrayViewMut1;
use numpy::ndarray::{Array1, Array2, Axis};
use numpy::IntoPyArray;
use pyo3::prelude::*;
use pyo3::types::PyList;
use rayon::prelude::*;
use crate::utils;


const ATOMIC_LUT: [i32; 256] = {
    let mut t = [0; 256];
    t[b'A' as usize] = 70;
    t[b'a' as usize] = 70;
    t[b'C' as usize] = 58;
    t[b'c' as usize] = 58;
    t[b'G' as usize] = 78;
    t[b'g' as usize] = 78;
    t[b'T' as usize] = 66;
    t[b't' as usize] = 66;
    t[b'U' as usize] = 66;
    t[b'u' as usize] = 66;
    t
};

fn atomic_after_fixed(sequence: &[u8], mut row: ArrayViewMut1<i32>)  {
    for (col, &b) in row.iter_mut().zip(sequence.iter()) {
        *col = ATOMIC_LUT[b as usize];
    };
}

fn atomic_before_fixed(sequence: &[u8], mut row: ArrayViewMut1<i32>){
    
    for (col, &b) in row.iter_mut().rev().zip(sequence.iter().rev()) {
        *col = ATOMIC_LUT[b as usize];
    };

}

fn atomic_no_pad(sequence: &[u8]) -> Array1<i32> {
    sequence.iter().map(|&b| ATOMIC_LUT[b as usize]).collect()
}

/// Encodes all sequences in parallel into a rectangular `Array2<f64>`
/// of the given `length`.
fn encode_parallel(
    sequences: &[Vec<u8>],
    pad_type: &str,
    length: usize,
    pool: &rayon::ThreadPool,
) -> Array2<i32> {
    let mut final_array= Array2::<i32>::zeros((sequences.len(), length));
    pool.install(|| {
        sequences
            .par_iter()
            .zip(final_array.axis_iter_mut(Axis(0)).into_par_iter())
            .for_each(|(seq,  row)| match pad_type {
                "after" => atomic_after_fixed(seq, row),
                "before" => atomic_before_fixed(seq, row),
                _ => panic!("The only 2 options for the type of padding are 'before' and 'after'."),
    
                })
    });

    final_array
}

/// Encodes all sequences in parallel with no padding/trimming: each keeps
/// its own length. Order is preserved guaranteed by rayon .collect()
fn encode_parallel_no_pad(sequences: &[Vec<u8>], pool: &rayon::ThreadPool) -> Vec<Array1<i32>> {
    pool.install(|| sequences.par_iter().map(|seq| atomic_no_pad(seq)).collect())
}

/// Returns a Numpy i32 2D array, or -- when `pad_length == 0` -- a Python
/// `list` of 1D Numpy i32 arrays, one per sequence, unpadded/untrimmed.
///
/// # Arguments
/// * `py` - Python GIL token
/// * `sequences_py` - list of sequences: `str`, `bytes`/`bytearray`, or any
///   object exposing a `.seq` attribute (e.g. a needletail `SequenceRecord`)
/// * `pad_type` - "before" or "after" (ignored when `pad_length == 0`)
/// * `pad_length` - -2 pad to longest, -1 trim to shortest, 0 = no padding
///   (returns a `list` of ragged 1D arrays), any positive number for a
///   fixed length.
/// * `n_jobs` - number of threads to use, 0 to use every cpu
#[pyfunction]
#[pyo3(signature = (sequences, pad_type="after", pad_length=0, n_jobs=1))]
pub fn atomic_encoding<'pyt>(
    py: Python<'pyt>,
    sequences: &Bound<'pyt, PyAny>,
    pad_type: &str,
    pad_length: i128,
    n_jobs: i16,
) -> PyResult<Py<PyAny>> {
    let sequences_rust = utils::extract_all_sequences(sequences)?;
    let cpu_to_use = utils::check_nb_cpus(n_jobs);

    
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(cpu_to_use)
        .build()
        .expect("Failed to build rayon thread pool");

    if pad_length == 0 {
        let results = py.detach(|| encode_parallel_no_pad(&sequences_rust, &pool));
        let py_list = PyList::empty(py);
        for arr in results {
            py_list.append(arr.into_pyarray(py))?;
        }
        return Ok(py_list.unbind().into());
    }

    let vec_length = utils::get_length_vec(&sequences_rust, pad_length);
    let final_array =
        py.detach(|| encode_parallel(&sequences_rust, pad_type, vec_length, &pool));

    Ok(final_array.into_pyarray(py).unbind().into())
}