import pytest
from dna_parser import dna_walk
import numpy

DNA_SEQUENCES = [
    "accgt",
    "atpzg-",
    "ACCGT"
]


def test_shape():

    results= dna_walk([DNA_SEQUENCES[0]])[0]

    assert results.shape == (5,2)

def test_padding_after():

    results= dna_walk(DNA_SEQUENCES, pad_length=10)

    assert results.shape == (3,10,2)
    assert results[0][-1][0] == results[0][-2][0]
    assert results[2][-1][0] == results[2][-2][0]

def test_padding_before():

    results= dna_walk(DNA_SEQUENCES, pad_type="before", pad_length=10)

    assert results.shape == (3,10,2)  
    assert results[0][0][0] == 0
    assert results[2][0][0] == 0

def test_dna_walk():

    expected= numpy.array([[-1,0],[-1,-1],[-1,-2],[-1,-1],[0,-1]])
    
    results= dna_walk([DNA_SEQUENCES[0]])[0]

    for iy, ix in numpy.ndindex(expected.shape):

        assert results[iy, ix] == expected[iy, ix]
    

def test_caps():
    
    results1= dna_walk([DNA_SEQUENCES[0]])[0]
    results2= dna_walk([DNA_SEQUENCES[-1]])[0]

    for iy, ix in numpy.ndindex(results1.shape):

        assert results1[iy, ix] == results2[iy, ix]

#for now only ACGT are mapped any other char result in not updating the values
def test_unexpected_char():
    expected= numpy.array([[-1,0],[0,0],[0,0],[0,0],[0,1],[0,1]])

    results= dna_walk([DNA_SEQUENCES[1]])[0]

    for iy, ix in numpy.ndindex(expected.shape):

        assert results[iy, ix] == expected[iy, ix]

