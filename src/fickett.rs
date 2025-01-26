use numpy::ndarray::{ArrayBase, ViewRepr};
use numpy::ndarray::{Axis, Dim};
use numpy::ToPyArray;
use numpy::ndarray::Array1;
use numpy::PyArray1;
use pyo3::prelude::*;
use pyo3::types::PyList;
use phf::phf_map;
use std::thread;
use crate::utils;


static POSITION_PROB: phf::Map<&'static str, [&'static f32; 10]> = phf_map! {
    "A"=> [&0.94, &0.68, &0.84, &0.93, &0.58, &0.68, &0.45, &0.34, &0.20, &0.22],
    "C"=> [&0.80, &0.70, &0.70, &0.81, &0.66, &0.48, &0.51, &0.33, &0.30, &0.23],
    "G"=> [&0.90, &0.88, &0.74, &0.64, &0.53, &0.48, &0.27, &0.16, &0.08, &0.08],
    "T"=> [&0.97, &0.97, &0.91, &0.68, &0.69, &0.44, &0.54, &0.20, &0.09, &0.09],
};


static POSITION_WEIGHT: phf::Map<&'static str, f32 >= phf_map! {
    "A"=> 0.26,
    "C"=> 0.18,
    "G"=> 0.31,
    "T"=> 0.33,
};

const POSITION_PARA: [&f32; 10] = [&1.9, &1.8, &1.7, &1.6, &1.5, &1.4, &1.3, &1.2, &1.1, &0.0];

static CONTENT_PROB: phf::Map<&'static str, [&'static f32; 10]> = phf_map! {
    "A"=> [&0.28, &0.49, &0.44, &0.55, &0.62, &0.49, &0.67, &0.65, &0.81, &0.21],
    "C"=> [&0.82, &0.64, &0.51, &0.64, &0.59, &0.59, &0.43, &0.44, &0.39, &0.31],
    "G"=> [&0.40, &0.54, &0.47, &0.64, &0.64, &0.73, &0.41, &0.41, &0.33, &0.29],
    "T"=> [&0.28, &0.24, &0.39, &0.40, &0.55, &0.75, &0.56, &0.69, &0.51, &0.58],
};

static CONTENT_WEIGHT: phf::Map<&'static str, f32> = phf_map! {
    "A"=> 0.11,
    "C"=> 0.12,
    "G"=> 0.15,
    "T"=> 0.14,
};

const CONTENT_PARA: [&f32; 10] = [&0.33, &0.31, &0.29, &0.27, &0.25, &0.23, &0.21, &0.19, &0.17, &0.0];

fn nt_counts(sequence: &str, a: &mut[i32;3], c: &mut[i32;3],  g: &mut[i32;3], t: &mut[i32;3]) {

    let mut position= 0;

    for nt in sequence.chars() {

        if position > 2 {
            position= 0;
        }
    

        match nt {
            'A' => a[position] += 1,
            'C' => c[position] += 1,
            'G' => g[position] += 1,
            'T' => t[position] += 1,
            'U' => t[position] += 1,
            'a' => a[position] += 1,
            'c' => c[position] += 1,
            'g' => g[position] += 1,
            't' => t[position] += 1,
            'u' => t[position] += 1,
            _=> {}
        }
        
        position += 1;
    }

}

fn get_position_prob(nt_value: f32, nt_type: &str) -> f32 {

    for (index, value) in POSITION_PARA.iter().enumerate() {
        if nt_value >= **value {

            let position_vec= POSITION_PROB.get(nt_type).unwrap();
            let position_prob= position_vec[index] * POSITION_WEIGHT.get(nt_type).unwrap();
            
            return position_prob
        }
    }

    panic!("Error computing Fickett score. Position Parameter in smaller than 0.")
}

fn get_content_prob(nt_value: f32, nt_type: &str) -> f32 {

    for (index, value) in CONTENT_PARA.iter().enumerate() {
        if nt_value >= **value {

            let content_vec= CONTENT_PROB.get(nt_type).unwrap();
            let content_prob= content_vec[index] * CONTENT_WEIGHT.get(nt_type).unwrap();
            
            return content_prob
        }
    }

    panic!("Error computing Fickett score. Content Parameter in smaller than 0.")
}



fn fickett_score(sequence: &str, mut score: ArrayBase<ViewRepr<&mut f32>, Dim<[usize; 0]>>){
    
    let seq_len= sequence.len();
    let mut a_counts= [0,0,0];
    let mut c_counts= [0,0,0];
    let mut g_counts= [0,0,0];
    let mut t_counts= [0,0,0];

    nt_counts(sequence, &mut a_counts, &mut c_counts, &mut g_counts, &mut t_counts);

    let a_content= a_counts.iter().sum::<i32>() as f32 / seq_len as f32;
    let c_content= c_counts.iter().sum::<i32>() as f32 / seq_len as f32;
    let g_content= g_counts.iter().sum::<i32>() as f32 / seq_len as f32;
    let t_content= t_counts.iter().sum::<i32>() as f32 / seq_len as f32;

    let a_max= a_counts.iter().max().unwrap();
    let a_min= a_counts.iter().min().unwrap();
    let c_max= c_counts.iter().max().unwrap();
    let c_min= c_counts.iter().min().unwrap();
    let g_max= g_counts.iter().max().unwrap();
    let g_min= g_counts.iter().min().unwrap();
    let t_max= t_counts.iter().max().unwrap();
    let t_min= t_counts.iter().min().unwrap();
    
    let a_position= *a_max as f32/ (a_min +1) as f32;
    let c_position= *c_max as f32/ (c_min +1) as f32;
    let g_position= *g_max as f32/ (g_min +1) as f32;
    let t_position= *t_max as f32/ (t_min +1) as f32;


    score += get_content_prob(a_content,"A");
    score += get_content_prob(c_content,"C");
    score += get_content_prob(g_content,"G");
    score += get_content_prob(t_content,"T");

    score += get_position_prob(a_position,"A");
    score += get_position_prob(c_position,"C");
    score += get_position_prob(g_position,"G");
    score += get_position_prob(t_position,"T");
    


}

fn encode_chunks(chunk: &[String], mut array: ArrayBase<ViewRepr<&mut f32>, Dim<[usize; 1]>> ) {

    for (seq, sub_array) in chunk.iter().zip(array.axis_iter_mut(Axis(0))) {

        fickett_score(seq, sub_array);
        
    }


}



fn multithreads(sequences: Vec<String>, mut array: Array1<f32>, nb_cpus: usize) -> Array1<f32> {


    //determine size of chunks based on number of threads and add 1 to be sure 
    //to have a number of chunks egal to nb of cpus and not superior
    let seq_len= sequences.len();
    let slice_len= (seq_len/ nb_cpus) + 1;

    
    thread::scope(|s|{

        
        for (chunk_seq,array_slice ) in sequences.chunks(slice_len).zip(array.axis_chunks_iter_mut(Axis(0), slice_len)){

            s.spawn( move || {
                
                encode_chunks(chunk_seq, array_slice);
                
            });

        }

    });

// ####################################### end of threads #####################################

    array

}



#[allow(unused_must_use)]
#[pyfunction]
pub fn fickett_score_rust<'pyt>(py:  Python <'pyt>, sequences_py: &Bound<'pyt, PyList>, n_jobs: i16 ) -> Bound<'pyt, PyArray1<f32>> {
    
    let sequences: Vec<String> = sequences_py.extract().expect("Error unpacking Python object to Rust");
    let mut final_array= Array1::<f32>::zeros(sequences.len());

    let cpu_to_use= utils::check_nb_cpus(n_jobs);
    final_array= py.allow_threads(move || multithreads(sequences, final_array, cpu_to_use));

    final_array.to_pyarray(py)
   
}