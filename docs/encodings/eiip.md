# **Function: eiip_encoding**

```python
def eiip_encoding(sequences, pad_type= "after", pad_length= 0, n_jobs= 1) -> list[np.ndarray] | np.ndarray:
```

### **Description**
Encodes sequences converting each nucleotide to its electron-ion interaction pseudopotential (EIIP):

* A= 0.1260
* C= 0.1340
* G= 0.0806
* T/U= 0.1335
* Other characters or gaps = 0.0

See the [About](../about/about.md#eiip) section for more details.

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
| `list[numpy.ndarray]` or `numpy.ndarray` | The encoded sequences. Returns a list of 1D numpy arrays if there is no trimming/padding, otherwise returns a 2D numpy array with rows representing sequences. |


### **Examples**

```python
import dna_parser as dps

sequences= ["agt","acc"]
encoding= dps.eiip_encoding(sequences)
print(encoding)

# Output:
#[array([0.126 , 0.0806, 0.1335]), array([0.126, 0.134, 0.134])]


reader= dps.SequenceReader("path/to/file") # file with sequences agt, acc, attagc
encoding= dps.eiip_encoding(reader, pad_type="after", pad_length=-2) #pad after the sequences to the longest one
print(encoding)
print(encoding.shape)

# Output:
#[[0.126  0.0806 0.1335 0.     0.     0.    ]
# [0.126  0.134  0.134  0.     0.     0.    ]
# [0.126  0.1335 0.1335 0.126  0.0806 0.134 ]]
#
#(3, 6)
```