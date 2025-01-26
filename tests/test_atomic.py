import pytest
from dna_parser import atomic_encoding
import numpy

DNA_SEQUENCES = [
    "accgtc",
    "atpzg-",
    "ACCGTC"
]


def test_shape():

    results= atomic_encoding([DNA_SEQUENCES[0]])[0]

    assert results.shape == (6,)


def test_atomic():

    expected= numpy.array([70,58,58,78,66,58])
    results= atomic_encoding([DNA_SEQUENCES[0]])[0]

    for index, val in enumerate(results):

        assert val == expected[index]

    

def test_caps():
    
    results1= atomic_encoding([DNA_SEQUENCES[0]])[0]
    results2= atomic_encoding([DNA_SEQUENCES[-1]])[0]

    test_matrix= results1 == results2

    for index, x in numpy.ndenumerate(test_matrix):
        
        assert x



#for now only ACGT are maps any other char result in 0 vec
def test_unexpected_char():

    expected= numpy.array([70,66,0,0,78,0])
    results= atomic_encoding([DNA_SEQUENCES[1]])[0]

    for index, val in enumerate(results):

        assert val == expected[index]
