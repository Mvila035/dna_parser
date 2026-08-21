# **Function: onehot_encoding**

```python
def onehot_encoding(sequences, pad_type= "after", pad_length= 0, n_jobs= 1) -> list[np.ndarray] | np.ndarray:
```

### **Description**
Each nucleotide is encoded as follows:

* C= [1,0,0,0]
* G= [0,1,0,0]
* A= [0,0,1,0]
* T/U= [0,0,0,1]
* Other characters or gaps = [0,0,0,0]

### **Parameters**

| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `sequences` | `any iterable containing str` | *Required* | Sequences to encode, can also be dna-parser `SequenceReader`, pysam `FastxFile` or Biopython `SeqIO.read` objects. |
| `pad_type` | `str` | `after` | Whether to trim/pad at the front or at the back of the sequences. Options are `before` and `after`. This paramater is ignored if there is no padding |
| <span style="white-space: nowrap;">`pad_length`</span> | `int` | `0` | Length for padding/trimming. -2 to pad to the longest sequence, -1 to trim to the shortest sequence, any positive integer for a fixed length. |
| `n_jobs` | `int` | `1` | number of threads used to encode the sequences in parallel. 0 to use all CPUs available. |

### **Returns**

| Type | Description |
| :--- | :--- |
| `list[numpy.ndarray]` or `numpy.ndarray` | The encoded sequences. Returns a list of 2D numpy arrays if there is no trimming/padding, otherwise returns a 3D numpy array with the outermost dimension representing the sequences. |


### **Examples**

```python
import dna_parser as dps

sequences= ["agt","acc"]
encoding= dps.onehot_encoding(sequences)
print(encoding)

# Output:
#[array([[0, 0, 1, 0],
#        [0, 1, 0, 0],
#        [0, 0, 0, 1]], dtype=int32),
# array([[0, 0, 1, 0],
#        [1, 0, 0, 0],
#        [1, 0, 0, 0]], dtype=int32)]


reader= dps.SequenceReader("path/to/file") # file with sequences agt, acc, attagc
encoding= dps.onehot_encoding(reader, pad_type="after", pad_length=-2) #pad after the sequences to the longest one
print(encoding)
print(encoding.shape)

# Output:
#[[[0 0 1 0]
#  [0 1 0 0]
#  [0 0 0 1]
#  [0 0 0 0]
#  [0 0 0 0]
#  [0 0 0 0]]
#
# [[0 0 1 0]
#  [1 0 0 0]
#  [1 0 0 0]
#  [0 0 0 0]
#  [0 0 0 0]
#  [0 0 0 0]]
#
# [[0 0 1 0]
#  [0 0 0 1]
#  [0 0 0 1]
#  [0 0 1 0]
#  [0 1 0 0]
#  [1 0 0 0]]]
#(3, 6, 4)
```