# **Function: make_kmers**

```python
def make_kmers(sequences, kmer_size=3, stride=3, drop_remainder=False) -> list[list[str]]:
```

### **Description**


### **Parameters**

| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `sequences` | `any iterable containing str` | *Required* | Sequences to encode, can also be dna-parser `SequenceReader`, pysam `FastxFile` or Biopython `SeqIO.read` objects.|
| `kmer_size` | `int` | `3` | Size of the kmers when splitting the sequences.|
| `stride` | `int` | `3` | Determines by how many residues the kmer window moves when splitting the sequence.|
| `drop_remainder` | `bool` | `False` | Determines whether to keep the last kmer with size < kmer_size if the stride and kmer_size do not perfectly split the sequence.|

### **Returns**

| Type | Description |
| :--- | :--- |
| `list[list[str]]` | The strings represent the kmers, and the inner list represents the collection of kmer for a sequence.|


### **Examples**

```python
import dna_parser as dps

sequences= ["agttcttctc","accttgtg"]
dps.make_kmers(sequences, kmer_size=3, stride=3)

#Output:
# [['agt', 'tct', 'tct', 'c'], ['acc', 'ttg', 'tg']]

sequences= ["agttcttctc","accttgtg"]
dps.make_kmers(sequences, kmer_size=3, stride=3, drop_remainder=True)

#Output:
# [['agt', 'tct', 'tct'], ['acc', 'ttg']]

```