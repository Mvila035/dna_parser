# dna_parser

dna_parser is a small python library written in rust to perform encoding/feature extraction for machine learning on dna and rna sequences.

# Table of contents
1. [Install](https://github.com/Mvila035/dna_parser/edit/main/README.md#install)
2. [Usage](https://github.com/Mvila035/dna_parser/edit/main/README.md#usage)
    1. [Loading Fasta Files](https://github.com/Mvila035/dna_parser/edit/main/README.md#loading-fasta-files)
    2. [Encodings](https://github.com/Mvila035/dna_parser/edit/main/README.md#encodings)
    3. [Other Functions](https://github.com/Mvila035/dna_parser/edit/main/README.md#other-functions)

# Install <a name="install"></a>

For now, you need to have the rust programming language installed on your computer to install the library. 

Run the following command on Unix-like OS to install rust:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
or see more options at https://www.rust-lang.org/tools/install.

then, to install the test version you can run:
```sh
pip install --index-url https://test.pypi.org/simple/ --extra-index-url https://pypi.org/simple dna-parser
```


# Usage

```python
import dna_parser
```

## Loading Fasta Files <a name="fasta"></a>

```python
#load both metadata and sequence in tuples (metadata,sequences)
metadata_and_sequences= dna_parser.load_fasta("path/to/fasta/file")

#load sequence only
sequences= dna_parser.seq_from_fasta("path/to/fasta/file")

#load metadata only
metadata= dna_parser.metadata_from_fasta("path/to/fasta/file")
```

## Encodings <a name="encodings"></a>

Currently only support ordinal encoding, onehot encoding and Term Frequency Inverse Document Frequency (TF-IDF).

### Ordinal Encoding

Nucleotides are currently encoded as follow:

* A= 0.25
* C= 0.50
* G= 0.75
* T/U= 0.75
* Other characters or gaps = 0

```python
#returns a list of 1D numpy arrays representing the encoding
encoding= dna_parser.ordinal_encoding(sequences, pad_type, pad_length, n_jobs)
```

Function Arguments:

* sequences: List of strings (representing your sequences). 
* pad_type: pad (or trim) "before" the sequence or "after" the sequences.
* pad_length: -2 to pad according to the longest sequence, -1 to trim to shortest, 0 for no paddding, any positive number for a fixed length.
* n_jobs: number of threads to use to encode the sequences. 0 to use all cpus available.


### OneHot Encoding

Nucleotides are currently encoded as follow:

* A= [1,0,0,0]
* C= [0,1,0,0]
* G= [0,0,1,0]
* T/U= [0,0,0,1]
* Other characters or gaps = [0,0,0,0]

```python
#returns a list of 2D numpy arrays representing the encoding
encoding= dna_parser.onehot_encoding(sequences, pad_type, pad_length, n_jobs)

```

Function Arguments:

* sequences: List of strings (representing your sequences). 
* pad_type: pad (or trim) "before" the sequence or "after" the sequences.
* pad_length: -2 to pad according to the longest sequence, -1 to trim to shortest, 0 for no paddding, any positive number for a fixed length.
* n_jobs: number of threads to use to encode the sequences. 0 to use all cpus available.

### TF-IDF Encoding

Note that for this function, your sequences need to be split up in words (or k-mers) where each word is separated by a whitespace. To do so you can use the make_kmers function (see Other Functions section)

```python
encoding= dna_parser.tfidf_encoding(corpus)
```

Function Arguments:

* corpus: List of strings (representing your sequences). 


## Other Functions <a name="others"></a>

### Generating Random sequences

This function generates random dna, rna or amino acid sequences and returns them in a list.

```python
sequences= dna_parser.random_seq(lenght, nb_of_seq, seq_type, n_jobs)
```
Function Arguments:

* length: integer representing the length of the sequences
* nb_of_seq: integer representing the number of sequences to generate
* seq_type: string representing the type of sequence. dna, rna or aa (for amino acid)
* n_jobs: number of threads to use to generate the sequences. 0 to use all cpus available.


### Making K-mers in Sequences

this function takes a string and returns a new one with withspaces inserted to form words of length k.

```python
seq_k_mers= dna_parser.make_kmers(seq, k)
```

Function Arguments:

* seq: string representing a sequence
* k: integer representing the length of words to form





