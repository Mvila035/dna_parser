use ndarray::ArrayViewMut1;
use numpy::ndarray::{Array1, Array2, Axis};
use numpy::IntoPyArray;
use pyo3::prelude::*;
use std::collections::HashMap;
use pyo3::types::{PyList};
use rayon::prelude::*;
use pyo3::exceptions::PyValueError;
use crate::utils;
use crate::token_maps::*;


#[pyclass]
pub struct DNATokenizer {
    #[pyo3(get)]
    kmer_size: usize,

    #[pyo3(get)]    
    drop_remainder: bool,

    #[pyo3(get)]
    pad_type: String,

    #[pyo3(get)]
    pad_length: i128,

    #[pyo3(get)]
    add_cls: bool,

    #[pyo3(get)]
    vocab: Option<HashMap<String, i32>>,
    
    #[pyo3(get)]
    reverse_vocab: Option<HashMap<i32, String>>,

    #[pyo3(get)]
    vocab_size: usize,

    #[pyo3(get)]
    as_aa: bool,

}
#[pymethods]
impl DNATokenizer {

    #[new]
    #[pyo3(signature = (kmer_size=3,  drop_remainder= false, pad_type="after".to_string(),
                        pad_length=0, add_cls= false,
                        vocab= None, as_aa= false ))]
    fn new(kmer_size: usize, drop_remainder: bool, pad_type:String,
            pad_length: i128, add_cls: bool, vocab:Option<HashMap<String, i32>>,
            as_aa: bool ) -> Self {
        
        if kmer_size != 3 && vocab.is_none() {
            panic!("If kmer_size is not 3, a vocabulary must be provided!")
        }
        let mut vocab_size= match as_aa {
            true => 25,
            false => 70
        };

        let mut reverse_vocab= Option::None;

        if vocab.is_some(){
                vocab_size= vocab.as_ref().unwrap().len();
                check_vocab(vocab.as_ref().unwrap(), add_cls);
                reverse_vocab = vocab.as_ref().map(|v| {
                v.iter()
                .map(|(k, id)| (*id, k.clone()))
                .collect::<HashMap<i32, String>>()
            });
        };
        
        DNATokenizer{
            kmer_size,
            drop_remainder,
            pad_type,
            pad_length,
            add_cls,
            vocab,
            reverse_vocab,
            vocab_size,
            as_aa,
        }
       
    }

    #[pyo3(signature = (sequences, n_jobs=1 ))]
    fn seqs_to_id<'pyt>(&self, py: Python<'pyt>, sequences: &Bound<'pyt, PyList>, n_jobs: i16)-> PyResult<Py<PyAny>> {
        
        let mut sequences = utils::extract_all_sequences(sequences)?;
        let cpu_to_use = utils::check_nb_cpus(n_jobs);
        let lookup= make_lookup(&self.vocab, self.as_aa);

        let config= TokenizeConfig{
            kmer_size: self.kmer_size,
            drop_remainder: self.drop_remainder,
            add_cls: self.add_cls,
            vocab: &self.vocab,
            lookup_fn: &lookup};

        
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(cpu_to_use)
            .build()
            .expect("Failed to build rayon thread pool");

        if self.pad_length == 0 {
            let results = py.detach(|| encode_parallel_no_pad(&mut sequences, &config, &pool));
            let py_list = PyList::empty(py);
            for arr in results {
                py_list.append(arr.into_pyarray(py))?;
            }
            return Ok(py_list.unbind().into());
        }

        let vec_length = utils::get_length_vec(&sequences, self.pad_length);
        let final_array =
            py.detach(|| encode_parallel(&mut sequences, &self.pad_type, vec_length, &config, &pool));

        Ok(final_array.into_pyarray(py).unbind().into())

    }

    fn ids_to_seq(&self, token_seq: &Bound<'_, PyAny>)-> PyResult<String>{
        
        if let Ok(tokens) = token_seq.extract::<Vec<i32>>() {
            let decoded= decode_ids(&tokens, &self.reverse_vocab, self.as_aa);
            let str= std::str::from_utf8(&decoded)
                            .map_err(|e| PyValueError::new_err(format!("Invalid UTF-8: {}", e)))?;
            return Ok(str.to_string())    
        }

        panic!("Cannot extract Token sequnce as a vector. Pass a Python List or numpy array only")
    }


}

fn decode_ids(ids: &[i32], reverse_vocab: &Option<HashMap<i32, String>>, as_aa: bool) -> Vec<u8> {
    let reverse_lookup = make_reverse_lookup(reverse_vocab, as_aa);

    let estimated_cap = ids.len() * 4;
    let mut out = Vec::with_capacity(estimated_cap);

    for  &id in ids.iter() {
        out.extend_from_slice(reverse_lookup(id));
    }

    out
}

fn check_vocab(vocab: &HashMap<String, i32>, add_cls: bool){

    if !vocab.contains_key("[UNK]") {
        panic!("Provided vocabulary must contain a '[UNK]' token!");
    }

     if !vocab.contains_key("[MASK]") {
        panic!("Provided vocabulary must contain a '[MASK]' token!");
    }

    if !vocab.contains_key("[PAD]") {
        panic!("Provided vocabulary must contain a '[PAD]' token!");
    }

    if add_cls && !vocab.contains_key("[CLS]") {
        panic!("Provided vocabulary must contain a '[CLS]' token when add_cls is true!");
    }
}
    

struct TokenizeConfig<'a> {
    kmer_size: usize,
    drop_remainder: bool,
    add_cls: bool,
    vocab: &'a Option<HashMap<String, i32>>,
    lookup_fn: &'a Box<dyn Fn(&[u8]) -> i32+ Send + Sync + 'a>
}


fn make_lookup(user_voc: &Option<HashMap<String, i32>>, as_aa: bool)
    -> Box<dyn Fn(&[u8]) -> i32 + Send + Sync + '_> {
    match user_voc {
        Some(map) => {
            let unk_id = map[UNK_TOKEN];
            Box::new(move |k: &[u8]| {
                // seq is already lowercase by the time this is called
                let decoded = std::str::from_utf8(k).expect("Invalid UTF-8 in sequence!");
                map.get(decoded).copied().unwrap_or(unk_id)
            })
        }
        None => {
            let default_fn: fn(&[u8]) -> Option<i32> =
                if as_aa { as_aa_to_int } else { codon_to_int };
            Box::new(move |k: &[u8]| default_fn(k).unwrap_or(UNK_ID))
        }
    }
}

fn make_reverse_lookup<'a>(
    reverse_vocab: &'a Option<HashMap<i32, String>>,
    as_aa: bool,
) -> Box<dyn Fn(i32) -> &'a [u8] + Send + Sync + 'a> {
    match reverse_vocab {
        Some(map) => {
            let unk_bytes = map[&(UNK_ID as i32)].as_bytes(); // resolved once
            Box::new(move |id: i32| {
                map.get(&id).map(|s| s.as_bytes()).unwrap_or(unk_bytes)
            })
        }
        None => {
            let decode_fn: fn(i32) -> Option<&'static [u8]> =
                if as_aa { int_to_aa } else { int_to_codon };
            Box::new(move |id: i32| decode_fn(id).unwrap_or(b"[UNK]"))
        }
    }
}


#[inline]
fn compute_array_length(length:usize, kmer_size: usize, drop_remainder: bool, add_cls: bool)-> usize{

    if kmer_size > length {
        panic!("kmer_size value is greater than the length of the sequence!")
    }

    if kmer_size < 1 {
        panic!("kmer_size value cannot be lower than 1")
    }

    let mut final_len;

    if length%kmer_size == 0 || drop_remainder {
        final_len= length/kmer_size;
    }

    else {
        final_len= (length/kmer_size)+1;
    }

    if add_cls {
        final_len +=1
    }

    final_len

}



fn tokenize_no_pad(seq:&mut Vec<u8>, config: &TokenizeConfig ) -> Array1<i32> {
    
    let arr_lentgh= compute_array_length(seq.len(), config.kmer_size,
                                                         config.drop_remainder, config.add_cls,);
    let mut array= Array1::<i32>::zeros(arr_lentgh);

    let mut cells=  array.outer_iter_mut();

    if config.add_cls {
       let cell= cells.next().expect("Array of length 0!").into_scalar();
        match config.vocab {
            Some(val) => *cell= val["[CLS]"],
            None => *cell= CLS_ID as i32
        }
    }

    let mut seq_chunks= seq.chunks_exact_mut(config.kmer_size);
    for (kmer, mut cell) in  (&mut seq_chunks).zip(&mut cells){
        kmer.make_ascii_lowercase();
        cell.fill((config.lookup_fn)(kmer));
    }

    let remainder= seq_chunks.into_remainder();
    if !config.drop_remainder && !remainder.is_empty() {
           let mut cell= cells.next().expect("Trying to index outside of array!");
           remainder.make_ascii_lowercase();
           cell.fill((config.lookup_fn)(remainder));
        }
        
    return array

}


fn encode_parallel_no_pad(sequences: &mut [Vec<u8>], config: &TokenizeConfig, pool: &rayon::ThreadPool) -> Vec<Array1<i32>> {
    pool.install(|| sequences.par_iter_mut().map(|seq| tokenize_no_pad(seq, config)).collect())
}


fn tokenize_before_fixed(seq:&mut Vec<u8>, mut row: ArrayViewMut1<i32>, config: &TokenizeConfig ) {
    
    let mut cells=  row.outer_iter_mut();

    if config.add_cls {
       let cell= cells.next().expect("Array of length 0!").into_scalar();
        match config.vocab {
            Some(val) => *cell= val["[CLS]"],
            None => *cell= CLS_ID as i32
        }
    }


    if !config.drop_remainder{
        let mut seq_chunks= seq.chunks_mut(config.kmer_size);
        for (kmer, mut cell) in (&mut seq_chunks).rev().zip((&mut cells).rev()).rev() {
            kmer.make_ascii_lowercase();
            cell.fill((config.lookup_fn)(kmer));
        }
    }

    else {
        let mut seq_chunks= seq.chunks_exact_mut(config.kmer_size);
        for (kmer, mut cell) in (&mut seq_chunks).rev().zip((&mut cells).rev()).rev() {
            kmer.make_ascii_lowercase();
            cell.fill((config.lookup_fn)(kmer));
    
        }
    }

        

}

fn tokenize_after_fixed(seq:&mut Vec<u8>, mut row: ArrayViewMut1<i32>, config: &TokenizeConfig ){
    

    let mut cells=  row.outer_iter_mut();

    if config.add_cls {
       let cell= cells.next().expect("Array of length 0!").into_scalar();
        match config.vocab {
            Some(val) => *cell= val["[CLS]"],
            None => *cell= CLS_ID as i32
        }
    }

    if !config.drop_remainder {
        let mut seq_chunks= seq.chunks_mut(config.kmer_size);
        for (kmer, mut cell ) in (&mut seq_chunks).zip(&mut cells) {
            kmer.make_ascii_lowercase();
            cell.fill((config.lookup_fn)(kmer));
        }
    }

    else{
        let mut seq_chunks= seq.chunks_exact_mut(config.kmer_size);
        for (kmer, mut cell ) in (&mut seq_chunks).zip(&mut cells) {
            kmer.make_ascii_lowercase();
            cell.fill((config.lookup_fn)(kmer));
        }

    }
    

    for mut cell in cells {
        cell.fill((config.lookup_fn)(b"[PAD]"))
    }
        

}


fn encode_parallel(
sequences: &mut [Vec<u8>],
pad_type: &str,
length: usize,
config: &TokenizeConfig,
pool: &rayon::ThreadPool,
) -> Array2<i32> {
let pad_val= (config.lookup_fn)(b"[PAD]");
let array_len= compute_array_length(length, config.kmer_size, config.drop_remainder, config.add_cls);
let mut final_array= Array2::<i32>::from_elem((sequences.len(), array_len), pad_val);
pool.install(|| {
    sequences
        .par_iter_mut()
        .zip(final_array.axis_iter_mut(Axis(0)).into_par_iter())
        .for_each(|(seq,  row)| match pad_type {
            "after" => tokenize_after_fixed(seq, row, config),
            "before" => tokenize_before_fixed(seq, row, config),
            _ => panic!("The only 2 options for the type of padding are 'before' and 'after'."),

            })
});

final_array

}

