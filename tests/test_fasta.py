import pytest
from dna_parser import SequenceReader, SequenceRecord, eiip_encoding, DNATokenizer
import numpy as np
from Bio import SeqIO
from pysam import FastxFile

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

def test_encode_from_file():

    reader_pysam= FastxFile("tests/seq_test.fasta")
    reader_biopython= SeqIO.parse("tests/seq_test.fasta", "fasta")
    reader_needletail= SequenceReader("tests/seq_test.fasta")

    assert eiip_encoding(reader_pysam, pad_length=-2).all() == eiip_encoding(reader_biopython, pad_length=-2).all()  == eiip_encoding(reader_needletail, pad_length=-2).all() 
    
    reader_pysam= FastxFile("tests/seq_test.fasta")
    reader_biopython= SeqIO.parse("tests/seq_test.fasta", "fasta")
    reader_needletail= SequenceReader("tests/seq_test.fasta")
    tokenizer= DNATokenizer(pad_length=-2)

    assert tokenizer.seqs_to_id(reader_pysam).all() == tokenizer.seqs_to_id(reader_biopython).all()  == tokenizer.seqs_to_id(reader_needletail).all() 
    
    reader_needletail= SequenceReader("tests/seq_test.fasta.xz")
    tokenizer= DNATokenizer(pad_length=0)

    assert isinstance(tokenizer.seqs_to_id(reader_needletail), list)
    reader_needletail.reset()
    assert isinstance(tokenizer.seqs_to_id(reader_needletail)[0], np.ndarray)
    