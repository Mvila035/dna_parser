use ndarray::ArrayViewMut2;
use numpy::ndarray::{Array3, Array2, Axis};
use numpy::IntoPyArray;
use pyo3::prelude::*;
use pyo3::types::PyList;
use rayon::prelude::*;
use crate::utils;

#[inline]
fn step(nucleotide: u8, r: &mut i32, y: &mut i32, m: &mut i32, k: &mut i32, w: &mut i32, s: &mut i32 ){

    let nuc_lower= nucleotide.to_ascii_lowercase();
    match &nuc_lower {
        b'a' | b'g' => *r += 1,
        b'c' | b't' | b'u' => *y += 1,
        _ => {}
    }

    match &nuc_lower {
        b'a' | b'c' => *m += 1,
        b't' | b'u' | b'g' => *k += 1,
        _ => {}
    }

    match &nuc_lower {
        b'a' | b't' | b'u' => *w += 1,
        b'g' | b'c'  => *s += 1,
        _ => {}
    }
    
}

#[inline]
fn get_downsampling_length(length:usize, downsampling: usize, drop_remainder: bool)-> usize{

    if downsampling > length {
        panic!("Downsampling value is greater than the length of the sequence!")
    }

    if downsampling < 1 {
        panic!("Downsampling value cannot be lower than 1")
    }

    if length%downsampling == 0 || drop_remainder {
        return length/downsampling
    }

    else {
        return (length/downsampling)+1
    }

}
    


fn zcurve_after_fixed(sequence: &[u8], mut array: ArrayViewMut2<i32>, downsampling: usize, drop_remainder: bool) {


    let mut r= 0;
    let mut y= 0;
    let mut m= 0;
    let mut k= 0;
    let mut w= 0;
    let mut s= 0;

    let mut rows=  array.outer_iter_mut();
    if !drop_remainder {
        let mut seq_chunks= sequence.chunks(downsampling);
        for (nucleotides, mut cols ) in (&mut seq_chunks).zip(&mut rows) {
            
            for nt in nucleotides{
            step(*nt, &mut r, &mut y, &mut m, &mut k, &mut w, &mut s);
            }
            cols[0]= r-y;
            cols[1]= m-k;
            cols[2]= w-s;

        }
    }

    else{
        let mut seq_chunks= sequence.chunks_exact(downsampling);
        for (nucleotides, mut cols ) in (&mut seq_chunks).zip(&mut rows) {
            
            for nt in nucleotides{
            step(*nt, &mut r, &mut y, &mut m, &mut k, &mut w, &mut s);
            }
            cols[0]= r-y;
            cols[1]= m-k;
            cols[2]= w-s;

        }

    }
    
    for mut cols in rows {
        cols[0]= r-y;
        cols[1]= m-k;
        cols[2]= w-s;
    }

}




fn zcurve_before_fixed(sequence: &[u8], mut array: ArrayViewMut2<i32>, downsampling: usize, drop_remainder: bool) {


    let mut r= 0;
    let mut y= 0;
    let mut m= 0;
    let mut k= 0;
    let mut w= 0;
    let mut s= 0;


    let mut rows=  array.outer_iter_mut();

    if !drop_remainder{
        let mut seq_chunks= sequence.chunks(downsampling);
        for (nucleotides, mut cols) in (&mut seq_chunks).rev().zip((&mut rows).rev()).rev() {

            for nt in nucleotides{
            step(*nt, &mut r, &mut y, &mut m, &mut k, &mut w, &mut s);
            }
            cols[0]= r-y;
            cols[1]= m-k;
            cols[2]= w-s;

        }
    }

    else {
        let mut seq_chunks= sequence.chunks_exact(downsampling);
        for (nucleotides, mut cols) in (&mut seq_chunks).rev().zip((&mut rows).rev()).rev() {

            for nt in nucleotides{
            step(*nt, &mut r, &mut y, &mut m, &mut k, &mut w, &mut s);
            }
            cols[0]= r-y;
            cols[1]= m-k;
            cols[2]= w-s;

        }
    }

    

}

fn zcurve_no_pad(sequence: &[u8], downsampling: usize, drop_remainder: bool) -> Array2<i32> {
    
    let downsampling_lentgh= get_downsampling_length(sequence.len(), downsampling, drop_remainder);
    let mut array= Array2::<i32>::zeros((downsampling_lentgh,3));
    let mut r= 0;
    let mut y= 0;
    let mut m= 0;
    let mut k= 0;
    let mut w= 0;
    let mut s= 0;


    let mut rows=  array.outer_iter_mut();
    let mut seq_chunks= sequence.chunks_exact(downsampling);
    for (nucleotides, mut cols) in  (&mut seq_chunks).zip(&mut rows){

        for nt in nucleotides {
        step(*nt, &mut r, &mut y, &mut m, &mut k, &mut w, &mut s);
        }

        cols[0]= r-y;
        cols[1]= m-k;
        cols[2]= w-s;

    }

    if !drop_remainder && !seq_chunks.remainder().is_empty() {
        for nt in seq_chunks.remainder(){
            step(*nt, &mut r, &mut y, &mut m, &mut k, &mut w, &mut s);
        }
        let mut cols= rows.next().unwrap();
        cols[0]= r-y;
        cols[1]= m-k;
        cols[2]= w-s;

    }
    array
}




/// Encodes all sequences in parallel into a rectangular `Array3<i32>`
/// of the given `length`.
fn encode_parallel(
    sequences: &[Vec<u8>],
    pad_type: &str,
    length: usize,
    downsampling: usize,
    drop_remainder: bool,
    pool: &rayon::ThreadPool,
) -> Array3<i32> {
    let downsampling_lentgh= get_downsampling_length(length, downsampling, drop_remainder);
    let mut final_array= Array3::<i32>::zeros((sequences.len(), downsampling_lentgh, 3));
    pool.install(|| {
        sequences
            .par_iter()
            .zip(final_array.axis_iter_mut(Axis(0)).into_par_iter())
            .for_each(|(seq,  row)| match pad_type {
                "after" => zcurve_after_fixed(seq, row, downsampling, drop_remainder),
                "before" => zcurve_before_fixed(seq, row, downsampling,  drop_remainder),
                _ => panic!("The only 2 options for the type of padding are 'before' and 'after'."),
    
                })
    });

    final_array
}

fn encode_parallel_no_pad(sequences: &[Vec<u8>],downsampling: usize, drop_remainder: bool, pool: &rayon::ThreadPool) -> Vec<Array2<i32>> {
    pool.install(|| sequences.par_iter().map(|seq| zcurve_no_pad(seq, downsampling, drop_remainder))).collect()
}

/// Returns a Numpy i32 3D array, or -- when `pad_length == 0` -- a Python
/// `list` of 2D Numpy i32 arrays, one per sequence, unpadded/untrimmed.
///
/// # Arguments
/// * `py` - Python GIL token (used to acquire the GIL)
/// * `sequences` - Vec of &str representing the sequences to encode
/// * `pad_type` - &str indicating to padd (or trim) "before" or "after" the sequences
/// * `pad_length` - -2 to pad according to the longest sequence, -1 to trim to the shortest sequence, 0 for no paddding, any positive number for a fixed length.
/// * `n_jobs` - number of threads to use. 0 to use every cpu
#[pyfunction]
pub fn zcurve_encoding_rust<'pyt>(
    py: Python<'pyt>,
    sequences_py: &Bound<'pyt, PyList>,
    pad_type: &str,
    pad_length: i128,
    downsampling: usize,
    drop_remainder: bool,
    n_jobs: i16,
) -> PyResult<Py<PyAny>> {
    let sequences = utils::extract_all_sequences(sequences_py)?;
    let cpu_to_use = utils::check_nb_cpus(n_jobs);

    
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(cpu_to_use)
        .build()
        .expect("Failed to build rayon thread pool");

    if pad_length == 0 {
        let results = py.detach(|| encode_parallel_no_pad(&sequences, downsampling, drop_remainder, &pool));
        let py_list = PyList::empty(py);
        for arr in results {
            py_list.append(arr.into_pyarray(py))?;
        }
        return Ok(py_list.unbind().into());
    }

    

    let vec_length = utils::get_length_vec(&sequences, pad_length);
    let final_array =
        py.detach(|| encode_parallel(&sequences, pad_type, vec_length, downsampling, drop_remainder, &pool));

    Ok(final_array.into_pyarray(py).unbind().into())
}

