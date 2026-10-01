# **Function: zcurve_encoding**

```python
def zcurve_encoding(sequences, pad_type= "after", pad_length= 0,  downsampling=1, drop_remainder= False, n_jobs= 1) -> list[np.ndarray] | np.ndarray:
```

### **Description**
the Z-curve encoding gives the disparity between purines (r) and pyrimidines (y), the disparity between nucleotides with an amino (m) and a keto (k) group, and the disparity between nucleotide with weak (w) and strong (s) bonds at positons along the sequence.

ee the [About](../about/about.md#z-curve) section for more details.


### **Parameters**

| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `sequences` | `any iterable containing str` | *Required* | Sequences to encode, can also be dna-parser `SequenceReader`, pysam `FastxFile` or Biopython `SeqIO.read` objects. |
| `pad_type` | `str` | `after` | Whether to trim/pad at the front or at the back of the sequences. Options are `before` and `after`. This paramater is ignored if there is no padding |
| <span style="white-space: nowrap;">`pad_length`</span> | `int` | `0` | Length for padding/trimming. -2 to pad to the longest sequence, -1 to trim to the shortest sequence, any positive integer for a fixed length. |
| `downsampling` | `int` | `1` | Determine how often the disparities are reported along the sequence. Default to 1 (every base).|
| `drop_remainder` | `bool` | `False` | Wether to drop the rest of the sequence when the next position at which to report disparities falls outside the sequence.|
| `n_jobs` | `int` | `1` | number of threads used to encode the sequences in parallel. 0 to use all CPUs available.|

### **Returns**

| Type | Description |
| :--- | :--- |
| `list[numpy.ndarray]` or `numpy.ndarray` | The encoded sequences. Returns a list of 2D numpy arrays if there is no trimming/padding, otherwise returns a 3D numpy array with the outer dimension representing sequences. |


### **Examples**

```python
import dna_parser as dps

sequences= ["agt","acc"]
encoding= dps.zcurve_encoding(sequences)
print(encoding)

# Output:
#        r-y  m-k  w-s 
#[array([[ 1,  1,  1],
#        [ 2,  0,  0],
#        [ 1, -1,  1]], dtype=int32),
# array([[ 1,  1,  1],
#        [ 0,  2,  0],
#        [-1,  3, -1]], dtype=int32)]


reader= dps.SequenceReader("path/to/file") # file with sequences agt, acc, attagc
encoding= dps.zcurve_encoding(reader, pad_type="after", pad_length=-2) #pad after the sequences to the longest one
print(encoding)
print(encoding.shape)

#Output:
# r-y  m-k  w-s 
#[[[ 1  1  1]
#  [ 0  2  0]
#  [ 1  1 -1]
#  [ 0  0  0]
#  [ 1  1  1]
#  [ 0  0  2]
#  [ 1 -1  1]
#  [ 0  0  0]
#  [ 1 -1 -1]
#  [ 0 -2  0]
#  [-1 -1 -1]
#  [ 0 -2 -2]
#  [-1 -3 -1]
#  [-2 -2 -2]
#  [-2 -2 -2]
#  [-2 -2 -2]]
#
# [[-1  1 -1]
#  [-2  2 -2]
#  [-3  3 -3]
#  [-2  2 -4]
#  [-3  1 -3]
#  [-2  0 -4]
#  [-1  1 -3]
#  [-1  1 -3]
#  [-1  1 -3]
#  [-1  1 -3]
#  [ 0  0 -4]
#  [-1 -1 -3]
#  [-2  0 -4]
#  [-1 -1 -5]
#  [ 0  0 -4]
#  [-1 -1 -3]]
#
# [[ 0  0  0]
#  [ 1 -1 -1]
#  [ 0 -2  0]
#  [-1 -1 -1]
#  [ 0 -2 -2]
#  [ 0 -2 -2]
#  [-1 -1 -3]
#  [ 0  0 -2]
#  [ 1  1 -1]
#  [ 2  2  0]
#  [ 1  1  1]
#  [ 0  2  0]
#  [ 1  1 -1]
#  [ 1  1 -1]
#  [ 1  1 -1]
#  [ 1  1 -1]]]
#(3, 16, 3)

```