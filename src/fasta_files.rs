use std::fs::File;
use pyo3::prelude::*;
use std::io::{prelude::*, BufReader};

//implement a struct for fasta file?

/// Returns a Vec of string where each string is one of the genomic sequences in the fasta file
/// The order in which the sequences appear in the fasta file is conserved
///
/// # Arguments
/// * `file_path` - path to the fasta file
#[pyfunction]
pub fn seq_from_fasta(file_path: &str) ->  Vec<String> {
   
    let f = File::open(file_path).expect("Unable to open file");
    let f = BufReader::new(f);

    let mut sequences = Vec::new();
    let mut seq= String::from("");

    for line in f.lines(){

        let line = line.expect("Unable to read line");

        if line.starts_with('>') {
            
            if !seq.is_empty() {

                sequences.push(seq);
                seq= String::from("");
            }
        }

        else {

            seq.push_str(line.trim());

        }

    }

    if !seq.is_empty() {
        sequences.push(seq);
    }

    sequences

}

/// Returns a Vec of string where each string is the metadata for one genomic sequence
/// The order in which the sequences metadata appear in the fasta file is conserved
///
/// # Arguments
/// * `file_path` - path to the fasta file
#[pyfunction]
pub fn metadata_from_fasta(file_path: &str) ->  Vec<String> {

    let f = File::open(file_path).expect("Unable to open file");
    let f = BufReader::new(f);

    let mut metadata = Vec::new();

    for line in f.lines() {

        let line = line.expect("Unable to read line");

        if line.starts_with('>') {

            metadata.push(line)
        }
    }

    metadata 
}


/// Returns a Vec of tuples each containing 2 strings. The first string is the metadata and the second one is the associated genomic sequence
/// The order in which the metadatas and sequences appear in the fasta file is conserved
///
/// # Arguments
/// * `file_path` - path to the fasta files
#[pyfunction]
pub fn load_fasta(file_path: &str) ->  Vec<(String,String)>  {

    let f = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(f);

    let mut metadata_and_seq = Vec::new();
    let mut metadata= String::from("");
    let mut seq= String::from("");

    for line in reader.lines() {

        let line = line.expect("Unable to read line");

        if line.starts_with('>') {

            if !seq.is_empty() {

                metadata_and_seq.push((metadata, seq));
                seq= String::from("");
            }

            metadata= line;
        }

        else {

            seq.push_str(line.trim())

        }

    }

    if !seq.is_empty() {

        metadata_and_seq.push((metadata, seq));
    }

    metadata_and_seq


}
