use needletail::parser::Format::{Fasta, Fastq};
use needletail::{parse_fastx_file, FastxReader};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

#[pyclass]
pub struct SequenceRecord{
     #[pyo3(get, set)]
    seq: String,
     #[pyo3(get, set)]
    id: String,
     #[pyo3(get, set)]
    qual: Option<String>,
     #[pyo3(get, set)]
    format: String,
}

#[pymethods]
impl SequenceRecord {
    #[new]
    fn new(seq:String, id:String, qual:Option<String>, format:String)-> PyResult<Self>{
        
        Ok(SequenceRecord{
            seq,
            id,
            qual,
            format})
    }

    fn __str__(&self) -> String {
        format!("[ID: {}\n Format: {}\n Sequence: {:.20}...\n Quality: {:?}]\n", self.id, self.format,self.seq, self.qual )
    }
}

#[pyclass(unsendable)]
pub struct SequenceReader {
    #[pyo3(get, set)]
    filepath: String,

    #[pyo3(get)]
    batch_size: usize,

    records: Box<dyn FastxReader>,
}

#[pymethods]
impl SequenceReader {
    #[new]
    #[pyo3(signature = (filepath, batch_size= 0))]
    fn new(filepath: String, batch_size: usize) -> PyResult<Self> {
        
        let records= parse_fastx_file(&filepath).expect("Invalid path/file");
        Ok(SequenceReader {
            filepath,
            batch_size,
            records
        })
    }

    fn get_ids(&mut self) -> PyResult<Vec<String>> {
        
        let limit= self.batch_limit();
        let mut ids: Vec<String> = Vec::new();

        while ids.len() < limit {

            match self.records.next() {
                Some(record) => {
                    let seqrec= record.expect("Invalid sequence record!");
                    ids.push(str::from_utf8( &seqrec.id() ).expect("Invalid UTF-8 data").to_string());
                }
                None => { break;}
            }
        }

        Ok(ids)
    }

    fn get_sequences(&mut self) -> PyResult<Vec<String>> {
        
        let limit= self.batch_limit();
        let mut sequences: Vec<String> = Vec::new();

        while sequences.len() < limit {

            match self.records.next() {
                Some(record) => {
                    let seqrec= record.expect("Invalid sequence record!");
                    sequences.push(str::from_utf8( &seqrec.seq() ).expect("Invalid UTF-8 data").to_string());
                }
                None => { break;}
            }
        }

        Ok(sequences)
    }

    fn get_records(&mut self) ->  PyResult<Vec<SequenceRecord>>{

        let limit= self.batch_limit();
        let mut py_records: Vec<SequenceRecord> = Vec::new();

        while py_records.len() < limit {

            match self.records.next() {
                Some(record) => {
                    let seqrec= record.expect("Invalid sequence record!");
                    let seq= str::from_utf8( &seqrec.seq() ).expect("Invalid UTF-8 data").to_string();
                    let id= str::from_utf8( &seqrec.id() ).expect("Invalid UTF-8 data").to_string();
                    let format= match seqrec.format() {
                        Fasta => {"Fasta".to_string()},
                        Fastq => {"Fastq".to_string()}
                    };
                    let qual=  seqrec.qual().map(|x| str::from_utf8(x).expect("Invalid UTF-8 data").to_string());
                    py_records.push(SequenceRecord::new(seq, id, qual, format)?);
                }
                None => { break;}
            }
        }

        Ok(py_records)
    }

    fn reset(&mut self){
        self.records= parse_fastx_file(&self.filepath).expect("Invalid path/file");
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    
    fn __next__(&mut self) -> PyResult<Option<Vec<SequenceRecord>>> {
        let records = self.get_records()?;
        
        if records.is_empty() {
            
            Ok(None)
        } else {
            Ok(Some(records))
        }
    }
}



impl SequenceReader {

    fn batch_limit(&self) -> usize {
        if self.batch_size ==0 {
            usize::MAX
        }
        else {
            self.batch_size
        }
    }
    
    pub fn next_bytes(&mut self) -> PyResult<Vec<Vec<u8>>> {
        let mut sequences: Vec<Vec<u8>> = Vec::new();
        let limit= self.batch_limit();
        
        while sequences.len() < limit {
            match self.records.next() {
                Some(record) => {
                    let seqrec= record.map_err(|e| PyValueError::new_err(e.to_string()))?;
                    sequences.push(seqrec.seq().into_owned());
                }

                None => break
            }
            
        }
 
        Ok(sequences)
    }
}