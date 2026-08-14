import pytest
from dna_parser import DNATokenizer
import numpy as np


DNA_SEQUENCES = [
    "accgacatg",
    "accztapatgaa",
    "ACCGACATG",
    "AggTTacg"
]

def test_tokenize():
    
    tokenizer= DNATokenizer()
    tokens= tokenizer.seqs_to_id(DNA_SEQUENCES, 1)

    expected= [
        np.array([12,40,21]),
        np.array([12,6,6,39]),
        np.array([12,40,21]),
        np.array([17,67,6]),
    ]

    assert type(tokens) == list
    np.testing.assert_allclose(tokens[0], tokens[2])
    np.testing.assert_allclose(tokens[1], expected[1])
    np.testing.assert_allclose(tokens[2], expected[2])
    np.testing.assert_allclose(tokens[3], expected[3])

    tokenizer= DNATokenizer(add_cls= True)

    expected= [
        np.array([0,12,40,21]),
        np.array([0,12,6,6,39]),
        np.array([0,12,40,21]),
        np.array([0,17,67,6]),
    ]

    tokens= tokenizer.seqs_to_id(DNA_SEQUENCES, 1)

    assert type(tokens) == list
    np.testing.assert_allclose(tokens[0], tokens[2])
    np.testing.assert_allclose(tokens[1], expected[1])
    np.testing.assert_allclose(tokens[2], expected[2])
    np.testing.assert_allclose(tokens[3], expected[3])

def test_padding():

    tokenizer= DNATokenizer(pad_type="after", pad_length=30)

    padded_tokens= tokenizer.seqs_to_id(DNA_SEQUENCES, n_jobs=1)

    assert isinstance(padded_tokens, np.ndarray)
    assert padded_tokens.shape == (4,10)
    assert padded_tokens[0][4] == 4 # 4 is [PAD] token
    assert padded_tokens[1][4] == 4
    assert padded_tokens[3][7] == 4

    tokenizer= DNATokenizer(pad_type="before", pad_length=30)
    padded_tokens= tokenizer.seqs_to_id(DNA_SEQUENCES, n_jobs=1)

    assert isinstance(padded_tokens, np.ndarray)
    assert padded_tokens.shape == (4,10)
    assert padded_tokens[0][0] == 4 # 4 is [PAD] token
    assert padded_tokens[1][1] == 4
    assert padded_tokens[3][6] == 4

def test_trimming():

    tokenizer= DNATokenizer(pad_type="after", pad_length=6)
    tokens= tokenizer.seqs_to_id(DNA_SEQUENCES, n_jobs=1)
    expected= [
        np.array([12,40]),
        np.array([12,6,]),
        np.array([12,40]),
        np.array([17,67]),
    ]

    np.testing.assert_allclose(tokens[0], expected[0])
    np.testing.assert_allclose(tokens[1], expected[1])
    np.testing.assert_allclose(tokens[2], expected[2])
    np.testing.assert_allclose(tokens[3], expected[3])
    
    tokenizer= DNATokenizer(pad_type="before", pad_length=6)
    tokens= tokenizer.seqs_to_id(DNA_SEQUENCES, n_jobs=1)
    expected= [
        np.array([40,21]),
        np.array([6,39]),
        np.array([40,21]),
        np.array([67,6]),
    ]

    np.testing.assert_allclose(tokens[0], expected[0])
    np.testing.assert_allclose(tokens[1], expected[1])
    np.testing.assert_allclose(tokens[2], expected[2])
    np.testing.assert_allclose(tokens[3], expected[3])

def test_vocab():
    
    vocab= {"[UNK]":0, "[CLS]":1, "[PAD]":2, "[MASK]":3, "acc":4}
    rev_vocab= {0:"[UNK]", 1:"[CLS]", 2:"[PAD]", 3:"[MASK]", 4:"acc"}
    tokenizer= DNATokenizer(pad_type="after", pad_length=30, vocab= vocab)

    assert tokenizer.vocab_size == 5
    assert tokenizer.reverse_vocab == rev_vocab

    tokens= tokenizer.seqs_to_id(DNA_SEQUENCES  )
    
    expected= [
        np.array([4,0,0,2,2,2,2,2,2,2]),
        np.array([4,0,0,0,2,2,2,2,2,2]),
        np.array([4,0,0,2,2,2,2,2,2,2]),
        np.array([0,0,0,2,2,2,2,2,2,2]),
    ]
    
    np.testing.assert_allclose(tokens[0], expected[0])
    np.testing.assert_allclose(tokens[1], expected[1])
    np.testing.assert_allclose(tokens[2], expected[2])
    np.testing.assert_allclose(tokens[3], expected[3])

def test_remainder():
    
    seq= ["accttaagata"]
    tokenizer= DNATokenizer(drop_remainder= False)
    tokens= tokenizer.seqs_to_id(seq, 1)[0]
    expected= np.array([12,67,15,6])

    np.testing.assert_allclose(expected, tokens)

    tokenizer= DNATokenizer(drop_remainder= True)
    tokens= tokenizer.seqs_to_id(seq, 1)[0]
    expected= np.array([12,67,15])

    np.testing.assert_allclose(expected, tokens)

def test_as_aa():

    tokenizer= DNATokenizer(as_aa= True)
    tokens= tokenizer.seqs_to_id(DNA_SEQUENCES, 1)

    expected= [
        np.array([22,9,2]),
        np.array([22,6,6,10]),
        np.array([22,9,2]),
        np.array([20,16,6]),
    ]

    assert type(tokens) == list
    np.testing.assert_allclose(tokens[0], tokens[2])
    np.testing.assert_allclose(tokens[1], expected[1])
    np.testing.assert_allclose(tokens[2], expected[2])
    np.testing.assert_allclose(tokens[3], expected[3])

def test_kmer():
    
    seq= ["aggaaccattgcaccata"]
    vocab= {"[UNK]":0, "[CLS]":1, "[PAD]":2, "[MASK]":3, "acca":4,"ttgc":5,"ta":6}
    rev_vocab= {0:"[UNK]", 1:"[CLS]", 2:"[PAD]", 3:"[MASK]", 4:"acc"}
    tokenizer= DNATokenizer(kmer_size=4, pad_length=0, vocab= vocab)

   
    tokens= tokenizer.seqs_to_id(seq, 1)[0]
    expected= np.array([0,4,5,4,6])

    np.testing.assert_allclose(expected, tokens)

def test_decode_ids():
    seq= ["aggaaccattgcaccata"]
    tokenizer= DNATokenizer(kmer_size=3, pad_length=0)
    tokens= tokenizer.seqs_to_id(seq)[0]
    recoded= tokenizer.ids_to_seq(tokens)

    assert seq[0] == recoded

    seq= ["aggaaccattgcacca"]
    tokenizer= DNATokenizer(kmer_size=3, pad_length=27)
    tokens= tokenizer.seqs_to_id(seq)[0]
    recoded= tokenizer.ids_to_seq(tokens)

    assert recoded == "aggaaccattgcacc[UNK][PAD][PAD][PAD]"