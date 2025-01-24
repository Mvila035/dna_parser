use numpy::ndarray::{ArrayBase, ArrayView,ViewRepr};
use numpy::ndarray::{Array3,Axis, Dim};
use numpy::ToPyArray;
use numpy::PyArray3;
use pyo3::types::PyList;
use pyo3::prelude::*;


use std::thread;
use crate::utils;



/// Returns an Array2<i8> which is the onehot encoding representation of the genomic sequence
///
/// this function iterates on the sequence to encode it.
pub fn onehot_after(sequence: &str, mut array: ArrayBase<ViewRepr<&mut i8>, Dim<[usize; 2]>>) {


    for (mut row,charac) in array.axis_iter_mut(Axis(0)).zip(sequence.chars()){


        match charac {

            'C' => row.assign(&ArrayView::from(&[ 1,  0,  0,  0])),
            'G' => row.assign(&ArrayView::from(&[ 0,  1,  0,  0])),
            'A' => row.assign(&ArrayView::from(&[ 0,  0,  1,  0])),
            'T' => row.assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            'U' => row.assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            'c' => row.assign(&ArrayView::from(&[ 1,  0,  0,  0])),
            'g' => row.assign(&ArrayView::from(&[ 0,  1,  0,  0])),
            'a' => row.assign(&ArrayView::from(&[ 0,  0,  1,  0])),
            't' => row.assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            'u' => row.assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            _ => row.assign(&ArrayView::from(&[ 0,  0,  0,  0])),
        }
    }

}


/// Returns an Array2<i8> which is the onehot encoding representation of the genomic sequence
///
/// this function iterates backward on the sequence to encode it and to pad/trim at the beginning of the sequence
pub fn onehot_before(sequence: &str, mut array: ArrayBase<ViewRepr<&mut i8>, Dim<[usize; 2]>>) {

    for (mut row , charac) in  array.axis_iter_mut(Axis(0)).rev().zip( sequence.chars().rev() ){


        match charac {

            'C' => row.assign(&ArrayView::from(&[ 1,  0,  0,  0])),
            'G' => row.assign(&ArrayView::from(&[ 0,  1,  0,  0])),
            'A' => row.assign(&ArrayView::from(&[ 0,  0,  1,  0])),
            'T' => row.assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            'U' => row.assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            'c' => row.assign(&ArrayView::from(&[ 1,  0,  0,  0])),
            'g' => row.assign(&ArrayView::from(&[ 0,  1,  0,  0])),
            'a' => row.assign(&ArrayView::from(&[ 0,  0,  1,  0])),
            't' => row.assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            'u' => row.assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            _ => row.assign(&ArrayView::from(&[ 0,  0,  0,  0])),
        }
    }

}



/// Returns the onehot encodings in a Vec for the sequences passed to this fucntion.
///
/// this function parse the type and length of padding for the encoding 
fn encode_chunks(chunk: &[String], mut array: ArrayBase<ViewRepr<&mut i8>, Dim<[usize; 3]>> , pad_type: &str ) {


    if pad_type== "after" {

        for (seq, sub_array) in chunk.iter().zip(array.axis_iter_mut(Axis(0))) {

            onehot_after(seq, sub_array);
            
        }
    }

    else if pad_type == "before" {
        
        for (seq, sub_array) in chunk.iter().zip(array.axis_iter_mut(Axis(0))) {

            onehot_before(seq, sub_array); 
            
        }
    }



    else {

        panic!("The only 2 options for the type of padding are 'before' and 'after'.")
    }


}

/// Returns a Vec of tuples (usize, Vec<Array2<i8>>)
///
/// This function splits the sequences to encode and distributes them to different threads. 
/// the usize is used to keep the order of sequences and the Vec<Array2<i8>> represent the onehot encodings of the genomic sequences
fn multithreads(sequences: Vec<String>, pad_type: &str, mut array: Array3<i8>, nb_cpus: usize) -> Array3<i8> {

    //determine size of chunks based on number of threads and add 1 to be sure 
    //to have a number of chunks equal to nb of cpus and not superior
    let seq_len= sequences.len();
    let slice_len= (seq_len/ nb_cpus) + 1;


// ####################################### begining of threads #####################################
    thread::scope(|s|{

        
        for (chunk_seq,array_slice ) in sequences.chunks(slice_len).zip(array.axis_chunks_iter_mut(Axis(0), slice_len)){

            s.spawn( move || {
                
                encode_chunks(chunk_seq, array_slice, pad_type );
                
            });

        }

    });


// ####################################### end of threads #####################################

    array

}


/// Returns a PyList of Numpy i8 2D array to Python
///
/// # Arguments
/// * `py` - Python GIL token (used to acquire the GIL)
/// * `sequences` - Vec of &str representing the sequences to encode
/// * `pad_type` - &str indicating to padd (or trim) "before" or "after" the sequences
/// * `pad_length` - -2 to pad according to the longest sequence, -1 to trim to the shortest sequence, 0 for no paddding, any positive number for a fixed length.
/// * `n_jobs` - number of threads to use. 0 to use every cpu
#[allow(unused_must_use)]
#[pyfunction]
pub fn onehot_encoding_rust<'pyt>(py:  Python <'pyt>, sequences_py: &Bound<'pyt, PyList>, pad_type: &str, pad_length: i128, n_jobs: i16 ) ->  Bound<'pyt, PyArray3<i8>> {

    let sequences: Vec<String> = sequences_py.extract().expect("Error unpacking Python object to Rust");

    let vec_length= utils::get_length(&sequences, pad_length);
    let cpu_to_use= utils::check_nb_cpus(n_jobs);

    let mut final_array= Array3::<i8>::zeros((sequences.len(), vec_length, 4));


    final_array= py.allow_threads(move || multithreads(sequences, pad_type, final_array, cpu_to_use));

    final_array.to_pyarray(py)


}
