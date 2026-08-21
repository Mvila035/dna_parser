# **DNATokenizer Class**

The `DNATokenizer` class encodes DNA/RNA sequences into integer token ids (and decodes them back), operating either on codons, amino acids, or a fully custom kmer vocabulary.

## **Class Signature**

### `DNATokenizer`

```python
class DNATokenizer(kmer_size=3, drop_remainder=False, pad_type="after",
                    pad_length=0, add_cls=False, vocab=None, as_aa=False)
```

**Constructor Parameters:**

| Name | Type | Default | Description  |
| :--- | :--- | :--- | :--- |
| `kmer_size` | `int` | `3` | Size of the kmers used to tokenize sequences. Must be `3` unless a custom `vocab` is provided, since the built-in codon/amino-acid tables only cover kmers of length 3. |
| <span style="white-space: nowrap;">`drop_remainder`</span> | `bool` | `False` | Whether to drop the trailing partial kmer when a sequence's length is not evenly divisible by `kmer_size`. |
| `pad_type` | `str` | `"after"` | Where padding is inserted relative to the encoded tokens when `pad_length` is set. `"after"` right-pads (tokens first, `[PAD]` at the end); `"before"` left-pads (`[PAD]` first, tokens at the end). |
| `pad_length` | `int` | `0` | Target sequence length **in bases**, used to size the fixed output array before tokenization. `0` disables padding (`.seqs_to_id()` returns one variable-length array per sequence). `-1` pads every sequence to the length of the shortest sequence in the batch. `-2` pads to the length of the longest. Any positive integer pads/truncates directly to that many bases. Values below `-2` are invalid. |
| `add_cls` | `bool` | `False` | Whether to prepend a `[CLS]` token to every encoded sequence. When enabled, `[CLS]` always occupies index `0` of the row, regardless of `pad_type`. |
| `vocab` | `dict[str,int]` | `None` | Custom vocabulary mapping kmers/tokens to ids. Must contain `[UNK]`, `[MASK]`, and `[PAD]`, plus `[CLS]` if `add_cls=True`. Required whenever `kmer_size != 3`. |
| `as_aa` | `bool` | `False` | When `vocab` is not provided, selects the built-in table used to tokenize: amino acids (`True`) or per-codon (`False`, default). Ignored if `vocab` is provided. |

=== "Raises"

    * `pyo3_runtime.PanicException`: If `kmer_size != 3` and no `vocab` is provided, or if `vocab` is missing a required special token (`[UNK]`, `[MASK]`, `[PAD]`, or `[CLS]` when `add_cls=True`).

---

## Attributes

All attributes below are read-only and reflect the values set at construction time.

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `kmer_size` | `int` | Size of the kmers used to tokenize sequences. |
| <span style="white-space: nowrap;">`drop_remainder`</span> | `bool` | Whether trailing partial kmers are dropped. |
| `pad_type` | `str` | `"before"` or `"after"` — where padding is inserted when `pad_length` is set. |
| `pad_length` | `int` | Target sequence length in bases, used for padding/truncation. `0` disables padding; `-1`/`-2` pad to the shortest/longest sequence in a given batch; a positive value pads/truncates to that exact base length. |
| `add_cls` | `bool` | Whether a `[CLS]` token is prepended to encoded sequences. |
| `vocab` | <span style="white-space: nowrap;">`dict[str,int]`</span> or `None` | Custom vocabulary mapping tokens to ids, if one was provided. `None` when using the built-in codon/amino-acid tables. |
| `reverse_vocab` | `dict[int,str]` or `None` | Reverse of `vocab` (id → token), automatically derived when a custom `vocab` is provided. `None` when using the built-in tables (decoding still works via the built-in reverse tables). |
| `vocab_size` | `int` | Number of entries in the vocabulary: the length of `vocab` when provided, otherwise `25` for `as_aa=True` or `70` for `as_aa=False`. |
| `as_aa` | `bool` | Whether the built-in amino-acid table is used instead of the per-codon table. Only relevant when `vocab` is `None`. |

---

## Methods

### `.seqs_to_id()` { #DNATokenizer.seqs_to_id }

```python
def seqs_to_id(sequences, n_jobs: int = 1)
```

Encodes sequences into integer token ids.

=== "Parameters"

    * **`sequences`**: The sequences to encode. Accepts a [`SequenceReader`](../load_seq/SequenceReader.md) (consumed directly), a Python `list`, or any iterable, where each item is one of:
        * a `str`, `bytes`, or `bytearray`
        * a Biopython `Seq`/`MutableSeq`
        * an object exposing a `.seq` or `.sequence` attribute (e.g. a needletail `SequenceRecord`, a Biopython `SeqRecord`, or a Pysam `FastxRecord`)
    * **`n_jobs`** (`int`, *optional*): Number of threads used to encode sequences in parallel. `0` uses all CPUs available. Defaults to `1`.

=== "Returns"

    * `list[numpy.ndarray]`: If `pad_length` is `0`. One 1-D `int32` array per sequence, each sized to that sequence's own token count.
    * `numpy.ndarray`: If `pad_length` is non-zero. A single 2-D `int32` array of shape `(n_sequences, length)`, padded/truncated per `pad_type`. `length` is derived from `pad_length` (in bases) converted to a token count via `kmer_size`.

=== "Raises"

    * `TypeError`: If an item in `sequences` is not a `str`, `bytes`/`bytearray`, a Biopython `Seq`/`MutableSeq`, or an object with a `.seq`/`.sequence` attribute.
    * `pyo3_runtime.PanicException`: If `kmer_size` is greater than the length of a sequence, if `kmer_size` is lower than `1`, if `pad_type` is neither `"before"` nor `"after"`, if `pad_length` is set below `-2`, or if `sequences` is empty while `pad_length` is non-zero.

---

### `.ids_to_seq()` { #DNATokenizer.ids_to_seq }

```python
def ids_to_seq(token_seq) -> str
```

Decodes a sequence of token ids back into its string representation by concatenating each id's corresponding token/kmer.

=== "Parameters"

    * **`token_seq`** (`list[int]` or `numpy.ndarray`): Sequence of token ids to decode, as produced by `.seqs_to_id()`.

=== "Returns"

    * `str`: The decoded sequence, formed by concatenating the token/kmer text for each id in order.

=== "Raises"

    * `ValueError`: If the decoded token bytes are not valid UTF-8.
    * `pyo3_runtime.PanicException`: If `token_seq` cannot be extracted as a list of integers (e.g. it is not a Python list or a numpy array of ints).

---

## Default Vocabularies

When `vocab` is not provided, `DNATokenizer` falls back to one of two built-in kmer_size=3 tables, both sharing the same special-token ids:

| Token | Id |
| :--- | :--- |
| `[CLS]` | `0` |
| `[SEP]` | `1` |
| `[BOS]` | `2` |
| `[MASK]` | `3` |
| `[PAD]` | `4` |
| `[RESERVED]` | `5` |
| `[UNK]` | `6` |

* **Codon table** (`as_aa=False`, default): each of the 64 codons gets its own id (`7`–`70`), reversible back to the canonical DNA codon via `.ids_to_seq()`. DNA (`t`) and RNA (`u`) spellings of the same codon map to the same id.
* **Amino-acid table** (`as_aa=True`): synonymous codons map to the same amino-acid id (`7`–`25`); start (`atg`/`aug`) reuses the `[BOS]` id and stop codons reuse the `[SEP]` id. Because this mapping is many-to-one, it cannot be reversed back to a specific codon — only to the amino-acid letter.

!!! note "Case Handling"
    Sequences are lowercased internally before tokenization, so mixed-case input is handled automatically.

!!! note "`pad_length` Is Measured in Bases, Not Tokens"
    `pad_length` (and the `-1`/`-2` shortest/longest resolution) operates on raw sequence length in bases. That base length is then converted into a column count using `kmer_size` to build the output array. Sequences that tokenize to more columns than the array provides are truncated; sequences that tokenize to fewer are padded with `[PAD]`.

---

## Exceptions

Most invalid-configuration and invalid-input errors surface as Rust panics, which PyO3 raises in Python as `pyo3_runtime.PanicException` (e.g. a bad `kmer_size`/`vocab` combination, an invalid `pad_type`, or a `pad_length` below `-2`). Two cases raise ordinary Python exceptions instead: `.seqs_to_id()` raises `TypeError` if an item in `sequences` isn't a supported type (`str`, `bytes`/`bytearray`, a Biopython `Seq`, or an object with a `.seq`/`.sequence` attribute), and `.ids_to_seq()` raises `ValueError` if the decoded bytes aren't valid UTF-8.

---

## Examples

### Basic Usage (Default Codon Vocabulary)

```python
import dna_parser as dps
sequences= ["agttcttctc","accttgtgt","acctgtcgc"]
tokenizer= dps.DNATokenizer()
tokens= tokenizer.seqs_to_id(sequences)
print(tokens)

#Output:
# [array([18, 62, 62,  6], dtype=int32), array([12, 69, 66], dtype=int32), array([12, 66, 32], dtype=int32)]

tokenizer.ids_to_seq(tokens[0])

#Output:
# 'agttcttct[UNK]'

```

### Encoding with a Fixed Padded Length

```python
import dna_parser as dps
sequences= ["agttcttctc","accttgtgt","acctgtcgc"]
tokenizer= dps.DNATokenizer(pad_lenght=20)
tokens= tokenizer.seqs_to_id(sequences)
print(tokens)

#Output:
# [[18 62 62  6  4  4  4]
#  [12 69 66  4  4  4  4]
#  [12 66 32  4  4  4  4]]

tokenizer.ids_to_seq(tokens[0])

#Output:
# 'agttcttct[UNK][PAD][PAD][PAD]'

```

### Encoding as Amino Acids

```python
import dna_parser as dps
sequences= ["agttcttctc","accttgtgt","acctgtcgc"]
tokenizer= dps.DNATokenizer(as_aa=True)
tokens= tokenizer.seqs_to_id(sequences)

tokens= tokenizer.seqs_to_id(sequences)
print(tokens)

#Output:
# [array([21, 21, 21,  6], dtype=int32), array([22, 16,  8], dtype=int32), array([22,  8, 20], dtype=int32)]

tokenizer.ids_to_seq(tokens[1])

#Output: as amino acids
# 'TLC'

```

### Using a Custom Vocabulary

```python
import dna_parser as dps
seq= ["aggaaccattgcaccata"]
vocab= {"[UNK]":0, "[CLS]":1, "[PAD]":2, "[MASK]":3, "acca":4,"ttgc":5,"ta":6}
tokenizer= dps.DNATokenizer(kmer_size=4, pad_length=0, vocab= vocab)
tokens= tokenizer.seqs_to_id(seq)

#Output:
#[array([0, 4, 5, 4, 6], dtype=int32)]

tokenizer.ids_to_seq(tokens[0])

#Output:
# '[UNK]accattgcaccata'
```