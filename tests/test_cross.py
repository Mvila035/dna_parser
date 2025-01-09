import pytest
from dna_parser import cross_encoding
import numpy

DNA_SEQUENCES = [
    "acc",
    "atpzg-",
    "ACC"
]


def test_shape():

    results= cross_encoding([DNA_SEQUENCES[0]])[0]

    assert results.shape == (3,2)


def test_ordinal():

    expected= numpy.array([[1,1],[-1,-1],[-1,-1]])

    results= cross_encoding([DNA_SEQUENCES[0]])[0]

    test_matrix= results == expected

    for index, x in numpy.ndenumerate(test_matrix):
        
        assert x
    


def test_caps():
    
    results1= cross_encoding([DNA_SEQUENCES[0]])[0]
    results2= cross_encoding([DNA_SEQUENCES[-1]])[0]

    test_matrix= results1 == results2

    for index, x in numpy.ndenumerate(test_matrix):
        
        assert x

#for now only ACGT are maps any other char result in [0,0] vec
def test_unexpected_char():
    expected= numpy.array([[1,1],[1,-1], [0,0], [0,0], [-1,1], [0,0]])

    results= cross_encoding([DNA_SEQUENCES[1]])[0]

    test_matrix= results == expected

    for index, x in numpy.ndenumerate(test_matrix):
        
        assert x