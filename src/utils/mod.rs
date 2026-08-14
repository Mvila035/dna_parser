use num_cpus;
use std::cmp::Ordering;
use rayon::ThreadPoolBuildError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList, PyString};
use pyo3::exceptions::PyTypeError;

fn is_biopython_seq(obj: &Bound<PyAny>) -> bool {
    obj.get_type()
        .getattr("__module__")
        .and_then(|m| m.extract::<String>())
        .map(|m| m == "Bio.Seq")
        .unwrap_or(false)
}

pub fn extract_seq_bytes(obj: &Bound<PyAny>) -> PyResult<Vec<u8>> {
    if let Ok(s) = obj.cast::<PyString>() {
        return Ok(s.to_string().into_bytes());
    }
    if let Ok(b) = obj.cast::<PyBytes>() {
        return Ok(b.as_bytes().to_vec());
    }
    if let Ok(seq_attr) = obj.getattr("seq") {
        return extract_seq_bytes(&seq_attr);
    }

    if let Ok(seq_attr) = obj.getattr("sequence") {
        return extract_seq_bytes(&seq_attr);
    }

    if is_biopython_seq(obj) {
        let s = obj.str()?;
        return Ok(s.to_string().into_bytes());
    }

    Err(PyTypeError::new_err(
        "Each sequence must be a str, bytes/bytearray, a Biopython Seq/MutableSeq, \
         or an object with a `.seq` attribute (e.g. a needletail SequenceRecord, Biopython SeqRecord or
         Pysam FastxRecord).",
    ))
}
 
/// Extracts a whole `PyList` of mixed str/bytes/needletail-record entries
/// into `Vec<Vec<u8>>`, ready for byte-LUT encoding.
pub fn extract_all_sequences(sequences_py: &Bound<PyAny>) -> PyResult<Vec<Vec<u8>>> {
    if let Ok(mut records) = sequences_py.extract::<PyRefMut<crate::fastx::SequenceReader>>() {
        return records.next_bytes();
    }
 
    if let Ok(list) = sequences_py.cast::<PyList>() {
        return list.iter().map(|item| extract_seq_bytes(&item)).collect();
    }
 
    let mut out = Vec::new();
    for item in sequences_py.try_iter()? {
        out.push(extract_seq_bytes(&item?)?);
    }
    Ok(out)
}


pub fn get_length_vec(sequences: &Vec<Vec<u8>>, pad_length: i128) -> usize {


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

    length
}

/// Returns a usize representing the length that the sequences should have after padding/trimming
/// or 0 for no padding/trimming
///
/// if pad_length = -1 searches for the shortest sequence
/// if pad_length = -2 searches for the longest sequence
pub fn get_length(sequences: &[String], pad_length: i128) -> usize {


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

    length
}


/// Returns the number of threads to use.
/// 
/// if n_jobs = 0; number of threads = number of cpus
pub fn check_nb_cpus(n_jobs: i16) -> usize {

    match n_jobs.cmp(&0_i16) {

        Ordering::Equal => num_cpus::get_physical(),
        Ordering::Less => panic!("Cannot have a negative number of cpu. Use 0 to use every cpus or input the number of desired cpus"),
        Ordering::Greater => n_jobs as usize,
    }

}

pub fn create_pool(num_threads: usize) -> Result<rayon::ThreadPool, ThreadPoolBuildError> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
 }