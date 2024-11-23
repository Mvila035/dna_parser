
pub mod onehot;
pub mod kmers;
pub mod random_sequence;
pub mod tfidf;
pub mod ordinal;
pub mod fasta_files;
pub mod utils;
pub mod cross;
pub mod zcurve;
pub mod chaos_game;
pub mod eiip;
pub mod dna_walk;


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
use cross::*;
use zcurve::*;
use chaos_game::*;
use eiip::*;
use dna_walk::*;




#[pymodule]
fn dna_parser(_py: Python<'_>, m: &Bound<PyModule>)-> PyResult<()> {

    m.add_function(wrap_pyfunction!(onehot_encoding_rust,m)?)?;
    m.add_function(wrap_pyfunction!(make_kmers,m)?)?;
    m.add_function(wrap_pyfunction!(random_seq_rust,m)?)?;
    m.add_function(wrap_pyfunction!(map_vocabulary_rust,m)?)?;
    m.add_function(wrap_pyfunction!(transform_idf_rust,m)?)?;
    m.add_function(wrap_pyfunction!(fit_idf_rust,m)?)?;
    m.add_function(wrap_pyfunction!(ordinal_encoding_rust,m)?)?;
    m.add_function(wrap_pyfunction!(load_fasta,m)?)?;
    m.add_function(wrap_pyfunction!(seq_from_fasta,m)?)?;
    m.add_function(wrap_pyfunction!(metadata_from_fasta,m)?)?;
    m.add_function(wrap_pyfunction!(cross_encoding_rust,m)?)?;
    m.add_function(wrap_pyfunction!(zcurve_encoding_rust,m)?)?;
    m.add_function(wrap_pyfunction!(chaos_encoding_rust,m)?)?;
    m.add_function(wrap_pyfunction!(eiip_encoding_rust,m)?)?;
    m.add_function(wrap_pyfunction!(dna_walk_rust,m)?)?;


    Ok(())
}



#[cfg(test)]
mod tests {

}