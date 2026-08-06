import pytest
from dna_parser import make_kmers

DNA_SEQUENCES = [
    "accgacatg",
    "accztapatgaa",
    "ACCGACATG"
]

def test_kmer():
    
    results= make_kmers(DNA_SEQUENCES)
    
    assert len(results[0]) == 3
    assert len(results[1]) == 4
    assert len(results[2]) == 3

def test_drop_remainder():

    results_drop= make_kmers(DNA_SEQUENCES, window_size=2, stride=2, drop_remainder=True)
    results_no_drop= make_kmers(DNA_SEQUENCES, window_size=2, stride=2, drop_remainder=False)
    
    assert len(results_drop[0]) == 4
    assert len(results_drop[1]) == 6
    assert len(results_drop[2]) == 4

    assert len(results_no_drop[0]) == 5
    assert len(results_no_drop[1]) == 6
    assert len(results_no_drop[2]) == 5

def test_window():
    
    results= make_kmers(DNA_SEQUENCES, window_size=4, stride=1)

    assert len(results[0][0]) == 4
    assert len(results[1][0]) == 4
    assert len(results[2][0]) == 4
    assert len(results[0][1]) == 4
    assert len(results[1][2]) == 4

def test_stride():

    results_no_drop= make_kmers(DNA_SEQUENCES, window_size=3, stride=1)
    results_drop= make_kmers(DNA_SEQUENCES, window_size=3, stride=1, drop_remainder=True)

    assert len(results_no_drop[0]) == 8
    assert len(results_no_drop[1]) == 11
    assert len(results_no_drop[2]) == 8

    assert len(results_drop[0]) == 7
    assert len(results_drop[1]) == 10
    assert len(results_drop[2]) == 7


