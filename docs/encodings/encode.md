# **Function: encode**

```python
def encode(sequences, mapping, default_value, padding_value=None, pad_type="after", pad_length=0, n_jobs=1) -> list[np.ndarray] | np.ndarray:
```

### **Description**
Encode sequences according to a mapping defined by the user. 


### **Parameters**

| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `sequences` | `any iterable containing str` | *Required* | Sequences to encode, can also be dna-parser `SequenceReader`, pysam `FastxFile` or Biopython `SeqIO.read` objects. |
| `mapping` | `dict[str, int/float]` or `dict[str, 1D array]` | *Required* | Mappings used to encode the sequences. The mapping can map a character to a value or to a 1D list/numpy.ndarray |
| <span style="white-space: nowrap;">`default_value`</span> | `int/float/1D array` | *Required* | Value to use if a character encountered in a sequence is not in `mapping`. Must be of the same type than the values in `mapping` |
| <span style="white-space: nowrap;">`padding_value`</span> | `int/float/1D array/None`  | `None` | Value used to pad the encoding. If `None`defaults to `default_value` |
| `pad_type` | `str` | `after` | Whether to trim/pad at the front or at the back of the sequences. Options are `before` and `after`. This paramater is ignored if there is no padding |
| <span style="white-space: nowrap;">`pad_length`</span> | `int` | `0` | Length for padding/trimming. -2 to pad to the longest sequence, -1 to trim to the shortest sequence, any positive integer for a fixed length. |
| `n_jobs` | `int` | `1` | number of threads used to encode the sequences in parallel. 0 to use all CPUs available. |

### **Returns**

| Type | Description |
| :--- | :--- |
| `list[numpy.ndarray]` or `numpy.ndarray` | The encoded sequences. Returns a list of numpy arrays if there is no trimming/padding, otherwise returns a  numpy array with the outermost dimension representing sequences. |


### **Examples**

```python
import dna_parser as dps

sequences= ["agt-","acc-"]
mapping= {"a":0.2, "g":-0.1, "t":-0.2, "c":0.3}
encoding= dps.encode(sequences, mapping, default_value=-1.0)
print(encoding)

# Output:
#[array([ 0.2, -0.1, -0.2, -1. ]), array([ 0.2,  0.3,  0.3, -1. ])]


reader= dps.SequenceReader("path/to/file") # file with sequences agt, acc, attagc
mapping= {"a":[0,4], "g":[0,3], "t":[0,2], "c":[0,1]}
encoding= dps.encode(reader, mapping, default_value=[0,0], padding_value=[-1,-1], pad_type="after", pad_length=-2) #pad after the sequences to the longest one
print(encoding)
print(encoding.shape)

# Output:
#[[[ 0.  4.]
#  [ 0.  3.]
#  [ 0.  2.]
#  [-1. -1.]
#  [-1. -1.]
#  [-1. -1.]]
#
# [[ 0.  4.]
#  [ 0.  1.]
#  [ 0.  1.]
#  [-1. -1.]
#  [-1. -1.]
#  [-1. -1.]]
#
# [[ 0.  4.]
#  [ 0.  2.]
#  [ 0.  2.]
#  [ 0.  4.]
#  [ 0.  3.]
#  [ 0.  1.]]]
#  
#(3, 6, 2)
```