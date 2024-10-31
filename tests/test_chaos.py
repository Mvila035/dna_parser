import pytest
from dna_parser import chaos_encoding
import numpy

DNA_SEQUENCES = [
    "accgt",
    "atpzg-",
    "ACCGT"
]


def test_shape():

    results= chaos_encoding([DNA_SEQUENCES[0]])[0]

    assert results.shape == (2,5)

def test_padding_after():

    results= chaos_encoding(DNA_SEQUENCES, pad_length=10)

    assert results.shape == (3,2,10)
    assert results[0][0][-1] == results[0][0][-2]
    assert results[2][0][-1] == results[2][0][-2]

def test_padding_before():

    results= chaos_encoding(DNA_SEQUENCES, pad_type="before", pad_length=10)

    assert results.shape == (3,2,10)  
    assert results[0][0][0] == 0
    assert results[2][0][0] == 0

def test_chaos():

    expected= numpy.array([[0.5, -0.25, -0.625, 0.1875, -0.40625], [0.5,-0.25,-0.625, -0.8125, 0.09375]])

    results= chaos_encoding([DNA_SEQUENCES[0]])[0]

    print(results)

    test_matrix= results == expected

    for index, x in numpy.ndenumerate(test_matrix):
        
        if x == False:
            assert False
    
    assert True

def test_caps():
    
    results1= chaos_encoding([DNA_SEQUENCES[0]])[0]
    results2= chaos_encoding([DNA_SEQUENCES[-1]])[0]

    test_matrix= results1 == results2

    for index, x in numpy.ndenumerate(test_matrix):
        
        if x == False:
            assert False
    
    assert True

#for now only ACGT are mapped any other char result in not updating the values
def test_unexpected_char():
    expected= numpy.array([[0.5, -0.25, -0.25, -0.25, 0.375, 0.375 ], [0.5, 0.75, 0.75, 0.75, -0.125, -0.125]])

    results= chaos_encoding([DNA_SEQUENCES[1]])[0]

    test_matrix= results == expected

    for index, x in numpy.ndenumerate(test_matrix):
        
        if x == False:
            assert False
    
    assert True

