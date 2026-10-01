# **Function: chaos_encoding**

```python
def chaos_encoding(sequences, pad_type= "after", pad_length= 0, n_jobs= 1) -> list[np.ndarray] | np.ndarray:
```

### **Description**
Each sequence is encoded in a square with vertices A: (1,1), C: (-1,-1),
G: (1,-1), T/U: (-1,1). The sequence representation starts at the center of the square in (0,0). The first nucleotide is represented as a point halfway between the starting point and its corresponding vertice. 
Each following nucleotide a new point halfaway between the previous point and its corresponding vertice. If a character other than A,C,G,T or U is encountered, the values are not updated and values from the previous point are used.

See the [About](../about/about.md#chaos-game) section for more details.

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
| `list[numpy.ndarray]` or `numpy.ndarray` | The encoded sequences. Returns a list of 2D numpy arrays if there is no trimming/padding, otherwise returns a 3D numpy array with the outer dimension representing sequences. |


### **Examples**

```python
import dna_parser as dps

sequences= ["agt","acc"]
encoding= dps.chaos_encoding(sequences)
print(encoding)

# Output:
#[array([[ 0.5  ,  0.5  ],
#        [ 0.75 , -0.25 ],
#        [-0.125,  0.375]], dtype=float32),
# array([[ 0.5  ,  0.5  ],
#        [-0.25 , -0.25 ],
#        [-0.625, -0.625]], dtype=float32)]


reader= dps.SequenceReader("path/to/file") # file with sequences agt, acc, attagc
encoding= dps.chaos_encoding(reader, pad_type="after", pad_length=-2) #pad after the sequences to the longest one
print(encoding)
print(encoding.shape)

# Output:
#[[[ 0.5       0.5     ]
#  [ 0.75     -0.25    ]
#  [-0.125     0.375   ]
#  [-0.125     0.375   ]
#  [-0.125     0.375   ]
#  [-0.125     0.375   ]]
#
# [[ 0.5       0.5     ]
#  [-0.25     -0.25    ]
#  [-0.625    -0.625   ]
#  [-0.625    -0.625   ]
#  [-0.625    -0.625   ]
#  [-0.625    -0.625   ]]
#
# [[ 0.5       0.5     ]
#  [-0.25      0.75    ]
#  [-0.625     0.875   ]
#  [ 0.1875    0.9375  ]
#  [ 0.59375  -0.03125 ]
#  [-0.203125 -0.515625]]]
#(3, 6, 2)
```