# **SequenceReader Class**

`SequenceReader` is a streaming reader for FASTA/FASTQ files, yielding [`SequenceRecord`](SequenceRecord.md) objects (or their raw ids/sequences) in batches.

## **Class Signature**

### `SequenceReader`

```python
class SequenceReader(filepath, batch_size=0)
```

**Constructor Parameters:**

| Name | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `filepath` | `str` | *Required* | Path to a FASTA or FASTQ file to read. |
| <span style="white-space: nowrap;">`batch_size`</span> | `int` | `0` | Maximum number of records returned per call to `.get_ids()`, `.get_sequences()`, or `.get_records()`. `0` returns every remaining record in a single call. |

=== "Raises"

    * `pyo3_runtime.PanicException`: If `filepath` cannot be opened or parsed as a valid FASTA/FASTQ file.

!!! warning "Not Thread-Safe"
    `SequenceReader` is marked `unsendable`: an instance cannot be shared or moved across threads. Use it from a single thread/process.

!!! note "Streaming Reader"
    `SequenceReader` reads the file lazily and keeps an internal cursor. Each call to `.get_ids()`, `.get_sequences()`, or `.get_records()` returns up to `batch_size` records starting from wherever the previous call left off, returning fewer records (or an empty list) once the file is exhausted. Call `.reset()` to rewind back to the start of `filepath`.

---

## Attributes

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `filepath` | `str` | Path to the file being read. Readable and writable, though reassigning it does not itself reopen the file — call `.reset()` to apply a new path. |
| <span style="white-space: nowrap;">`batch_size`</span> | `int` | *Read-only.* Maximum number of records returned per call to `.get_ids()`, `.get_sequences()`, or `.get_records()`. `0` means no limit. |

---

## Methods

### `.get_ids()` { #SequenceReader.get_ids }

```python
def get_ids() -> list[str]
```

Returns the next batch of record identifiers, advancing the internal cursor.

=== "Parameters"

    * None

=== "Returns"

    * `list[str]`: Up to `batch_size` identifiers (or all remaining ones if `batch_size` is `0`). An empty list once the file is exhausted.

=== "Raises"

    * `pyo3_runtime.PanicException`: If a record fails to parse, or if its identifier is not valid UTF-8.

---

### `.get_sequences()` { #SequenceReader.get_sequences }

```python
def get_sequences() -> list[str]
```

Returns the next batch of raw sequence strings, advancing the internal cursor.

=== "Parameters"

    * None

=== "Returns"

    * `list[str]`: Up to `batch_size` sequences (or all remaining ones if `batch_size` is `0`). An empty list once the file is exhausted.

=== "Raises"

    * `pyo3_runtime.PanicException`: If a record fails to parse, or if its sequence is not valid UTF-8.

---

### `.get_records()` { #SequenceReader.get_records }

```python
def get_records() -> list[SequenceRecord]
```

Returns the next batch of full records (id, sequence, quality, and format), advancing the internal cursor.

=== "Parameters"

    * None

=== "Returns"

    * `list[SequenceRecord]`: Up to `batch_size` records (or all remaining ones if `batch_size` is `0`). An empty list once the file is exhausted.

=== "Raises"

    * `pyo3_runtime.PanicException`: If a record fails to parse, or if its identifier, sequence, or quality string is not valid UTF-8.

---

### `.reset()` { #SequenceReader.reset }

```python
def reset() -> None
```

Rewinds the reader back to the beginning of `filepath` by reopening the file, so subsequent calls to `.get_ids()`, `.get_sequences()`, or `.get_records()` start over from the first record.

=== "Parameters"

    * None

=== "Returns"

    * `None`

=== "Raises"

    * `pyo3_runtime.PanicException`: If `filepath` can no longer be opened or parsed.

---

## Integration with Other Classes

A `SequenceReader` instance can be passed directly wherever the package accepts a `sequences`/`corpus` argument — for example `Tfidf`'s `corpus`, or `DNATokenizer.seqs_to_id()`'s `sequences`. In that case it is consumed internally in batches of `batch_size` records, without needing to call `.get_sequences()` manually first.

---

## Exceptions

`SequenceReader` surfaces invalid files, malformed records, and invalid UTF-8 as Rust panics, raised in Python as `pyo3_runtime.PanicException`. It does not raise an ordinary Python exception for these cases (unlike `DNATokenizer`, which raises `TypeError`/`ValueError` for some invalid inputs).

---

## Examples

### Reading an Entire File at Once and Resetting

```python
import dna_parser as dps

reader= dps.SequenceReader("path/to/file.fasta")
records= reader.get_records()
print(records)

#Output: List of SequenceRecord for the 3 sequences in the file
# [<builtins.SequenceRecord object at 0x106994b30>, <builtins.SequenceRecord object at 0x108e81430>, <builtins.SequenceRecord object at 0x106adc830>]

reader.reset()
sequences= reader.get_sequences()
print(sequences)

#Ouput:
# ['acgtatgcgtcgtc', 'cccgtga---gtcgat', 'xgtcgycaaatcg-?']

reader= dps.SequenceReader("path/to/file.fasta.xz") # can also read compressed files (.xz, .gz, .bz/bz2, .zstd)
ids= reader.get_ids()
print(ids)

#Output:
# ['sequence1', 'sequence2', 'sequence3']
```

### Reading in Batches and as an Iterator

```python
import dna_parser as dps

reader= dps.SequenceReader("path/to/file.fasta", batch_size=2)
records= reader.get_records()
print(records)

#Output:
# [<builtins.SequenceRecord object at 0x10c337830>, <builtins.SequenceRecord object at 0x10c335830>]

records= reader.get_records()
print(records)
#Output:
# [<builtins.SequenceRecord object at 0x10c337830>]

reader= dps.SequenceReader("path/to/file.fasta", batch_size=1)

#as an Iterator reader yields a list of SequenceRecord containing batch_size sequences
for [seq] in reader:
    print(seq)

#Output: 
#[ID: sequence1
# Format: Fasta
# Sequence: acgtatgcgtcgtc...
# Quality: None]
#
#[ID: sequence2
# Format: Fasta
# Sequence: cccgtga---gtcgat...
# Quality: None]
#
#[ID: sequence3
# Format: Fasta
# Sequence: xgtcgycaaatcg-?...
# Quality: None]
```


### Passing a SequenceReader Directly to Encoding Functions

```python
import dna_parser as dps
# 3 sequences in reader (1 batch of 2 sequences, 1 batch of 1 sequence)
reader= dps.SequenceReader("path/to/file.fasta", batch_size=2)

dps.eiip_encoding(reader)
#Output:
#[array([0.126 , 0.134 , 0.0806, 0.1335, 0.126 , 0.1335, 0.0806, 0.134 ,
#        0.0806, 0.1335, 0.134 , 0.0806, 0.1335, 0.134 ]),
# array([0.134 , 0.134 , 0.134 , 0.0806, 0.1335, 0.0806, 0.126 , 0.    ,
#        0.    , 0.    , 0.0806, 0.1335, 0.134 , 0.0806, 0.126 , 0.1335])]

dps.eiip_encoding(reader)
#Output:
#[array([0.    , 0.0806, 0.1335, 0.134 , 0.0806, 0.    , 0.134 , 0.126 ,
#        0.126 , 0.126 , 0.1335, 0.134 , 0.0806, 0.    , 0.    ])]
```
!!! warning "Padding/Trimming on Batches"
    Encoding functions only have access to the current batch. Setting the padding option to longest or shortest sequence pads according to the longest/shortest sequence in the batch. We recommend using no padding or fixed padding when working with batches.  