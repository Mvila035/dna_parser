
use rand::seq::IteratorRandom;
use pyo3::prelude::*;
use std::sync::Mutex;
use std::thread;
use std::sync::Arc;

fn generate_dna_seq(length: i64) -> String {

    const NT_DNA: &str = "atgc";
    let mut rng = rand::thread_rng();
    let mut seq= String::from("");

    for _ in 0..length {

        seq.push( NT_DNA.chars().choose(&mut rng).unwrap() );
        
    }

    seq
}

fn generate_rna_seq(length: i64) -> String{

    const NT_RNA: &str = "aucg";
    let mut rng = rand::thread_rng();
    let mut seq= String::from("");

    for _ in 0..length {

        seq.push( NT_RNA.chars().choose(&mut rng).unwrap() );
        
    }

    seq

}

fn generate_aa_seq(length: i64) -> String{

    const AA: &str = "GAVCPLIMWFSTYNQKRHDE";
    let mut rng = rand::thread_rng();
    let mut seq= String::from("");

    for _ in 0..length {

        seq.push( AA.chars().choose(&mut rng).unwrap() );
        
    }

    seq

}

fn parse_type_seq(length: i64, nb_of_seq: i64, seq_type: &str)-> Vec<String> {

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



#[pyfunction]
pub fn random_seq_rust(length: i64, nb_of_seq: i64, seq_type: &str, n_jobs: i16) -> Vec<String>{

    let results= Arc::new(Mutex::new(Vec::new()));

    let mut nb_of_threads_left= n_jobs;

    let mut nb_seq_left= nb_of_seq;

    let mut seq_per_thread = nb_seq_left/(n_jobs as i64);

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


