# **SequenceRecord Class**

`SequenceRecord` is the lightweight record type returned by `SequenceReader.get_records()`, holding a single sequence's id, sequence, quality, and source format.

## **Class Signature**

### `SequenceRecord`

```python
class SequenceRecord(seq, id, qual, format)
```

**Constructor Parameters:**

| Name | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `seq` | `str` | *Required* | The sequence string. |
| `id` | `str` | *Required* | The record identifier. |
| `qual` | `str` or `None` | *Required* | Per-base quality string. `None` for FASTA records, which don't carry quality scores. |
| `format` | `str` | *Required* | Source format of the record, e.g. `"Fasta"` or `"Fastq"`. |

!!! note "Direct Construction"
    `SequenceRecord` instances are normally produced by `SequenceReader.get_records()` rather than constructed directly, but the class can be instantiated manually since all four fields are plain, required arguments.

---

## Attributes

All attributes are readable and writable.

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `seq` | `str` | The sequence string. |
| `id` | `str` | The record identifier. |
| `qual` | `str` or `None` | Per-base quality string, or `None` if unavailable (e.g. FASTA records). |
| `format` | `str` | Source format of the record. |

---

## Methods

### `.__str__()` { #SequenceRecord.str}

```python
def __str__() -> str
```

Returns a human-readable summary of the record, invoked implicitly by `str()` and `print()`.

=== "Parameters"

    * None

=== "Returns"

    * `str`: A multi-line summary containing `id`, `format`, the first 20 characters of `seq` followed by `"..."`, and `qual`.


---

## Examples

### Constructing a Record Directly and Printing

```python
import dna_parser as dps

record= dps.SequenceRecord("agtcgtcgctttttgc", "Sequence1", qual=None, format= "Fasta")
print(record)

# Output:
#[ID: Sequence1
# Format: Fasta
# Sequence: agtcgtcgctttttgc...
# Quality: None]
```

