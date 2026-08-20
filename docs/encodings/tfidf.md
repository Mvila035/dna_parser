# **Tfidf Class**

The `Tfidf` class encodes sequences using the Term Frequency-Inverse Document Frequency technique.

## **Class Signature**

### `Tfidf`

```python
class Tfidf(corpus, kmer, vocabulary=None, n_jobs=1, reader_fn=None, format=None)
```

**Constructor Parameters:**

| Name | Type | Default | Description  |
| :--- | :--- | :--- | :--- |
| `corpus` | `iterable containing str` or `str` | *Required* | Sequences to encode, can also be a path to a file. |
| `kmer` | `int` | *Required* | Size of the kmers used to build the vocabulary and compute the TF-IDF. |
| <span style="white-space: nowrap;">`vocabulary`</span> | `dict[str,int]` | `None` | Dictionary mapping each kmers to consider for encoding to a unique integer value. |
| `n_jobs` | `int` | `1` | number of threads used to encode the sequences in parallel. 0 to use all CPUs available. |
| `reader_fn` | `Callable` | `None` | Function to use to read sequences if `corpus`is a path to a file. |
| `format` | `str` | `None` | Format of the file when using SeqIO.read from Biopython. Either `fasta` or `fastq`. |

=== "Raises"

    * `WrongTfidfConfig`: If `corpus` is a `str` (a file path) but no `reader_fn` is provided.
---

!!! warning "Vocabulary Indices Must Be Contiguous"
    If you provide a vocabulary, its integer values must form a contiguous range starting at 0 — i.e. 0, 1, 2, ..., len(vocabulary) - 1, with no gaps. A vocabulary like {"aaa": 0, "aac": 2} (skipping 1) is invalid.

## Attributes

Use these attributes to check the live state of your `Tfidf` instance.

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `corpus` | `iterable containing str` or `str` | The sequences (or file path) currently associated with the instance. |
| `kmer_size` | `int` | Size of the kmers used to build the vocabulary and compute the TF-IDF. |
| `vocabulary` | `dict[str,int]` or `None` | Mapping of each kmer to its column index. `None` until it is provided in the constructor, via `set_vocabulary()`, or learned by `fit()` / `fit_transform()`. |
| `idf` | `numpy.ndarray` or `None` | Inverse document frequency value for each term in `vocabulary`. `None` until `fit()` or `fit_transform()` has been called. |
| <span style="white-space: nowrap;">`is_idf_uptodate`</span> | `bool` | `True` once `idf` has been computed for the current `vocabulary` and `corpus`. Reset to `False` by `set_vocabulary()` and `add_to_corpus()`. |
| `n_jobs` | `int` | Number of threads used to encode sequences in parallel. `0` uses all available CPUs. |
| `reader_fn` | `Callable` or `None` | Function used to read sequences when `corpus` is a file path. |
| `format` | `str` or `None` | Format passed to `reader_fn` when reading `corpus` from a file (e.g. `fasta` or `fastq`). |

---

## Methods

### `.set_vocabulary()` { #Tfidf.set_vocabulary }

```python
def set_vocabulary(vocabulary: dict[str, int]) -> None
```

Replaces the current vocabulary used to encode sequences.

=== "Parameters"

    * **`vocabulary`** (`dict[str,int]`): Dictionary mapping each kmer to consider for encoding to a unique integer value.

=== "Returns"

    * `None`

!!! warning "Refit Required"
    Calling `set_vocabulary()` sets `is_idf_uptodate` to `False`. You must call `fit()` (or `fit_transform()`) again before `transform()` will produce valid results.

---

### `.set_threads()` { #Tfidf.set_threads }

```python
def set_threads(n_jobs: int) -> None
```

Updates the number of threads used to encode sequences in parallel.

=== "Parameters"

    * **`n_jobs`** (`int`): Number of threads to use. `0` uses all CPUs available.

=== "Returns"

    * `None`

### `.add_to_corpus()` { #Tfidf.add_to_corpus }

```python
def add_to_corpus(new_corpus: iterable) -> None
```

Appends new sequences to the existing corpus in place.

=== "Parameters"

    * **`new_corpus`** (`iterable containing str`): Sequences to add to the current corpus. Cannot itself be a `str`.

=== "Returns"

    * `None`

=== "Raises"

    * `TypeError`: If `new_corpus` is a `str`. Pass a single sequence as `list[str]` instead.
    * `UnextendableCorpus`: If the instance's `corpus` is not an extendable collection of sequences (i.e. not a `list`, `set`, or `tuple`, and without its own `.extend()` method) — most commonly when `corpus` is a file path.

!!! warning "Refit Required"
    Calling `add_to_corpus()` sets `is_idf_uptodate` to `False`. You must call `fit()` (or `fit_transform()`) again before `transform()` will produce valid results for the updated corpus.

---

### `.fit()` { #Tfidf.fit }

```python
def fit() -> None
```

Learns the vocabulary from `corpus` (if one was not already set) and computes the inverse document frequency (`idf`) values needed to encode sequences with `transform()`.

=== "Parameters"

    * None

=== "Returns"

    * `None`

!!! note "Existing Vocabulary"
    If `vocabulary` is already set (either via the constructor or `set_vocabulary()`), `fit()` skips vocabulary learning and only computes `idf` values against that vocabulary.

---

### `.transform()` { #Tfidf.transform }

```python
def transform(sequences=None, normalization: str = "L2") -> scipy.sparse.csr_matrix
```

Encodes sequences into a TF-IDF weighted sparse matrix using the fitted vocabulary and `idf` values.

=== "Parameters"

    * **`sequences`** (`iterable containing str` or `str`, *optional*): Sequences to encode, can also be a path to a file. Defaults to the instance's `corpus` when not provided.
    * **`normalization`** (`str`, *optional*): Row normalization to apply to the resulting matrix. Currently supports `"L2"`. Defaults to `"L2"`.

=== "Returns"

    * `scipy.sparse.csr_matrix`: The TF-IDF encoded matrix, with one row per sequence and one column per vocabulary term.

=== "Raises"

    * `NotFittedError`: If `fit()` (or `fit_transform()`) has not been called since the last vocabulary or corpus change.
---

### `.fit_transform()` { #Tfidf.fit_transform }

```python
def fit_transform(normalization: str = "L2") -> scipy.sparse.csr_matrix
```

Fits the vocabulary (if not already set) and `idf` values on `corpus`, then returns the TF-IDF encoded matrix in a single call.

=== "Parameters"

    * **`normalization`** (`str`, *optional*): Row normalization to apply to the resulting matrix. Currently supports `"L2"`. Defaults to `"L2"`.

=== "Returns"

    * `scipy.sparse.csr_matrix`: The TF-IDF encoded matrix for `corpus`, with one row per sequence and one column per vocabulary term.
---

### `.compute_idf()` { #Tfidf.compute_idf }

```python
def compute_idf(matrix: scipy.sparse.csr_matrix) -> None
```

Computes the inverse document frequency for each vocabulary term from a raw term-count matrix and stores it in `idf`. Called internally by `fit()` and `fit_transform()`.

=== "Parameters"

    * **`matrix`** (`scipy.sparse.csr_matrix`): Sparse matrix of raw kmer counts, with one row per sequence and one column per vocabulary term.

=== "Returns"

    * `None`
---

## Exceptions

### `WrongTfidfConfig`

Raised by the constructor when `corpus` is a `str` (a file path) but no `reader_fn` is provided to read it.

### `NotFittedError`

Raised by `.transform()` when called before the instance has been fitted (i.e. before `fit()` or `fit_transform()` has been run against the current vocabulary and corpus).

### `UnextendableCorpus`

Raised by `.add_to_corpus()` when the instance's `corpus` cannot be extended in place — i.e. it is not a `list`, `set`, `tuple`, or another object exposing its own `.extend()` method. This most commonly happens when `corpus` was set to a file path.

---

## Examples

### Basic Usage

```python
import dna_parser as dps
sequences= ["agttcttctc","accttgtgt","acctgtcgc"]
tfidf= dps.Tfidf(sequences, 3)
matrix= tfidf.fit_transform()
print(matrix)

#Output:
#<Compressed Sparse Row sparse matrix of dtype 'float64'
#        with 9 stored elements and shape (3, 7)>
#  Coords        Values
#  (0, 0)        0.408248290463863
#  (0, 1)        0.816496580927726
#  (0, 2)        0.408248290463863
#  (1, 3)        0.32718457421366
#  (1, 4)        0.8865102981879298
#  (1, 5)        0.32718457421366
#  (2, 3)        0.32718457421366
#  (2, 5)        0.32718457421366
#  (2, 6)        0.8865102981879298

```


### Fitting with a Predefined Vocabulary

```python
import dna_parser as dps
sequences= ["agttcttctc","accttgtgt","acctgtcgc"]
vocabulary= {"acc":0, "agt":1,"tct":2, "ttg":3}

tfidf= dps.Tfidf(sequences, 3, vocabulary)
matrix= tfidf.fit_transform()
print(matrix)

#Output:
#<Compressed Sparse Row sparse matrix of dtype 'float64'
#        with 5 stored elements and shape (3, 4)>
#  Coords        Values
#  (0, 1)        0.4472135954999579
#  (0, 2)        0.8944271909999159
#  (1, 0)        0.3462415530579614
#  (1, 3)        0.9381453975456102
#  (2, 0)        1.0


```

### Encoding Sequences from a File

```python
import dna_parser as dps
from Bio import SeqIO

reader_fn= SeqIO.parse
form="fasta"
tfidf= dps.Tfidf("path/to/file.fasta", 3, reader_fn= reader_fn, format=form)
tfidf.fit_transform()

#Output:
#<Compressed Sparse Row sparse matrix of dtype 'float64'
#        with 16 stored elements and shape (3, 16)>

reader_fn= dps.SequenceReader
tfidf= dps.Tfidf("path/to/file.fasta", 3, reader_fn= reader_fn)
tfidf.fit_transform()

#Output:
#<Compressed Sparse Row sparse matrix of dtype 'float64'
#        with 16 stored elements and shape (3, 16)>
```

### Updating the Corpus and Re-fitting

```python
import dna_parser as dps
sequences= ["agttcttctc","accttgtgt","acctgtcgc"]

tfidf= dps.Tfidf(sequences, 3)
matrix= tfidf.fit_transform()

tfidf.add_to_corpus(["agtcgcgataaaa", "agatagagactgatagat"])
matrix= tfidf.fit_transform()
print(matrix)

#<Compressed Sparse Row sparse matrix of dtype 'float64'
#        with 11 stored elements and shape (5, 7)>
#  Coords        Values
#  (0, 0)        0.24673728071874365
#  (0, 1)        0.8667736564079552
#  (0, 2)        0.4333868282039776
#  (1, 3)        0.4434518002301797
#  (1, 4)        0.7789101371437052
#  (1, 5)        0.4434518002301797
#  (2, 3)        0.5773502691896257
#  (2, 5)        0.5773502691896257
#  (2, 6)        0.5773502691896257
#  (3, 0)        0.7071067811865475
#  (3, 6)        0.7071067811865475

```
