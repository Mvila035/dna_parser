# dna-parser
![Build Status](https://github.com/Mvila035/dna_parser/workflows/CI/badge.svg)

dna-parser is a Python library written in Rust to encode (or perform feature extraction on) DNA/RNA sequences for machine learning.

the source code is available on [GitHub](https://github.com/Mvila035/dna_parser)

## Installation

To install dna-parser simply run:
```sh
pip install dna-parser
```

If there is no Python wheel available for your OS you can install Rust and re-install dna-parser which should now compile on your machine.
Run the following command on Unix-like OS to install Rust:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
or see more options at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## Quick Start

```python
import dna_parser as dps

sequences= ["agt","acc"]
encodings= dps.onehot_encoding(sequences)
print(encodings)

# Output:
#[[[0 0 1 0]
#  [0 1 0 0]
#  [0 0 0 1]]

# [[0 0 1 0]
#  [1 0 0 0]
#  [1 0 0 0]]]
```
All encodings with examples are available in the [Documentation](https://mvila035.github.io/dna_parser/documentation/) section.
