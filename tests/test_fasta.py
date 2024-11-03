import pytest
from dna_parser import seq_from_fasta, metadata_from_fasta, load_fasta
import numpy

def test_load_fasta():
    
    expected= [ (">sequence1", "acgtatgcgtcgtc"),
    (">sequence2","cccgtga---gtcgat"),
    (">sequence3","xgtcgycaaatcg-?")]

    results= load_fasta("tests/seq_test.fasta")

    for expected_tup, result_tup in list(zip(expected,results)):
        
        print(expected_tup, result_tup)

        assert expected_tup == result_tup
    
    

def test_seq_from_fasta():
    
    expected= ["acgtatgcgtcgtc", "cccgtga---gtcgat", "xgtcgycaaatcg-?"]

    results= seq_from_fasta("tests/seq_test.fasta")

    assert expected == results


def test_seq_metadata_fasta():

    expected= [">sequence1", ">sequence2", ">sequence3"]

    results= metadata_from_fasta("tests/seq_test.fasta")

    assert expected == results