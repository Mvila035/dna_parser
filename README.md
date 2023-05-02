# dna_parser

dna_parser is a small python library written in rust to perform encoding/feature extraction for machine learning on dna and rna sequences.

## Usage

```python
import dna_parser
```

### Loading Fasta Files

```python
#load both metadata and sequence in tuples (metadata,sequences)
metadata_and_sequences= dna_parser.load_fasta("path/to/fasta/file")

#load sequence only
sequences= dna_parser.seq_from_fasta("path/to/fasta/file")

#load metadata only
metadata= dna_parser.metadata_from_fasta("path/to/fasta/file")
```

### Encoding 

Currently only support ordinal encoding, onehot encoding and Term Frequency Inverse Document Frequency (TF-IDF).

#### Ordinal Encoding

Nucleotides are currently encoded as follow:

* A= 0.25
* C= 0.50
* G= 0.75
* T/U= 0.75
* Other characters or gaps = 0

```python

encoding= dna_parser.ordinal_encoding(sequences, pad_type, pad_length, n_jobs)
```

Function Arguments:

* sequences: List of strings (representing your sequences). 
* pad_type: pad (or trim) "before" the sequence or "after" the sequences.
* pad_length: -2 to pad according to the longest sequence, -1 to trim to shortest, 0 for no paddding, any positive number for a fixed length.
* n_jobs: number of threads to use to encode the sequences. 0 to use all cpus available.


#### OneHot Encoding

Nucleotides are currently encoded as follow:

* A= [1,0,0,0]
* C= [0,1,0,0]
* G= [0,0,1,0]
* T/U= [0,0,0,1]
* Other characters or gaps = [0,0,0,0]

```python

encoding= dna_parser.onehot_encoding(sequences, pad_type, pad_length, n_jobs)
```

Function Arguments:

* sequences: List of strings (representing your sequences). 
* pad_type: pad (or trim) "before" the sequence or "after" the sequences.
* pad_length: -2 to pad according to the longest sequence, -1 to trim to shortest, 0 for no paddding, any positive number for a fixed length.
* n_jobs: number of threads to use to encode the sequences. 0 to use all cpus available.

### Other Functions








