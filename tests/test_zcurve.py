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

    assert results.shape == (5,3)

def test_padding_after():

    results= zcurve_encoding(DNA_SEQUENCES, pad_length=10)

    assert results.shape == (3,10,3)
    assert results[0][0][-1] == results[0][0][-2]
    assert results[2][0][-1] == results[2][0][-2]

def test_padding_before():

    results= zcurve_encoding(DNA_SEQUENCES, pad_type="before", pad_length=10)

    assert results.shape == (3,10,3)  
    assert results[0][0][0] == 0
    assert results[2][0][0] == 0

def test_zcurve():

    expected= numpy.array([[1,1,1], [0,2,0], [-1,3,-1], [0,2,-2], [-1,1,-1]])

    results= zcurve_encoding([DNA_SEQUENCES[0]])[0]

    test_matrix= results == expected

    for index, x in numpy.ndenumerate(test_matrix):
        
        assert x

def test_caps():
    
    results1= zcurve_encoding([DNA_SEQUENCES[0]])[0]
    results2= zcurve_encoding([DNA_SEQUENCES[-1]])[0]

    test_matrix= results1 == results2

    for index, x in numpy.ndenumerate(test_matrix):
        
        assert x

#for now only ACGT are mapped any other char result in not updating the values
def test_unexpected_char():
    expected= numpy.array([[1,1,1], [0,0,2], [0,0,2], [0,0,2], [1,-1,1], [1,-1,1]]) 
    results= zcurve_encoding([DNA_SEQUENCES[1]])[0]

    print(results)
    test_matrix= results == expected

    for index, x in numpy.ndenumerate(test_matrix):
        
        assert x

def test_window():
    expected= numpy.array([ [0,0,2], [0,0,2], [1,-1,1]]) 
    results= zcurve_encoding([DNA_SEQUENCES[1]], window= 2)[0]

    test_matrix= results == expected

    for index, x in numpy.ndenumerate(test_matrix):
        
        assert x

def test_window():
    expected= numpy.array([ [0,0,2], [0,0,2], [1,-1,1]]) 
    results= zcurve_encoding([DNA_SEQUENCES[1]], window= 2)[0]

    test_matrix= results == expected

    for index, x in numpy.ndenumerate(test_matrix):
        
        assert x
    
def before_test_window():
    expected= numpy.array([ [0,0,0],[0,0,0],[0,0,2], [0,0,2], [1,-1,1]]) 
    results= zcurve_encoding([DNA_SEQUENCES[1]],pad_length=10, pad_type="before",window= 2)[0]

    test_matrix= results == expected

    for index, x in numpy.ndenumerate(test_matrix):
        
        assert x

