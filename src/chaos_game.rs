use numpy::ndarray::{ArrayBase, ViewRepr};
use numpy::ndarray::{Array3,Axis, Dim};
use numpy::ToPyArray;
use numpy::PyArray3;
use pyo3::types::PyList;
use pyo3::prelude::*;
use itertools::Itertools;


use std::thread;
use crate::utils;



fn chaos_after(sequence: &str, mut array: ArrayBase<ViewRepr<&mut f64>, Dim<[usize; 2]>>) {


    let mut previous_x= 0 as f64;
    let mut previous_y= 0 as f64;

    for item in array.axis_iter_mut(Axis(0)).zip_longest(sequence.chars()) {

        let nucleotide: char;
        let mut col: ArrayBase<ViewRepr<&mut f64>, Dim<[usize; 1]>>;

        if item.is_both() {
            let (option_col,char) = item.left_and_right();
            col= option_col.unwrap();
            nucleotide= char.unwrap().to_ascii_lowercase();
        }

        else if !item.is_left() {
            break
        }

        else {
            col= item.left().unwrap();
            nucleotide= ' ';
        }
        
        if nucleotide == 'a' {
            col[0]= 0.5 * 1. + 0.5 * previous_x;
            col[1]= 0.5 * 1. + 0.5 * previous_y;

            previous_x= col[0];
            previous_y= col[1];
        }

        else if  nucleotide == 'c'{
            col[0]= 0.5 * -1. + 0.5 * previous_x;
            col[1]= 0.5 * -1. + 0.5 * previous_y;

            previous_x= col[0];
            previous_y= col[1];
        }

        else if nucleotide == 't' || nucleotide == 'u' {
            col[0]= 0.5 * -1. + 0.5 * previous_x;
            col[1]= 0.5 * 1. + 0.5 * previous_y;

            previous_x= col[0];
            previous_y= col[1];
            
        }

        else if nucleotide == 'g' {
            col[0]= 0.5 * 1. + 0.5 * previous_x;
            col[1]= 0.5 * -1. + 0.5 * previous_y;

            previous_x= col[0];
            previous_y= col[1];
            
        }

        else {
            col[0]= previous_x;
            col[1]= previous_y;
        }


    }



}

fn chaos_before(sequence: &str, mut array: ArrayBase<ViewRepr<&mut f64>, Dim<[usize; 2]>>) {


    let mut previous_x= 0 as f64;
    let mut previous_y= 0 as f64;


    let array_rev = array.axis_iter_mut(Axis(0)).rev();
    let sequence_rev: Vec<char>= sequence.chars().rev().collect();

    let zip_tup= array_rev.zip(sequence_rev).rev();

    for (mut col, charac) in zip_tup {

        let nucleotide= charac.to_ascii_lowercase();
        
        if nucleotide == 'a' {
            col[0]= 0.5 * 1. + 0.5 * previous_x;
            col[1]= 0.5 * 1. + 0.5 * previous_y;

            previous_x= col[0];
            previous_y= col[1];
        }

        else if  nucleotide == 'c'{
            col[0]= 0.5 * -1. + 0.5 * previous_x;
            col[1]= 0.5 * -1. + 0.5 * previous_y;

            previous_x= col[0];
            previous_y= col[1];
        }

        else if nucleotide == 't' || nucleotide == 'u' {
            col[0]= 0.5 * -1. + 0.5 * previous_x;
            col[1]= 0.5 * 1. + 0.5 * previous_y;

            previous_x= col[0];
            previous_y= col[1];
            
        }

        else if nucleotide == 'g' {
            col[0]= 0.5 * 1. + 0.5 * previous_x;
            col[1]= 0.5 * -1. + 0.5 * previous_y;

            previous_x= col[0];
            previous_y= col[1];
            
        }

        else {
            col[0]= previous_x;
            col[1]= previous_y;
        }

    }

}





/// Returns the onehot encodings in a Vec for the sequences passed to this fucntion.
///
/// this function parse the type and length of padding for the encoding 
fn encode_chunks(chunk: &[String], mut array: ArrayBase<ViewRepr<&mut f64>, Dim<[usize; 3]>> , pad_type: &str ) {


    if pad_type== "after" {

        for (seq, sub_array) in chunk.iter().zip(array.axis_iter_mut(Axis(0))) {

            chaos_after(seq, sub_array);
            
        }
    }

    else if pad_type == "before" {
        
        for (seq, sub_array) in chunk.iter().zip(array.axis_iter_mut(Axis(0))) {

            chaos_before(seq, sub_array); 
            
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
fn multithreads(sequences: Vec<String>, pad_type: &str, mut array: Array3<f64>, nb_cpus: usize) -> Array3<f64> {

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
pub fn chaos_encoding_rust<'pyt>(py:  Python <'pyt>, sequences_py: &Bound<'pyt, PyList>, pad_type: &str, pad_length: i128, n_jobs: i16 ) ->  Bound<'pyt, PyArray3<f64>> {

    let sequences: Vec<String> = sequences_py.extract().expect("Error unpacking Python object to Rust");

    let vec_length= utils::get_length(&sequences, pad_length);
    let cpu_to_use= utils::check_nb_cpus(n_jobs);

    let mut final_array= Array3::<f64>::zeros((sequences.len(), vec_length, 2));


    final_array= py.allow_threads(move || multithreads(sequences, pad_type, final_array, cpu_to_use));

    final_array.to_pyarray(py)


}