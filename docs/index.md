<h1 style="text-align:center;"><b>dna-parser</b></h1>

<p style="text-align:center;"><b>dna-parser is a Python library written in Rust to encode (or perform feature extraction on) DNA/RNA sequences for machine learning.</b></p>

The source code is available on [GitHub](https://github.com/Mvila035/dna_parser)

## **Installation**

To install dna-parser run:
```sh
pip install dna-parser
```

If there is no Python wheel available for your OS, you can install Rust and re-install dna-parser which should now compile on your machine.
Run the following command on Unix-like OS to install Rust:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
or see more options at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## **Quick Start**

### Loading Sequences

```python
import dna_parser as dps

reader= dps.SequenceReader("path/to/file")
reader.get_sequences()
# Output:
# [agcgcggtgcgc, agcggcgcgtata, ...]

reader.reset() # read all sequences and exhausted the reader so we reset it.
reader.get_ids()
# Output:
#['sequence1', 'sequence2', 'sequence3']

reader.reset()
for seq in reader.get_records():
    print(seq) # seq -> SequenceRecord object
               # with attributes .id .format .seq .qual (if Fastq file)
# Output:
#[ID: sequence1
# Format: Fasta
# Sequence: acgtatgcgtcgtc...
# Quality: None]
#
#[ID: sequence2
# ...
```
If the file does not fit in memory you can use the `batch_size` option to return a list containing the desired number of objects.

```python
import dna_parser as dps

reader= dps.SequenceReader("path/to/file.fasta", batch_size=1)
reader.get_sequences()
# Output:
# [agcgcggtgcgc]
reader.get_sequences()
# Output:
# [agcggcgcgtata]
```

### Encoding Sequences
Encoding functions accept any iterables (list, set, tuple etc.) containing strings. It also accepts dna-parser `SequenceReader`, pysam `FastxFile` and Biopython `SeqIO.read` objects.
```python
import dna_parser as dps

sequences= ["agt","acc"]
encodings= dps.onehot_encoding(sequences)
print(encodings)
# Output: list of numpy ndarray
#[[[0 0 1 0]
#  [0 1 0 0]
#  [0 0 0 1]]

# [[0 0 1 0]
#  [1 0 0 0]
#  [1 0 0 0]]]

reader= dps.SequenceReader("path/to/file") #or pysam.FastxFile("path/to/file") or SeqIO.parse("path/to/file", "fasta")
encodings= dps.eiip_encoding(reader)
print(encodings)
# Output: list of numpy ndarray
#[array([0.126 , 0.134 , 0.0806, 0.1335, 0.126 , 0.1335, 0.0806, 0.134]),
# array([0.134 , 0.134 , 0.134 , 0.0806, 0.1335, 0.0806, 0.126 , 0.   ],
# ... ]

```

Most encodings provide options to pad/trim the sequence and to encode them in parallel for shorter encoding time.
```python
import dna_parser as dps

sequences= ["agt","accgt"]                                
encodings= dps.eiip_encoding(sequences, pad_type="after", pad_length=-2, n_jobs=2) #-2 pad to longest sequence, -1 trim to shortest
print(encodings)                                                                   # 0 no padding, 1+ fixed length trim/padding 
# Output: numpy ndarray
# array([[0.126 , 0.0806, 0.1335, 0.    , 0.    ],
#        [0.126 , 0.134 , 0.134 , 0.0806, 0.1335]])
```
All encodings with examples are available in the [Documentation](documentation.md) section.
