# **Function: random_seq**

```python
def random_seq(length, nb_of_seq, seq_type= "dna", n_jobs= 1) -> list[str]:
```

### **Description**
Generates random sequences from a uniform probability distribution.

### **Parameters**

| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `length` | `int` | *Required* | Length of sequences to generate. |
| `nb_of_seq` | `int` | *Required* | Number of sequences to generate.|
| `seq_type` | `str` | `dna` | Type of sequences. Either `dna`, `rna`, or `aa` for amino acids.|
| `n_jobs` | `int` | `1` | Number of threads used to generate sequences in parallel. |



### **Returns**

| Type | Description |
| :--- | :--- |
| `list[str]` | Returns a list containing the sequences |


### **Examples**

```python
import dna_parser as dps

sequences= dps.random_seq(15,3)
print(sequences)

# Output:
# ['tagtccaaccacttg', 'gcagtactaaactca', 'caaggccatgaggta']
```