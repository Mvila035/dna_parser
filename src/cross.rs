
use numpy::ndarray::ArrayView;
use numpy::ndarray::Array2;
use numpy::IntoPyArray;
use pyo3::types::PyList;
use pyo3::prelude::*;

use std::sync::Mutex;
use std::thread;
use crate::utils;




/// Returns an Array2<i8> which is the cross encoding representation of the genomic sequence
///
/// this function iterates on the sequence to encode it.
pub fn cross_after(sequence: &str, ncols: usize) -> Array2<i8>{

    let mut vec= Array2::<i8>::zeros((ncols, 2));


    for (index,charac) in sequence.chars().enumerate() {

        if index == ncols {
            break
        }

        match charac {

            'A' => vec.row_mut(index).assign(&ArrayView::from(&[0,-1])),
            'C' => vec.row_mut(index).assign(&ArrayView::from(&[-1,0])),
            'G' => vec.row_mut(index).assign(&ArrayView::from(&[1,0])),
            'T' => vec.row_mut(index).assign(&ArrayView::from(&[0,1])),
            'U' => vec.row_mut(index).assign(&ArrayView::from(&[0,1])),
            'a' => vec.row_mut(index).assign(&ArrayView::from(&[0,-1])),
            'c' => vec.row_mut(index).assign(&ArrayView::from(&[-1,0])),
            'g' => vec.row_mut(index).assign(&ArrayView::from(&[1,0])),
            't' => vec.row_mut(index).assign(&ArrayView::from(&[0,1])),
            'u' => vec.row_mut(index).assign(&ArrayView::from(&[0,1])),
            _ => vec.row_mut(index).assign(&ArrayView::from(&[0,0])),
        }
    }

    vec  
}


/// Returns an Array2<i8> which is the cross encoding representation of the genomic sequence
///
/// this function iterates backward on the sequence to encode it and to pad/trim at the beginning of the sequence
pub fn cross_before(sequence: &str, ncols: usize) -> Array2<i8>{

    let mut vec= Array2::<i8>::zeros((ncols, 2));


    for (index , charac) in sequence.chars().rev().enumerate(){

        if index == ncols {
            break
        }

        //add value from the end of the vec
        let vec_index= ncols-1-index;

        match charac {

            'A' => vec.row_mut(vec_index).assign(&ArrayView::from(&[0,-1])),
            'C' => vec.row_mut(vec_index).assign(&ArrayView::from(&[-1,0])),
            'G' => vec.row_mut(vec_index).assign(&ArrayView::from(&[1,0])),
            'T' => vec.row_mut(vec_index).assign(&ArrayView::from(&[0,1])),
            'U' => vec.row_mut(vec_index).assign(&ArrayView::from(&[0,1])),
            'a' => vec.row_mut(vec_index).assign(&ArrayView::from(&[0,-1])),
            'c' => vec.row_mut(vec_index).assign(&ArrayView::from(&[-1,0])),
            'g' => vec.row_mut(vec_index).assign(&ArrayView::from(&[1,0])),
            't' => vec.row_mut(vec_index).assign(&ArrayView::from(&[0,1])),
            'u' => vec.row_mut(vec_index).assign(&ArrayView::from(&[0,1])),
            _ => vec.row_mut(vec_index).assign(&ArrayView::from(&[0,0])),
        }            
        
    }

    vec  
}



/// Returns the cross encodings in a Vec for the sequences passed to this fucntion.
///
/// this function parse the type and length of padding for the encoding 
fn encode_chunks(chunk: &[&str], pad_type: &str, vec_length: usize ) -> Vec<Array2<i8>> {

    
    let mut encoded_sequences= Vec::new();

    if pad_type== "after" && vec_length > 0 {

        for seq in chunk {

            let encoding= cross_after(seq, vec_length);
            
            encoded_sequences.push(encoding);
            
        }
    }

    else if pad_type == "before" && vec_length > 0 {
        
        for seq in chunk{

            let encoding= cross_before(seq, vec_length);
            
            encoded_sequences.push(encoding);
            
        }
    }

    else if pad_type== "after" && vec_length == 0 {

        for seq in chunk.iter(){

            let seq_len = seq.len();
            let encoding= cross_after(seq, seq_len);
            
            encoded_sequences.push(encoding);
            
        }
    }

    else if pad_type== "before" && vec_length == 0 {

        for seq in chunk.iter(){

            let seq_len = seq.len();
            let encoding= cross_before(seq, seq_len);
            
            encoded_sequences.push(encoding);
            
        }
    }


    else {

        panic!("The only 2 options for the type of padding are 'before' and 'after'.")
    }

    encoded_sequences
}

/// Returns a Vec of tuples (usize, Vec<Array2<i8>>)
///
/// This function splits the sequences to encode and distributes them to different threads. 
/// the usize is used to keep the order of sequences and the Vec<Array2<i8>> represent the onehot encodings of the genomic sequences
fn multithreads(sequences: Vec<&str>, pad_type: &str, vec_length: usize, nb_cpus: usize) -> Vec<(usize, Vec<Array2<i8>>)> {

    //determine size of chunks based on number of threads and add 1 to be sure 
    //to have a number of chunks equal to nb of cpus and not superior
    let seq_len= sequences.len();
    let slice_len= (seq_len/ nb_cpus) + 1;

    let results= Mutex::new(Vec::new());

// ####################################### begining of threads #####################################
    thread::scope(|s|{

        let results= &results;
        for (index,chunk) in sequences.chunks(slice_len).enumerate(){

            s.spawn( move || {
                
                let vec_to_push= encode_chunks(chunk, pad_type, vec_length);
                results.lock().unwrap().push((index, vec_to_push));

            });

        }

    });


// ####################################### end of threads #####################################


    let mut result_vec= results.into_inner().unwrap();

    result_vec.sort_by_key(|k| k.0);

    result_vec

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
pub fn cross_encoding_rust<'pyt>(py:  Python <'pyt>, sequences: Vec<&str>, pad_type: &str, pad_length: i128, n_jobs: i16 ) ->  &'pyt PyList{


    let vec_length= utils::get_length(&sequences, pad_length);
    let cpu_to_use= utils::check_nb_cpus(n_jobs);

    let py_list= PyList::empty(py);

    let results=py.allow_threads(move || multithreads(sequences, pad_type, vec_length, cpu_to_use));

  
    for (_index, sequences ) in results {

        for seq in sequences {

            py_list.append(seq.into_pyarray(py));
        }
    }
    
    
    py_list
}

