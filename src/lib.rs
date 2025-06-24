
pub mod onehot;
pub mod kmers;
pub mod random_sequence;
pub mod tfidf;
pub mod real;
pub mod utils;
pub mod cross;
pub mod zcurve;
pub mod chaos_game;
pub mod eiip;
pub mod dna_walk;
pub mod fickett;
pub mod atomic_number;



use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyModule;
use pyo3::PyResult;


use onehot::*;
use kmers::*;
use random_sequence::*;
use tfidf::*;
use real::*;
use cross::*;
use zcurve::*;
use chaos_game::*;
use eiip::*;
use dna_walk::*;
use fickett::*;
use atomic_number::*;





#[pymodule]
fn dna_parser(_py: Python<'_>, m: &Bound<PyModule>)-> PyResult<()> {

    m.add_function(wrap_pyfunction!(onehot_encoding_rust,m)?)?;
    m.add_function(wrap_pyfunction!(make_kmers,m)?)?;
    m.add_function(wrap_pyfunction!(random_seq_rust,m)?)?;
    m.add_function(wrap_pyfunction!(real_encoding_rust,m)?)?;
    m.add_function(wrap_pyfunction!(cross_encoding_rust,m)?)?;
    m.add_function(wrap_pyfunction!(zcurve_encoding_rust,m)?)?;
    m.add_function(wrap_pyfunction!(chaos_encoding_rust,m)?)?;
    m.add_function(wrap_pyfunction!(eiip_encoding_rust,m)?)?;
    m.add_function(wrap_pyfunction!(dna_walk_rust,m)?)?;
    m.add_function(wrap_pyfunction!(fickett_score_rust,m)?)?;
    m.add_function(wrap_pyfunction!(atomic_encoding_rust,m)?)?;
    m.add_function(wrap_pyfunction!(fit_transform_rust,m)?)?;
    m.add_function(wrap_pyfunction!(fit_rust,m)?)?;
    m.add_function(wrap_pyfunction!(transform_rust,m)?)?;
    


    Ok(())
}



#[cfg(test)]
mod tests {

}