import pytest
from dna_parser import SequenceReader, SequenceRecord
import numpy

def test_load_fasta():

    expected= ["acgtatgcgtcgtc", "cccgtga---gtcgat", "xgtcgycaaatcg-?"]
    reader= SequenceReader("tests/seq_test.fasta")
    results= reader.get_records()

    for res, exp in list(zip(results, expected)):
        
        assert isinstance(res, SequenceRecord)
        assert res.seq == exp
    

def test_seq_from_fasta():
    
    expected= ["acgtatgcgtcgtc", "cccgtga---gtcgat", "xgtcgycaaatcg-?"]

    reader= SequenceReader("tests/seq_test.fasta")
    results= reader.get_sequences()

    assert expected == results


def test_seq_metadata_fasta():

    expected= ["sequence1", "sequence2", "sequence3"]

    reader= SequenceReader("tests/seq_test.fasta")
    results= reader.get_ids()

    assert expected == results

# def test_list_of_paths():
#     paths= ["tests/seq_test.fasta","tests/seq_test.fasta","tests/seq_test.fasta"]

#     fasta_expected= [ ("sequence1", "acgtatgcgtcgtc"), ("sequence2","cccgtga---gtcgat"), ("sequence3","xgtcgycaaatcg-?")]*3
#     metadata_expected= ["sequence1", "sequence2", "sequence3"]*3
#     sequences_expected= ["acgtatgcgtcgtc", "cccgtga---gtcgat", "xgtcgycaaatcg-?"]*3
    
#     fasta= load_fasta(paths)
#     metadata= load_metadata(paths)
#     sequences= load_sequences(paths)

#     assert fasta == fasta_expected
#     assert metadata == metadata_expected
#     assert sequences == sequences_expected
   


def test_reset():

    expected= ["sequence1", "sequence2", "sequence3"]

    reader= SequenceReader("tests/seq_test.fasta")
    results= reader.get_ids()

    assert expected == results

    reader.reset()

    expected= ["acgtatgcgtcgtc", "cccgtga---gtcgat", "xgtcgycaaatcg-?"]

    reader= SequenceReader("tests/seq_test.fasta")
    results= reader.get_sequences()

    assert expected == results
    