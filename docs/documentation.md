# Documentation

## Encodings

### Atomic Number

```python
dna_parser.atomic_encoding(sequences, pad_type= "after", pad_length= -2, n_jobs= 1)
```

#### Function Arguments

* **sequences** (list of str): list of genomic sequences.
* **pad_type** (str): pad (or trim) "before" or "after" the sequences.
* **pad_length** (int): -2 to pad according to the longest sequence, -1 to trim to the shortest sequence, any positive number for a fixed length.
* **n_jobs** (int): number of threads used to encode the sequences in parallel. 0 to use all CPUs available.

#### Output

**Numpy array with shape (number of sequences, length of sequences)**. Each nucleotide is encoded as its atomic number:

* A= 70
* C= 58
* G= 78
* T/U= 66
* Other characters or gaps = 0

```python
import dna_parser as dps

sequences= ["agt","acc"]
encoding= dps.atomic_encoding(sequences)
print(encoding)
print(encoding.shape)

# Output:
#[[70 78 66]
# [70 58 58]]
#
# (2, 3)
```
<br/>

### Chaos Game

```python
dna_parser.chaos_encoding(sequences, pad_type= "after", pad_length= -2, n_jobs= 1)
```

#### Function Arguments:

* **sequences** (list of str): list of genomic sequences.
* **pad_type** (str): pad (or trim) "before" or "after" the sequences.
* **pad_length** (int): -2 to pad according to the longest sequence, -1 to trim to the shortest sequence, any positive number for a fixed length.
* **n_jobs** (int): number of threads used to encode the sequences in parallel. 0 to use all CPUs available.

#### Output:

**Numpy array with shape (number of sequences, length of sequences, 2)**. <br />Each sequence is encoded in a square with vertices A: (1,1), C: (-1,-1),
G: (1,-1), T/U: (-1,1). The sequence representation starts at the center of the square in (0,0). The first nucleotide is represented as a point halfway between the starting point and its corresponding vertice. 
Each following nucleotide a new point halfaway between the previous point and its corresponding vertice. If a character other than A,C,G,T or U is encountered, the values are not updated and values from the previous point are used.

```python
import dna_parser as dps

sequences= ["agt","acc"]
encoding= dps.chaos_encoding(sequences)
print(encoding)
print(encoding.shape)

# Output:
#     x  ,   y
#[[[ 0.5    0.5  ]
#  [ 0.75  -0.25 ]
#  [-0.125  0.375]]
#
# [[ 0.5    0.5  ]
#  [-0.25  -0.25 ]
#  [-0.625 -0.625]]]
#(2, 3, 2)
```
<br/>

### Cross

```python
dna_parser.cross_encoding(sequences, pad_type= "after", pad_length= -2, n_jobs= 1)
```

#### Function Arguments

* **sequences** (list of str): list of genomic sequences.
* **pad_type** (str): pad (or trim) "before" or "after" the sequences.
* **pad_length** (int): -2 to pad according to the longest sequence, -1 to trim to the shortest sequence, any positive number for a fixed length.
* **n_jobs** (int): number of threads used to encode the sequences in parallel. 0 to use all CPUs available.

#### Output:

**Numpy array with shape (number of sequences, length of sequences,2).** Each nucleotide is encoded as follows:

* A= [0,-1]
* C= [-1,0]
* G= [1,0]
* T/U= [0,1]
* Other characters or gaps = [0,0]

```python
import dna_parser as dps

sequences= ["agt","acc"]
encoding= dps.cross_encoding(sequences)
print(encoding)
print(encoding.shape)

# Output:
#[[[ 1  1]
#  [-1  1]
#  [ 1 -1]]
#
# [[ 1  1]
#  [-1 -1]
#  [-1 -1]]]
#
#(2, 3, 2)

```
<br/>

### DNA Walk

```python
dna_parser.dna_walk(sequences, pad_type= "after", pad_length= -2, n_jobs= 1)
```

#### Function Arguments

* **sequences** (list of str): list of genomic sequences.
* **pad_type** (str): pad (or trim) "before" or "after" the sequences.
* **pad_length** (int): -2 to pad according to the longest sequence, -1 to trim to the shortest sequence, any positive number for a fixed length.
* **n_jobs** (int): number of threads used to encode the sequences in parallel. 0 to use all CPUs available.

#### Output

**Numpy array with shape (number of sequences, length of sequences, 2).** <br/> Each sequence is represented on a 2D grid, with its representation starting at coordinates (0,0). For each nucleotide in the sequence coordinates are updated to form a path as follows:


* A: $x_{n+1}= x_{n}-1$; &nbsp; $y_{n+1}= y_{n}$
* C: $x_{n+1}= x_{n}$; &nbsp; $y_{n+1}= y_{n}-1$
* G: $x_{n+1}= x_{n}$; &nbsp; $y_{n+1}= y_{n}+1$
* T/U: $x_{n+1}= x+1$; &nbsp; $y_{n+1}= y_{n}$
* Other characters or gaps: $x_{n+1}= x_{n}$; &nbsp; $y_{n+1}= y_{n}$


```python
import dna_parser as dps

sequences= ["agt","acc"]
encoding= dps.dna_walk(sequences)
print(encoding)
print(encoding.shape)

# Output:
#   x , y
#[[[-1  0]
#  [-1  1]
#  [ 0  1]]
#
# [[-1  0]
#  [-1 -1]
#  [-1 -2]]]
#(2, 3, 2)

```
<br/>

### EIIP

```python
dna_parser.eiip_encoding(sequences, pad_type= "after", pad_length= -2, n_jobs= 1)
```

#### Function Arguments

* **sequences** (list of str): list of genomic sequences.
* **pad_type** (str): pad (or trim) "before" or "after" the sequences.
* **pad_length** (int): -2 to pad according to the longest sequence, -1 to trim to the shortest sequence, any positive number for a fixed length.
* **n_jobs** (int): number of threads used to encode the sequences in parallel. 0 to use all CPUs available.

#### Output

**Numpy array with shape (number of sequences, length of sequences).** Each nucleotide is encoded as its electron-ion interaction pseudopotential (EIIP):

* A= 0.1260
* C= 0.1340
* G= 0.0806
* T/U= 0.1335
* Other characters or gaps = 0.0

```python
import dna_parser as dps

sequences= ["agt","acc"]
encoding= dps.eiip_encoding(sequences)
print(encoding)
print(encoding.shape)

# Output:
#[[0.126  0.0806 0.1335]
# [0.126  0.134  0.134 ]]
#
# (2, 3)
```
<br/>

### Fickett Score

```python
dna_parser.fickett_score(sequences, n_jobs= 1)
```

#### Function Arguments

* **sequences** (list of str): list of genomic sequences.
* **n_jobs** (int): number of threads used to encode the sequences in parallel. 0 to use all CPUs available.

#### Output

**Numpy array with shape (number of sequences).** <br/>Compute the probability of each sequence to be a coding sequence. See the [About](about.md#fickett-score) section for more details.


```python
import dna_parser as dps

sequences= ["agt","acc"]
encoding= dps.fickett_score(sequences)
print(encoding)
print(encoding.shape)

# Output:
#[0.3203 0.407 ]
#
#(2,)
```
<br/>

### Onehot (Voss)

```python
dna_parser.onehot_encoding(sequences, pad_type= "after", pad_length= -2, n_jobs= 1)
```

#### Function Arguments

* **sequences** (list of str): list of genomic sequences.
* **pad_type** (str): pad (or trim) "before" or "after" the sequences.
* **pad_length** (int): -2 to pad according to the longest sequence, -1 to trim to the shortest sequence, any positive number for a fixed length.
* **n_jobs** (int): number of threads used to encode the sequences in parallel. 0 to use all CPUs available.

#### Output

**Numpy array with shape (number of sequences, length of sequences, 4).** Each nucleotide is encoded as follows:

* C= [1,0,0,0]
* G= [0,1,0,0]
* A= [0,0,1,0]
* T/U= [0,0,0,1]
* Other characters or gaps = [0,0,0,0]

```python
import dna_parser as dps

sequences= ["agt","acc"]
encoding= dps.onehot_encoding(sequences)
print(encoding)
print(encoding.shape)

# Output:
#[[[0 0 1 0]
#  [0 1 0 0]
#  [0 0 0 1]]

# [[0 0 1 0]
#  [1 0 0 0]
#  [1 0 0 0]]]

# (2, 3, 4)
```
<br/>

### Real-number (or PAM)

```python
dna_parser.real_encoding(sequences, pad_type= "after", pad_length= -2, n_jobs= 1)
```

#### Function Arguments

* **sequences** (list of str): list of genomic sequences.
* **pad_type** (str): pad (or trim) "before" or "after" the sequences.
* **pad_length** (int): -2 to pad according to the longest sequence, -1 to trim to the shortest sequence, any positive number for a fixed length.
* **n_jobs** (int): number of threads used to encode the sequences in parallel. 0 to use all CPUs available.

#### Output

**Numpy array with shape (number of sequences, length of sequences).** Each nucleotide is encoded as follows:

* A= -1.5
* G= -0.5
* C= 0.5
* T/U= 1.5
* Other characters or gaps = 0

```python
import dna_parser as dps

sequences= ["agt","acc"]
encoding= dps.real_encoding(sequences)
print(encoding)
print(encoding.shape)

# Output:
#[-1.5 -0.5  1.5]
#[-1.5  0.5  0.5]]
#
# (2, 3)
```
<br/>

### TF-IDF

```python
#Class
dna_parser.Tfidf(corpus , kmer, vocabulary= None)
```
**Parameters:** 

* **corpus** (list of str): list of genomic sequences.
* **kmer** (int): length to use to generate kmers in the sequences.
* **vocabulary** (dict(str:int)): Dictionary mapping each kmers to consider for encoding to a unique integer value.
* **n_jobs** (int): number of threads used to encode the sequences in parallel. 0 to use all CPUs available. 
<br/>

#### Methods:

##### add_to_corpus
```python
Tfidf.add_to_corpus(new_corpus)
```
* **new_corpus**: (list of str): list of genomic sequences.
Adds sequences to the existing corpus.

##### fit
```python
Tfidf.fit()
```

Fits the Tfidf instance. Compiles the vocabulary if it is not provided. Computes the Inverse Document Frequency.

##### fit_transform
```python
Tfidf.fit_transform(sequences= None, normalization= "L2")
```
* **sequences** (list of str or None): list of genomic sequences to transform.
* **normalization** (str): "L2" for L2 normalization. Anything else results in no normalization.

Fits the Tfidf instance. Compiles the vocabulary if it is not provided. Computes the Inverse Document Frequency
and transforms the sequences in their TF-IDF representation. If "sequences= None", transforms the corpus.


##### set_threads
```python
Tfidf.set_threads(n_jobs)
```
**n_jobs** (int): number of threads used to encode the sequences in parallel. 0 to use all CPUs available.

Adjusts the number of threads used to encode the sequences in parallel.

##### set_vocabulary
```python
Tfidf.set_vocabulary(vocabulary):
```
* **vocabulary**: (dict(str:int)) Dictionary mapping each kmers to consider for encoding to a unique integer value.

The integer associated with each kmer needs to be unique, continuous, and start at 0.
```python
#this vocabulary is correct:
{"gtc":0, "atg":1, "acg":2}

#these are not correct:
{"gtc":1, "atg":2, "acg":3}
{"gtc":0, "atg":2, "acg":3}
```

##### transform
```python
Tfidf.transform(sequences= None, normalization= "L2")
```
* **sequences**: (list of str or None): list of genomic sequences to transform.
* **normalization**: (str): "L2" for L2 normalization. Anything else results in no normalization.

Transforms the sequences in their TF-IDF representation. If "sequences= None", transforms the corpus. The Tfidf instance
needs to be fitted with the fit() or fit_transform() function before calling transform().

#### Attributes
```python
Tfidf.vocabulary # None or dict(str:int)
```

```python
Tfidf.corpus     # List(str)
```

```python
Tfidf.kmer_size  # Int
```

```python
Tfidf.idf        # None or numpy array
```

```python
Tfidf.is_idf_uptodate # Bool
```

```python
Tfidf.n_jobs # Int
```

#### Examples
```python
import dna_parser as dps

sequences= ["agtcgc","accgtc"]
tfidf= dps.Tfidf(sequences,2)
tfidf.fit()
encoding= tfidf.transform()
print(encoding)
print(encoding.shape)

# Output:
#<Compressed Sparse Row sparse matrix of dtype 'float64'
#        with 2 stored elements and shape (2, 5)>
#  Coords        Values
#  (0, 1)        -0.1351550360360548
#  (1, 1)        -0.1351550360360548
#
# (2, 5)
```

```python
import dna_parser as dps

sequences= ["attcggagt","attctggga"]
tfidf= dps.Tfidf(sequences,3)
encodings= tfidf.fit_transform()

tfidf.add_to_corpus(["agccgcgga"])
encodings2= tfidf.fit_transform(normalization= None)
print(encodings)
print(encodings2)

# Output:
#<Compressed Sparse Row sparse matrix of dtype 'float64'
#        with 6 stored elements and shape (2, 5)>
#  Coords        Values
#  (0, 0)        0.0
#  (0, 1)        0.7071067811865476
#  (0, 2)        0.7071067811865476
#  (1, 0)        0.0
#  (1, 3)        0.7071067811865476
#  (1, 4)        0.7071067811865476
#<Compressed Sparse Row sparse matrix of dtype 'float64'
#        with 7 stored elements and shape (3, 5)>
#  Coords        Values
#  (0, 0)        0.4054651081081644
#  (0, 1)        1.0986122886681098
#  (0, 2)        1.0986122886681098
#  (1, 0)        0.4054651081081644
#  (1, 3)        1.0986122886681098
#  (1, 4)        0.4054651081081644
#  (2, 4)        0.4054651081081644
```
<br/>

### Z-Curve


```python
dna_parser.zcurve_encoding(sequences, pad_type= "after", pad_length= -2, window= 1, n_jobs= 1)
```

#### Function Arguments

* **sequences** (list of str): list of genomic sequences.
* **pad_type** (str): pad (or trim) "before" or "after" the sequences.
* **pad_length** (int): -2 to pad according to the longest sequence, -1 to trim to the shortest sequence, any positive number for a fixed length.
* **window** (int): non-overlapping window size to consider along the sequence for the encoding.
* **n_jobs** (int): number of threads used to encode the sequences in parallel. 0 to use all CPUs available.

#### Output

**Numpy array with shape (number of sequences, (length of sequences/window), 3).** <br /> The sequences are encoded within a cube. At each window position the Z-curve encoding gives the disparity between purines (r) and pyrimidines (y), the disparity between nucleotides with an amino (m) and a keto (k) group, and the disparity between nucleotide with weak (w) and strong (s) bonds.


```python
import dna_parser as dps

sequences= ["agtc","acc"]
encoding= dps.zcurve_encoding(sequences)
print(encoding)
print(encoding.shape)

# Output:
#  r-y m-k w-s 
#[[[ 1  1  1]
#  [ 2  0  0]
#  [ 1 -1  1]
#  [ 0  0  0]]

# [[ 1  1  1]
#  [ 0  2  0]
#  [-1  3 -1]
#  [-1  3 -1]]]
#
#(2, 4, 3)

sequences= ["agtc","acc"]
encoding= dps.zcurve_encoding(sequences, window=2)
print(encoding)
print(encoding.shape)

#Output:
#[[[ 2  0  0]
#  [ 0  0  0]]
#
# [[ 0  2  0]
#  [-1  3 -1]]]
#(2, 2, 3)

```
<br/>

## Importing Sequences

### Importing Fasta Files

```python
dna_parser.load_fasta(path)
```

#### Function Arguments

* **path** (str or list of str): a path or list of paths of files to import.

#### Output

A list of tuples containing the metadata and sequences of each entry in the fasta file (metadata, sequence).

```python
import dna_parser as dps

sequences= dps.load_fasta("path/to/fasta/file")
print(sequences)

# Output:
#[('>sequence1', 'acgtatgcgtcgtc'), ('>sequence2', 'cccgtga---gtcgat'), ('>sequence3', 'xgtcgycaaatcg-?')]
```
<br/>

### Importing Sequences Only

```python
dna_parser.load_sequences(path)
```

#### Function Arguments

* **path** (str or list of str): a path or list of paths of files to import.

#### Output

A list of str containing the sequences imported from fasta files.

```python
import dna_parser as dps

sequences= dps.load_sequences("tests/seq_test.fasta")
print(sequences)

# Output:
# ['acgtatgcgtcgtc', 'cccgtga---gtcgat', 'xgtcgycaaatcg-?']
```
<br/>

### Importing Metadata Only

```python
dna_parser.load_metadata(path)
```

#### Function Arguments

* **path** (str or list of str): a path or list of paths of files to import.

#### Output

A list of str containing the metadata imported from fasta files.

```python
import dna_parser as dps

metadata= dps.load_metadata("tests/seq_test.fasta")
print(metadata)

# Output:
# ['>sequence1', '>sequence2', '>sequence3']
```
<br/>

## Other Functions

### Kmers 

```python
dna_parser.make_kmers(seq, k)
```
#### Function Arguments

* **seq** (str): a genomic sequence.
* **k** (int): a number representing the length of kmers.

#### Output

A new sequence with white spaces inserted to form kmers of length k.

```python
import dna_parser as dps

kmer_seq= dps.make_kmers("agtcgtgcgtggaagagt", 3)
print(kmer_seq)

# Output:
# 'agt cgt gcg tgg aag agt '
```
<br/>

### Generating Random Sequences

```python
dna_parser.random_seq(length, nb_of_seq, seq_type= "dna", n_jobs= 1)
```

#### Function Arguments

* **length** (int): length of sequences to generate.
* **nb_of_seq** (int): number of sequences to generate.
* **seq_type** (str): type of sequence to generate. either "dna", "rna", or "aa" for amino acid
* **n_jobs** (int): number of threads used to generate sequences in parallel.

#### Output

A list of str representing the random sequences generated from a uniform probability distribution.

```python
import dna_parser as dps

sequences= dps.random_seq(15,3)
print(sequences)

# Output:
# ['tagtccaaccacttg', 'gcagtactaaactca', 'caaggccatgaggta']
```