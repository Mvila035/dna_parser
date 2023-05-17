use numpy::ndarray::ArrayView;
use numpy::ndarray::Array2;
use numpy::IntoPyArray;
use pyo3::types::PyList;
use pyo3::prelude::*;

use std::sync::Mutex;
use std::thread;
use num_cpus;




pub fn onehot_after(sequence: &str, ncols: usize) -> Array2<i8>{

    /// Returns an Array2<i8> which is the onehot encoding representation of the genomic sequence
    ///
    /// this function iterates on the sequence to encode it.


    let mut vec= Array2::<i8>::zeros((ncols, 4));


    for (index,charac) in sequence.chars().enumerate() {

        if index == ncols {
            break
        }

        match charac {

            'A' => vec.row_mut(index).assign(&ArrayView::from(&[ 1,  0,  0,  0])),
            'C' => vec.row_mut(index).assign(&ArrayView::from(&[ 0,  1,  0,  0])),
            'G' => vec.row_mut(index).assign(&ArrayView::from(&[ 0,  0,  1,  0])),
            'T' => vec.row_mut(index).assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            'U' => vec.row_mut(index).assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            'a' => vec.row_mut(index).assign(&ArrayView::from(&[ 1,  0,  0,  0])),
            'c' => vec.row_mut(index).assign(&ArrayView::from(&[ 0,  1,  0,  0])),
            'g' => vec.row_mut(index).assign(&ArrayView::from(&[ 0,  0,  1,  0])),
            't' => vec.row_mut(index).assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            'u' => vec.row_mut(index).assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            _ => vec.row_mut(index).assign(&ArrayView::from(&[ 0,  0,  0,  0])),
        }
    }

    vec  
}



pub fn onehot_before(sequence: &str, ncols: usize) -> Array2<i8>{

    /// Returns an Array2<i8> which is the onehot encoding representation of the genomic sequence
    ///
    /// this function iterates backward on the sequence to encode it and to pad/trim at the beginning of the sequence

    let mut vec= Array2::<i8>::zeros((ncols, 4));


    for (index , charac) in sequence.chars().rev().enumerate(){

        if index == ncols {
            break
        }

        //add value from the end of the vec
        let vec_index= ncols-1-index;

        match charac {

            'A' => vec.row_mut(vec_index).assign(&ArrayView::from(&[ 1,  0,  0,  0])),
            'C' => vec.row_mut(vec_index).assign(&ArrayView::from(&[ 0,  1,  0,  0])),
            'G' => vec.row_mut(vec_index).assign(&ArrayView::from(&[ 0,  0,  1,  0])),
            'T' => vec.row_mut(vec_index).assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            'U' => vec.row_mut(vec_index).assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            'a' => vec.row_mut(vec_index).assign(&ArrayView::from(&[ 1,  0,  0,  0])),
            'c' => vec.row_mut(vec_index).assign(&ArrayView::from(&[ 0,  1,  0,  0])),
            'g' => vec.row_mut(vec_index).assign(&ArrayView::from(&[ 0,  0,  1,  0])),
            't' => vec.row_mut(vec_index).assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            'u' => vec.row_mut(vec_index).assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            _ => vec.row_mut(vec_index).assign(&ArrayView::from(&[ 0,  0,  0,  0])),
        }
    }

    vec  
}




fn get_length(sequences: &Vec<&str>, pad_length: i128) -> usize {

    /// Returns a usize representing the length that the sequences should have after padding/trimming
    /// or 0 for no padding/trimming

    /// if pad_length = -1 searches for the shortest sequence
    /// if pad_length= -2 searches for the longest sequence



    let mut length= sequences[0].len();


    if pad_length < -2 {

        panic!("Invalid padding length. Here are the available options:
         -2 to pad to longest sequence; 
         -1 to pad to the shortest sequence;
          0 for no padding;
          any positive number to choose the maximum length you want your sequences to be.");
        
    }

    else if pad_length == 0 {

        length= 0;
    }
    

    //padd to shortest
    else if pad_length == -1 {

        for seq in sequences.iter() {

            if seq.len() < length {
            
            length = seq.len();

            }
        }
    }

    //pad to longest
    else if pad_length == -2 {

        for seq in sequences.iter() {

            if seq.len() > length {
            
                length = seq.len();

            }
        }   
    }


    else {

        length= pad_length as usize;
    }

    return length
}



fn check_nb_cpus(n_jobs: i16) -> usize {

    /// Returns the number of threads to use.
    /// 
    /// if n_jobs = 0; number of threads = number of cpus

    let nb_cpus;

    if n_jobs == 0 {

        nb_cpus= num_cpus::get_physical();
    }

    else if n_jobs < 0 {

       panic!("Cannot have a negative number of cpu. Use 0 to use every cpus or input the number of desired cpus")

    }

    else {

        nb_cpus= n_jobs as usize;
    }

    nb_cpus
}




fn encode_chunks(chunk: &[&str], pad_type: &str, vec_length: usize ) -> Vec<Array2<i8>> {

    /// Returns the onehot encodings in a Vec for the sequences passed to this fucntion.
    ///
    /// this function parse the type and length of padding for the encoding 

    let mut encoded_sequences= Vec::new();

    if pad_type== "after" && vec_length > 0 {

        for seq in chunk {

            let encoding= onehot_after(seq, vec_length);
            
            encoded_sequences.push(encoding);
            
        }
    }

    else if pad_type == "before" && vec_length > 0 {
        
        for seq in chunk{

            let encoding= onehot_before(seq, vec_length);
            
            encoded_sequences.push(encoding);
            
        }
    }

    else if pad_type== "after" && vec_length == 0 {

        for seq in chunk.into_iter(){

            let seq_len = seq.len();
            let encoding= onehot_after(seq, seq_len);
            
            encoded_sequences.push(encoding);
            
        }
    }

    else if pad_type== "before" && vec_length == 0 {

        for seq in chunk.into_iter(){

            let seq_len = seq.len();
            let encoding= onehot_before(seq, seq_len);
            
            encoded_sequences.push(encoding);
            
        }
    }


    else {

        panic!("The only 2 options for the type of padding are 'before' and 'after'.")
    }

    encoded_sequences
}


fn multithreads(sequences: Vec<&str>, pad_type: &str, vec_length: usize, nb_cpus: usize) -> Vec<(usize, Vec<Array2<i8>>)> {

    /// Returns a Vec of tuples (usize, Vec<Array2<i8>>)
    ///
    /// This function splits the sequences to encode and distributes them to different threads. 
    /// the usize is used to keep the order of sequences and the Vec<Array2<i8>> represent the onehot encodings of the genomic sequences


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



#[allow(unused_must_use)]
#[pyfunction]
pub fn onehot_encoding_rust<'pyt>(py:  Python <'pyt>, sequences: Vec<&str>, pad_type: &str, pad_length: i128, n_jobs: i16 ) ->  &'pyt PyList{

    /// Returns a PyList of Numpy i8 2D array to Python
    ///
    /// # Arguments
    /// * `py` - Python GIL token (used to acquire the GIL)
    /// * `sequences` - Vec of &str representing the sequences to encode
    /// * `pad_type` - &str indicating to padd (or trim) "before" or "after" the sequences
    /// * `pad_length` - -2 to pad according to the longest sequence, -1 to trim to the shortest sequence, 0 for no paddding, any positive number for a fixed length.
    /// * `n_jobs` - number of threads to use. 0 to use every cpu

    let vec_length= get_length(&sequences, pad_length);
    let cpu_to_use= check_nb_cpus(n_jobs);

    let py_list= PyList::empty(py);

    let results=py.allow_threads(move || multithreads(sequences, pad_type, vec_length, cpu_to_use));

  
    for (_index, sequences ) in results {

        for seq in sequences {

            py_list.append(seq.into_pyarray(py));
        }
    }
    
    
    py_list
}

