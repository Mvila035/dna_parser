import pytest
from dna_parser import ordinal_encoding
import numpy

DNA_SEQUENCES = [
    "acc",
    "atpzg-",
    "ACC"
]


def test_shape():

    results= ordinal_encoding([DNA_SEQUENCES[0]])[0]

    assert results.shape == (3,)


def test_ordinal():

    expected= numpy.array([0.25, 0.50, 0.50])

    results= ordinal_encoding([DNA_SEQUENCES[0]])[0]

    test_matrix= results == expected

    for index, x in numpy.ndenumerate(test_matrix):
        
        assert x
    

def test_caps():
    
    results1= ordinal_encoding([DNA_SEQUENCES[0]])[0]
    results2= ordinal_encoding([DNA_SEQUENCES[-1]])[0]

    test_matrix= results1 == results2

    for index, x in numpy.ndenumerate(test_matrix):
        
        assert x



#for now only ACGT are maps any other char result in 0 vec
def test_unexpected_char():
    expected= numpy.array([0.25, 1.0, 0, 0, 0.75, 0 ])

    results= ordinal_encoding([DNA_SEQUENCES[1]])[0]

    test_matrix= results == expected

    for index, x in numpy.ndenumerate(test_matrix):
        
        assert x