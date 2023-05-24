# dna_parser
![Build Status](https://github.com/Mvila035/dna_parser/workflows/CI/badge.svg)

dna-parser is a Python library written in rust to encode (or perform feature extraction on) DNA/RNA sequences for machine learning.

# Table of contents
1. [Install](https://github.com/Mvila035/dna_parser/edit/main/README.md#install)
2. [Usage](https://github.com/Mvila035/dna_parser/edit/main/README.md#usage)
    1. [Loading Fasta Files](https://github.com/Mvila035/dna_parser/edit/main/README.md#loading-fasta-files)
    2. [Encodings](https://github.com/Mvila035/dna_parser/edit/main/README.md#encodings)
    3. [Other Functions](https://github.com/Mvila035/dna_parser/edit/main/README.md#other-functions)

# Install <a name="install"></a>

To install dna-parser simply run:
```sh
pip install dna-parser
```

If there is no Python wheel available for your OS you can install Rust and re-install dna-parser which should now compile and your machine.
Run the following command on Unix-like OS to install Rust:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
or see more options at https://www.rust-lang.org/tools/install.

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

Currently only support ordinal encoding, onehot encoding, cross encoding and Term Frequency Inverse Document Frequency (TF-IDF).

### Ordinal Encoding

Nucleotides are currently encoded as follow:

* A= 0.25
* C= 0.50
* G= 0.75
* T/U= 1.0
* Other characters or gaps = 0

```python
#returns a list of 1D numpy arrays representing the encoding
encoding= dna_parser.ordinal_encoding(sequences, pad_type, pad_length, n_jobs)
```

Function Arguments:

* sequences (list of str): list of genomic sequences.
* pad_type (str; default= "after"): pad (or trim) "before" or "after" the sequences.
* pad_length (int; default= 0): -2 to pad according to the longest sequence, -1 to trim to the shortest sequence, 0 for no paddding, any positive number for a fixed length.
* n_jobs (int; default= 1): number of threads to use to encode the sequences. 0 to use all cpus available.


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

* sequences (list of str): list of genomic sequences.
* pad_type (str; default= "after"): pad (or trim) "before" or "after" the sequences.
* pad_length (int; default= 0): -2 to pad according to the longest sequence, -1 to trim to the shortest sequence, 0 for no paddding, any positive number for a fixed length.
* n_jobs (int; default= 1): number of threads to use to encode the sequences. 0 to use all cpus available.

### Cross Encoding

Nucleotides are currently encoded as follow:

* A= [0,-1]
* C= [-1,0]
* G= [1,0]
* T/U= [0,1]
* Other characters or gaps = [0,0]

```python
#returns a list of 2D numpy arrays representing the encoding
encoding= dna_parser.cross_encoding(sequences, pad_type, pad_length, n_jobs)

```

Function Arguments:

* sequences (list of str): list of genomic sequences.
* pad_type (str; default= "after"): pad (or trim) "before" or "after" the sequences.
* pad_length (int; default= 0): -2 to pad according to the longest sequence, -1 to trim to the shortest sequence, 0 for no paddding, any positive number for a fixed length.
* n_jobs (int; default= 1): number of threads to use to encode the sequences. 0 to use all cpus available.



### TF-IDF Encoding

Note that for this function, your sequences need to be split up in words (or k-mers) where each word is separated by a whitespace. To do so you can use the make_kmers function (see Other Functions section).

```python
encoding= dna_parser.tfidf_encoding(corpus)
```

Function Arguments:

* corpus (list of str): genomic sequences.


## Other Functions <a name="others"></a>

### Generating Random sequences

This function generates random dna, rna or amino acid sequences and returns them in a list.

```python
sequences= dna_parser.random_seq(lenght, nb_of_seq, seq_type, n_jobs)
```
Function Arguments:

* length (int): length of the sequences.
* nb_of_seq (int): number of sequences to generate.
* seq_type (str; default= dna): type of sequences. "dna", "rna" or "aa" (for amino acid).
* n_jobs (int, default= 1): number of threads to use to generate the sequences. 0 to use all cpus available.


### Making K-mers in Sequences

this function takes a string and returns a new one with withspaces inserted to form words of length k.

```python
seq_k_mers= dna_parser.make_kmers(seq, k)
```

Function Arguments:

* seq (str): the genomic sequence.
* k (int): length of words to create in the sequence.





