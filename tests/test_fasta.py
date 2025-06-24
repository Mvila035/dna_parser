import pytest
from dna_parser import load_sequences, load_metadata, load_fasta
import numpy

def test_load_fasta():
    
    expected= [ ("sequence1", "acgtatgcgtcgtc"),
    ("sequence2","cccgtga---gtcgat"),
    ("sequence3","xgtcgycaaatcg-?")]

    results= load_fasta("tests/seq_test.fasta")

    for expected_tup, result_tup in list(zip(expected,results)):
        
        print(expected_tup, result_tup)

        assert expected_tup == result_tup
    
    

def test_seq_from_fasta():
    
    expected= ["acgtatgcgtcgtc", "cccgtga---gtcgat", "xgtcgycaaatcg-?"]

    results= load_sequences("tests/seq_test.fasta")

    assert expected == results


def test_seq_metadata_fasta():

    expected= ["sequence1", "sequence2", "sequence3"]

    results= load_metadata("tests/seq_test.fasta")

    assert expected == results

def test_list_of_paths():
    paths= ["tests/seq_test.fasta","tests/seq_test.fasta","tests/seq_test.fasta"]

    fasta_expected= [ ("sequence1", "acgtatgcgtcgtc"), ("sequence2","cccgtga---gtcgat"), ("sequence3","xgtcgycaaatcg-?")]*3
    metadata_expected= ["sequence1", "sequence2", "sequence3"]*3
    sequences_expected= ["acgtatgcgtcgtc", "cccgtga---gtcgat", "xgtcgycaaatcg-?"]*3
    
    fasta= load_fasta(paths)
    metadata= load_metadata(paths)
    sequences= load_sequences(paths)

    assert fasta == fasta_expected
    assert metadata == metadata_expected
    assert sequences == sequences_expected
   


