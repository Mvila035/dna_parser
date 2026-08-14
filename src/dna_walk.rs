use ndarray::ArrayViewMut2;
use numpy::ndarray::{Array3, Array2, Axis};
use numpy::IntoPyArray;
use pyo3::prelude::*;
use pyo3::types::PyList;
use rayon::prelude::*;
use crate::utils;

#[inline]
fn step(nucleotide: u8, x: &mut i32, y: &mut i32){
    match nucleotide.to_ascii_lowercase() {
        b'a' => *x -= 1,
        b'c' => *y -= 1,
        b'g' => *y += 1,
        b't' | b'u' => *x += 1,
        _ => {}
    }

}


fn walk_after_fixed(sequence: &[u8], mut array: ArrayViewMut2<i32>) {

    let mut x= 0 as i32;
    let mut y= 0 as i32;

    let mut rows=  array.outer_iter_mut();
    for (&nucleotide, mut cols) in sequence.iter().zip(&mut rows) {

        step(nucleotide, &mut x, &mut y);
        cols[0]= x;
        cols[1]= y;

    }
    
    for mut cols in rows {
        cols[0] = x;
        cols[1] = y;
    }
}


fn walk_before_fixed(sequence: &[u8], mut array: ArrayViewMut2<i32>) {


    let mut x= 0 as i32;
    let mut y= 0 as i32;

    let mut rows=  array.outer_iter_mut();
    for (mut cols, &nucleotide) in (&mut rows).rev().zip(sequence.iter().rev()).rev() {

        step(nucleotide, &mut x, &mut y);
        cols[0]= x;
        cols[1]= y;

    }
}


fn walk_no_pad(sequence: &[u8], ) -> Array2<i32> {

    let mut array= Array2::<i32>::zeros((sequence.len(),2));
    let mut x= 0 as i32;
    let mut y= 0 as i32;

    let mut rows=  array.outer_iter_mut();
    for (mut cols, &nucleotide) in (&mut rows).zip(sequence.iter()) {

        step(nucleotide, &mut x, &mut y);
        cols[0]= x;
        cols[1]= y;

    }
    array   
}



/// Encodes all sequences in parallel into a rectangular `Array3<i32>`
/// of the given `length`.
fn encode_parallel(
    sequences: &[Vec<u8>],
    pad_type: &str,
    length: usize,
    pool: &rayon::ThreadPool,
) -> Array3<i32> {
    let mut final_array= Array3::<i32>::zeros((sequences.len(), length, 2));
    pool.install(|| {
        sequences
            .par_iter()
            .zip(final_array.axis_iter_mut(Axis(0)).into_par_iter())
            .for_each(|(seq,  row)| match pad_type {
                "after" => walk_after_fixed(seq, row),
                "before" => walk_before_fixed(seq, row),
                _ => panic!("The only 2 options for the type of padding are 'before' and 'after'."),
    
                })
    });

    final_array
}

fn encode_parallel_no_pad(sequences: &[Vec<u8>], pool: &rayon::ThreadPool) -> Vec<Array2<i32>> {
    pool.install(|| sequences.par_iter().map(|seq| walk_no_pad(seq)).collect())
}

/// Returns a Numpy i32 3D array, or -- when `pad_length == 0` -- a Python
/// `list` of 1D Numpy i32 arrays, one per sequence, unpadded/untrimmed.
///
/// # Arguments
/// * `py` - Python GIL token (used to acquire the GIL)
/// * `sequences` - Vec of &str representing the sequences to encode
/// * `pad_type` - &str indicating to padd (or trim) "before" or "after" the sequences
/// * `pad_length` - -2 to pad according to the longest sequence, -1 to trim to the shortest sequence, 0 for no paddding, any positive number for a fixed length.
/// * `n_jobs` - number of threads to use. 0 to use every cpu
#[pyfunction]
pub fn dna_walk_rust<'pyt>(
    py: Python<'pyt>,
    sequences_py: &Bound<'pyt, PyAny>,
    pad_type: &str,
    pad_length: i128,
    n_jobs: i16,
) -> PyResult<Py<PyAny>> {
    let sequences = utils::extract_all_sequences(sequences_py)?;
    let cpu_to_use = utils::check_nb_cpus(n_jobs);

    
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(cpu_to_use)
        .build()
        .expect("Failed to build rayon thread pool");

    if pad_length == 0 {
        let results = py.detach(|| encode_parallel_no_pad(&sequences, &pool));
        let py_list = PyList::empty(py);
        for arr in results {
            py_list.append(arr.into_pyarray(py))?;
        }
        return Ok(py_list.unbind().into());
    }

    let vec_length = utils::get_length_vec(&sequences, pad_length);
    let final_array =
        py.detach(|| encode_parallel(&sequences, pad_type, vec_length, &pool));

    Ok(final_array.into_pyarray(py).unbind().into())
}
