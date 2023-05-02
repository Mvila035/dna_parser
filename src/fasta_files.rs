use std::fs::File;
use pyo3::prelude::*;
use std::io::{prelude::*, BufReader};


#[pyfunction]
pub fn seq_from_fasta(file_path: &str) ->  Vec<String> {

    let f = File::open(file_path).expect("Unable to open file");
    let f = BufReader::new(f);

    let mut sequences = Vec::new();
    let mut seq= String::from("");

    for line in f.lines(){

        let line = line.expect("Unable to read line");

        if line.chars().nth(0).unwrap() == '>' {
            
            if seq.len() > 0 {

                sequences.push(seq);
                seq= String::from("");
            }
        }

        else {

            seq.push_str(line.trim());

        }

    }

    if seq.len() > 0 {
        sequences.push(seq);
    }

    sequences

}

#[pyfunction]
pub fn metadata_from_fasta(file_path: &str) ->  Vec<String> {

    let f = File::open(file_path).expect("Unable to open file");
    let f = BufReader::new(f);

    let mut metadata = Vec::new();

    for line in f.lines() {

        let line = line.expect("Unable to read line");

        if line.chars().nth(0).unwrap() == '>' {

            metadata.push(line)
        }
    }

    metadata 
}

#[pyfunction]
pub fn load_fasta(file_path: &str) ->  Vec<(String,String)>  {

    let f = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(f);

    let mut metadata_and_seq = Vec::new();
    let mut metadata= String::from("");
    let mut seq= String::from("");

    for line in reader.lines() {

        let line = line.expect("Unable to read line");

        if line.chars().nth(0).unwrap() == '>' {

            if seq.len() > 0 {

                metadata_and_seq.push((metadata, seq));
                seq= String::from("");
            }

            metadata= String::from(line);
        }

        else {

            seq.push_str(line.trim())

        }

    }

    if seq.len() > 0 {

        metadata_and_seq.push((metadata, seq));
    }

    metadata_and_seq


}
