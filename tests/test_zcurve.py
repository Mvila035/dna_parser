import pytest
from dna_parser import zcurve_encoding
import numpy

DNA_SEQUENCES = [
    "accgt",
    "atpzg-",
    "ACCGT"
]


def test_shape():

    results= zcurve_encoding([DNA_SEQUENCES[0]])[0]

    assert results.shape == (3,5)

def test_padding_after():

    results= zcurve_encoding(DNA_SEQUENCES, pad_length=10)

    assert results.shape == (3,3,10)
    assert results[0][0][-1] == results[0][0][-2]
    assert results[2][0][-1] == results[2][0][-2]

def test_padding_before():

    results= zcurve_encoding(DNA_SEQUENCES, pad_type="before", pad_length=10)

    assert results.shape == (3,3,10)  
    assert results[0][0][0] == 0
    assert results[2][0][0] == 0

def test_zcurve():

    expected= numpy.array([[1,0,-1,0,-1], [1,2,3,2,1], [1,0,-1,-2,-1]])

    results= zcurve_encoding([DNA_SEQUENCES[0]])[0]

    test_matrix= results == expected

    for index, x in numpy.ndenumerate(test_matrix):
        
        if x == False:
            assert False
    
    assert True

def test_caps():
    
    results1= zcurve_encoding([DNA_SEQUENCES[0]])[0]
    results2= zcurve_encoding([DNA_SEQUENCES[-1]])[0]

    test_matrix= results1 == results2

    for index, x in numpy.ndenumerate(test_matrix):
        
        if x == False:
            assert False
    
    assert True

#for now only ACGT are mapped any other char result in not updating the values
def test_unexpected_char():
    expected= numpy.array([[1,0,0,0,1,1], [1,0,0,0,-1,-1], [1,2,2,2,1,1]])

    results= zcurve_encoding([DNA_SEQUENCES[1]])[0]

    test_matrix= results == expected

    for index, x in numpy.ndenumerate(test_matrix):
        
        if x == False:
            assert False
    
    assert True