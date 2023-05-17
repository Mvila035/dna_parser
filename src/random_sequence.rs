
use rand::seq::IteratorRandom;
use pyo3::prelude::*;
use std::sync::Mutex;
use std::thread;
use std::sync::Arc;
use num_cpus;

fn generate_dna_seq(length: i64) -> String {

    /// Returns a randomly generate string representing a DNA sequence

    const NT_DNA: &str = "atgc";
    let mut rng = rand::thread_rng();
    let mut seq= String::from("");

    for _ in 0..length {

        seq.push( NT_DNA.chars().choose(&mut rng).unwrap() );
        
    }

    seq
}

fn generate_rna_seq(length: i64) -> String{

    /// Returns a randomly generate string representing a RNA sequence

    const NT_RNA: &str = "aucg";
    let mut rng = rand::thread_rng();
    let mut seq= String::from("");

    for _ in 0..length {

        seq.push( NT_RNA.chars().choose(&mut rng).unwrap() );
        
    }

    seq

}

fn generate_aa_seq(length: i64) -> String{

    /// Returns a randomly generate string representing an Amino Acid sequence

    const AA: &str = "GAVCPLIMWFSTYNQKRHDE";
    let mut rng = rand::thread_rng();
    let mut seq= String::from("");

    for _ in 0..length {

        seq.push( AA.chars().choose(&mut rng).unwrap() );
        
    }

    seq

}

fn parse_type_seq(length: i64, nb_of_seq: i64, seq_type: &str)-> Vec<String> {

    /// Returns a Vec with the randomly generated sequences
    ///
    /// this function parses the type of sequences to generate and calls the appropriate function to generate them.

    let mut vec_of_seq= Vec::new();

    match seq_type.to_lowercase().as_str() {

        "dna" => for _i in 0..nb_of_seq {
            vec_of_seq.push(generate_dna_seq(length))
        },

        "rna" => for _i in 0..nb_of_seq {
            vec_of_seq.push(generate_rna_seq(length))
        },

        "aa" => for _i in 0..nb_of_seq {
            vec_of_seq.push(generate_aa_seq(length))
        },

        _ => panic!()

    }

    vec_of_seq
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




#[pyfunction]
pub fn random_seq_rust(length: i64, nb_of_seq: i64, seq_type: &str, n_jobs: i16) -> Vec<String>{

    /// Returns a Vec of strings representing a sequences
    ///
    /// This functions split the sequences to generate in different threads and collects them.
    ///
    /// # Arguments
    /// * `length` - Length of the sequences to generate
    /// * `nb_of_seq` - number of sequences to generate and store in the Vec
    /// * `seq_type` - either "dna", "rna" or "aa" (for amino acid)
    /// * `n_jobs` - number of threads to use. 0 to use every cpu in your machine

    let cpu_to_use= check_nb_cpus(n_jobs);

    let results= Arc::new(Mutex::new(Vec::new()));

    let mut nb_of_threads_left= cpu_to_use;

    let mut nb_seq_left= nb_of_seq;

    let mut seq_per_thread = nb_seq_left/(cpu_to_use as i64);

    thread::scope(|s| {


        while nb_of_threads_left > 0 {

            if nb_of_threads_left == 1 {

                seq_per_thread = nb_seq_left;
            }

            else {
                nb_seq_left = nb_seq_left - seq_per_thread;
            }


            s.spawn({ let results= results.clone(); 
                 move|| {
                
                let  vec_of_seq = parse_type_seq(length, seq_per_thread, seq_type );

                results.lock().unwrap().push(vec_of_seq);
                 }
            });

            nb_of_threads_left = nb_of_threads_left -1;
        }

    });

    let getter= results.lock().unwrap();

    let mut vec_to_return= Vec::new();

    for i in getter.iter().flatten() {

        vec_to_return.push(i.clone());
    }

    vec_to_return
    
}


