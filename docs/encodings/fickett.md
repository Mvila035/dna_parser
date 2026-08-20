# **Function: fickett_score**

```python
def fickett_score(sequences, n_jobs= 1) -> np.ndarray:
```

### **Description**
Compute the probability of each sequence to be a coding sequence. See the [About](../about/about.md#fickett-score) section for more details.

### **Parameters**

| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `sequences` | `any iterable containing str` | *Required* | Sequences to encode, can also be dna-parser `SequenceReader`, pysam `FastxFile` or Biopython `SeqIO.read` objects. |
| `n_jobs` | `int` | `1` | number of threads used to encode the sequences in parallel. 0 to use all CPUs available. |

### **Returns**

| Type | Description |
| :--- | :--- |
| `list[numpy.ndarray]` or `numpy.ndarray` | Returns a 1D numpy.ndarray. Each entry is the probability of the given sequence to be a coding sequence.|


### **Examples**

```python
import dna_parser as dps

sequences= ["agt","acc"]
encoding= dps.fickett_score(sequences)
print(encoding)

# Output:
#[0.3203 0.407 ]


reader= dps.SequenceReader("path/to/file") # file with sequences agt, acc, attagc
encoding= dps.fickett_score(reader)
print(encoding)
print(encoding.shape)

# Output:
#[0.3203     0.407      0.49100003]
#(3,)
```