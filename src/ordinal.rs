use numpy::ndarray::{Dim, OwnedRepr, ArrayBase,Array1};
use numpy::{IntoPyArray};
use std::sync::Mutex;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::thread;
use num_cpus;




pub fn ordinal_after(sequence: &str, ncols: usize) -> Array1<f32>{


    let mut vec= Array1::<f32>::zeros(ncols);


    for (index,charac) in sequence.chars().enumerate() {

        if index == ncols {
            break
        }

        match charac {

            'A' => vec[index]= 0.25,
            'C' => vec[index]= 0.50,
            'G' => vec[index]= 0.75,
            'T' => vec[index]= 1.0,
            'U' => vec[index]= 1.0,
            'a' => vec[index]= 0.25,
            'c' => vec[index]= 0.50,
            'g' => vec[index]= 0.75,
            't' => vec[index]= 1.0,
            'u' => vec[index]= 1.0,
            _ => vec[index]= 0.0

        }       
    }

    vec  
}



pub fn ordinal_before(sequence: &str, ncols: usize) -> Array1<f32>{ 

    let mut vec= Array1::<f32>::zeros(ncols);

    for (index, charac) in sequence.chars().rev().enumerate() {

        if index == ncols {
            break
        }

        //add value from the end of the vec
        let vec_index= ncols-1-index;

        match charac {

            'A' => vec[vec_index]= 0.25,
            'C' => vec[vec_index]= 0.50,
            'G' => vec[vec_index]= 0.75,
            'T' => vec[vec_index]= 1.0,
            'U' => vec[vec_index]= 1.0,
            'a' => vec[vec_index]= 0.25,
            'c' => vec[vec_index]= 0.50,
            'g' => vec[vec_index]= 0.75,
            't' => vec[vec_index]= 1.0,
            'u' => vec[vec_index]= 1.0,
            _ => vec[vec_index]= 0.0

        }       
    }

    vec
}




fn get_length(sequences: &Vec<&str>, pad_length: i128) -> usize {

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




fn encode_chunks(chunk: &[&str], pad_type: &str, vec_length: usize ) -> Vec<ArrayBase<OwnedRepr<f32>, Dim<[usize; 1]>>> {

    let mut encoded_sequences= Vec::new();

    if pad_type== "after" && vec_length > 0 {

        for seq in chunk {

            let encoding= ordinal_after(seq, vec_length);
            
            encoded_sequences.push(encoding);
            
        }
    }

    else if pad_type == "before" && vec_length > 0 {
        
        for seq in chunk{

            let encoding= ordinal_before(seq, vec_length);
            
            encoded_sequences.push(encoding);
            
        }
    }

    else if pad_type== "after" && vec_length == 0 {

        for seq in chunk.into_iter(){

            let seq_len = seq.len();
            let encoding= ordinal_after(seq, seq_len);
            
            encoded_sequences.push(encoding);
            
        }
    }

    else if pad_type== "before" && vec_length == 0 {

        for seq in chunk.into_iter(){

            let seq_len = seq.len();
            let encoding= ordinal_before(seq, seq_len);
            
            encoded_sequences.push(encoding);
            
        }
    }


    else {

        panic!("The only 2 options for the type of padding are 'before' and 'after'.")
    }

    encoded_sequences
}


fn multithreads(sequences: Vec<&str>, pad_type: &str, vec_length: usize, nb_cpus: usize) -> Vec<(usize, Vec<ArrayBase<OwnedRepr<f32>, Dim<[usize; 1]>>>)> {


    //determine size of chunks based on number of threads and add 1 to be sure 
    //to have a number of chunks egal to nb of cpus and not superior
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
pub fn ordinal_encoding_rust<'pyt>(py:  Python <'pyt>, sequences: Vec<&str>, pad_type: &str, pad_length: i128, n_jobs: i16 ) ->  &'pyt PyList{

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