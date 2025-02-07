use numpy::ndarray::{ArrayBase, ViewRepr};
use numpy::ndarray::{Array3,Axis, Dim};
use numpy::ToPyArray;
use numpy::PyArray3;
use pyo3::types::PyList;
use pyo3::prelude::*;
use itertools::Itertools;


use std::thread;
use crate::utils;


fn zcurve_after(sequence: &str, mut array: ArrayBase<ViewRepr<&mut i64>, Dim<[usize; 2]>>, window: usize) {


    let mut r= 0;
    let mut y= 0;
    let mut m= 0;
    let mut k= 0;
    let mut w= 0;
    let mut s= 0;

    for item in array.axis_iter_mut(Axis(0)).zip_longest(sequence.chars().chunks(window).into_iter()) {

        let mut col: ArrayBase<ViewRepr<&mut i64>, Dim<[usize; 1]>>;

        if item.is_left() {
            break
        }

        else if item.is_right() {
            col= item.left().unwrap();
            col[0]= r-y;
            col[1]= m-k;
            col[2]= w-s;
        }

        else  {
            let (option_col,nt_chunk) = item.left_and_right();
            col= option_col.unwrap();
            
            for mut nucleotide in nt_chunk.unwrap() {
                nucleotide= nucleotide.to_ascii_lowercase();

                if nucleotide == 'a' || nucleotide == 'g' {
                    r += 1;
                    col[0]= r-y;
                }
        
                else if  nucleotide == 'c' || nucleotide == 't' || nucleotide == 'u'{
                    y += 1;
                    col[0]= r-y;
                }
        
                if nucleotide == 'a' || nucleotide == 'c' {
                    m += 1;
                    col[1]= m-k;
                    
                }
        
                else if nucleotide == 't' || nucleotide == 'u'|| nucleotide == 'g' {
                    k += 1;
                    col[1]= m-k;
                }
        
                if nucleotide == 'a' || nucleotide == 't' || nucleotide == 'u' {
                    w += 1;
                    col[2]= w-s
                }
        
                else if nucleotide == 'g' || nucleotide == 'c' {
                    s += 1;
                    col[2]= w-s
        
                }
        
                col[0]= r-y;
                col[1]= m-k;
                col[2]= w-s;
                
            }
        }


    


    }



}

fn zcurve_before(sequence: &str, mut array: ArrayBase<ViewRepr<&mut i64>, Dim<[usize; 2]>>, window: usize) {


    let mut r= 0;
    let mut y= 0;
    let mut m= 0;
    let mut k= 0;
    let mut w= 0;
    let mut s= 0;


    let array_rev = array.axis_iter_mut(Axis(0)).rev();
    let sequence_rev: Vec<char>= sequence.chars().rev().collect();

    let zip_tup= array_rev.zip(sequence_rev.chunks(window)).rev();

    for (mut col, nt_chunk) in zip_tup {

        for nt in nt_chunk {

            let nucleotide= nt.to_ascii_lowercase();
        
            if nucleotide == 'a' || nucleotide == 'g' {
                r += 1;
                col[0]= r-y;
            }

            else if  nucleotide == 'c' || nucleotide == 't' || nucleotide == 'u'{
                y += 1;
                col[0]= r-y;
            }

            else {
                col[0]= r-y;
            }

            if nucleotide == 'a' || nucleotide == 'c' {
                m += 1;
                col[1]= m-k;
                
            }

            else if nucleotide == 't' || nucleotide == 'u'|| nucleotide == 'g' {
                k += 1;
                col[1]= m-k;
            }

            else {
                col[1]= m-k;
            }

            if nucleotide == 'a' || nucleotide == 't' || nucleotide == 'u' {
                w += 1;
                col[2]= w-s
            }

            else if nucleotide == 'g' || nucleotide == 'c' {
                s += 1;
                col[2]= w-s

            }

            else {
                col[2]= w-s
            }
        }

    }

}





/// Returns the onehot encodings in a Vec for the sequences passed to this fucntion.
///
/// this function parse the type and length of padding for the encoding 
fn encode_chunks(chunk: &[String], mut array: ArrayBase<ViewRepr<&mut i64>, Dim<[usize; 3]>> , pad_type: &str , window: usize) {


    if pad_type== "after" {

        for (seq, sub_array) in chunk.iter().zip(array.axis_iter_mut(Axis(0))) {

            zcurve_after(seq, sub_array, window);
            
        }
    }

    else if pad_type == "before" {
        
        for (seq, sub_array) in chunk.iter().zip(array.axis_iter_mut(Axis(0))) {

            zcurve_before(seq, sub_array, window); 
            
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
fn multithreads(sequences: Vec<String>, pad_type: &str, mut array: Array3<i64>, window: usize, nb_cpus: usize) -> Array3<i64> {

    //determine size of chunks based on number of threads and add 1 to be sure 
    //to have a number of chunks equal to nb of cpus and not superior
    let seq_len= sequences.len();
    let slice_len= (seq_len/ nb_cpus) + 1;


// ####################################### begining of threads #####################################
    thread::scope(|s|{

        
        for (chunk_seq,array_slice ) in sequences.chunks(slice_len).zip(array.axis_chunks_iter_mut(Axis(0), slice_len)){

            s.spawn( move || {
                
                encode_chunks(chunk_seq, array_slice, pad_type, window );
                
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
pub fn zcurve_encoding_rust<'pyt>(py:  Python <'pyt>, sequences_py: &Bound<'pyt, PyList>, pad_type: &str, pad_length: i128, window: usize, n_jobs: i16 ) ->  Bound<'pyt, PyArray3<i64>> {

    let sequences: Vec<String> = sequences_py.extract().expect("Error unpacking Python object to Rust");

    let mut vec_length= utils::get_length(&sequences, pad_length);
    vec_length= vec_length.div_ceil(window);
    let cpu_to_use= utils::check_nb_cpus(n_jobs);

    let mut final_array= Array3::<i64>::zeros((sequences.len(), vec_length, 3));


    final_array= py.allow_threads(move || multithreads(sequences, pad_type, final_array, window, cpu_to_use));

    final_array.to_pyarray(py)


}
