
pub mod onehot;
pub mod kmers;
pub mod random_sequence;
pub mod tfidf;
pub mod ordinal;
pub mod fasta_files;




use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyModule;
use pyo3::PyResult;


use onehot::*;
use kmers::*;
use random_sequence::*;
use tfidf::*;
use ordinal::*;
use fasta_files::*;





#[pymodule]
fn dna_parser(_py: Python<'_>, m: &PyModule)-> PyResult<()> {

    m.add_function(wrap_pyfunction!(onehot_encoding_rust,m)?)?;
    m.add_function(wrap_pyfunction!(make_kmers,m)?)?;
    m.add_function(wrap_pyfunction!(random_seq_rust,m)?)?;
    m.add_function(wrap_pyfunction!(tfidf_encoding,m)?)?;
    m.add_function(wrap_pyfunction!(ordinal_encoding_rust,m)?)?;
    m.add_function(wrap_pyfunction!(load_fasta,m)?)?;
    m.add_function(wrap_pyfunction!(seq_from_fasta,m)?)?;
    m.add_function(wrap_pyfunction!(metadata_from_fasta,m)?)?;

    Ok(())
}



#[cfg(test)]
mod tests {

}