from typing import List
import pytest
from dna_parser import zcurve_encoding
import numpy as np

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

    expected= np.array([[1,1,1], [0,2,0], [-1,3,-1], [0,2,-2], [-1,1,-1]])

    results= zcurve_encoding([DNA_SEQUENCES[0]])[0]

    test_matrix= results == expected

    for index, x in np.ndenumerate(test_matrix):
        
        assert x

def test_caps():
    
    results1= zcurve_encoding([DNA_SEQUENCES[0]])[0]
    results2= zcurve_encoding([DNA_SEQUENCES[-1]])[0]

    test_matrix= results1 == results2

    for index, x in np.ndenumerate(test_matrix):
        
        assert x

#for now only ACGT are mapped any other char result in not updating the values
def test_unexpected_char():
    expected= np.array([[1,1,1], [0,0,2], [0,0,2], [0,0,2], [1,-1,1], [1,-1,1]]) 
    results= zcurve_encoding([DNA_SEQUENCES[1]])[0]

    print(results)
    test_matrix= results == expected

    for index, x in np.ndenumerate(test_matrix):
        
        assert x

def test_downsampling():
    expected= np.array([ [0,0,2], [0,0,2], [1,-1,1]]) 
    results= zcurve_encoding([DNA_SEQUENCES[1]], downsampling= 2)[0]

    test_matrix= results == expected

    for index, x in np.ndenumerate(test_matrix):
        
        assert x

def test_drop_remainder():
    expected= np.array([ [0,2,0], [0,2,-2] ]) 
    results= zcurve_encoding([DNA_SEQUENCES[2]], downsampling= 2, drop_remainder=True)[0]

    test_matrix= results == expected

    for index, x in np.ndenumerate(test_matrix):
        
        assert x

    
def test_before_downsampling():
    expected= np.array([[0,0,-2], [-1,-1,-1]]) 
    expected_drop= np.array([[0,0,-2]])
    results= zcurve_encoding(DNA_SEQUENCES, pad_length=3, pad_type="before", downsampling= 2, drop_remainder=False)[0]
    results_drop= zcurve_encoding(DNA_SEQUENCES, pad_length=3, pad_type="before", downsampling= 2, drop_remainder=True)[0]

  
    assert results.shape == expected.shape
    assert np.allclose(expected, results)

    assert expected_drop.shape == results_drop.shape
    assert np.allclose(expected_drop, results_drop)

def test_no_padding():
    
    results= zcurve_encoding(DNA_SEQUENCES, pad_length=0)
    expected0=np.array([[1,1,1], [0,2,0], [-1,3,-1], [0,2,-2], [-1,1,-1]])
    expected1= np.array([[1,1,1], [0,0,2], [0,0,2], [0,0,2], [1,-1,1], [1,-1,1]]) 
    expected2= np.array([[1,1,1], [0,2,0], [-1,3,-1], [0,2,-2], [-1,1,-1]])

    assert type(results) is list
    assert results[0].shape == (5,3)
    assert results[1].shape == (6,3)
    assert results[2].shape == (5,3)

    assert np.allclose(results[0],expected0)
    assert np.allclose(results[1],expected1)
    assert np.allclose(results[2],expected2)



