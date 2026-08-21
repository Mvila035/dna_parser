from unittest import result
import pytest
from dna_parser import encode
import numpy as np

DNA_SEQUENCES = [
    "accgtc",
    "atpzg-",
    "ACCGTC"
]


def test_shape():

    mappings= {"a":1,"c":2,"g":3}

    results= encode(DNA_SEQUENCES, mappings, default_value= -2, pad_length=-2)

    assert results.shape == (3,6)

    mappings= {"a":[0,2],"c":[0,3],"g":[0,4]}

    results= encode(DNA_SEQUENCES, mappings, default_value= [0,1], pad_length=-2)
    assert results.shape == (3,6,2)



def test_encode():

    mappings= {"a":1,"c":2,"g":3}
    expected= np.array([[1,2,2,3,-2,2], [1,-2,-2,-2,3,-2], [1,2,2,3,-2,2] ])
    results= encode(DNA_SEQUENCES, mappings, default_value= -2, pad_length=-2)

    np.testing.assert_allclose(expected, results)

   

    

def test_caps():
    
    mappings= {"a":[0,2],"c":[0,3],"g":[0,5]}
    results= encode(DNA_SEQUENCES, mappings, default_value= [0,-1], pad_length=0)

    np.testing.assert_allclose(results[0], results[-1])


def test_unexpected_char():

    mappings= {"a":[0,2],"c":[0,3],"g":[0,5]}
    default_value= np.array([0,-1])
    result= encode(DNA_SEQUENCES, mappings, default_value= default_value, pad_length=0)[1]

    np.testing.assert_allclose(result[1], default_value)
    np.testing.assert_allclose(result[2], default_value)
    np.testing.assert_allclose(result[-1], default_value)

def test_no_padding():

    mappings= {"a":[0,2],"c":[0,3],"g":[0,5]}
    default_value= np.array([0,-1])
    results= encode(DNA_SEQUENCES, mappings, default_value= default_value, pad_length=0)
    
    assert type(results) is list
    assert type(results[0]) is np.ndarray

def test_padding_after():
    seqs= [
    "accgtc",
    "atpzg-agtagt",
    "ACCGT"
    ]

    mappings= {"a":[0,2],"c":[0,3],"g":[0,5]}
    default_value= np.array([0,-1])
    results= encode(seqs, mappings, default_value= default_value, pad_type= "after", pad_length=-1)

    assert results.shape == (3,5,2)
    np.testing.assert_allclose(results[0][0], [0,2])
    np.testing.assert_allclose(results[-1][-1], default_value)

    mappings= {"a":[0,2],"c":[0,3],"g":[0,5]}
    default_value= np.array([0,-1])
    results= encode(seqs, mappings, default_value= default_value, pad_type= "after", pad_length=-2)
    np.testing.assert_allclose(results[0][0], [0,2])
    np.testing.assert_allclose(results[1][-1], default_value)
    np.testing.assert_allclose(results[1][-2], [0,5])
    np.testing.assert_allclose(results[-1][-1], default_value)

    mappings= {"a":[0,2],"c":[0,3],"g":[0,5]}
    default_value= np.array([0,-1])
    results= encode(seqs, mappings, default_value= default_value, pad_type= "after", pad_length=20)
    assert results.shape == (3,20,2)
    np.testing.assert_allclose(results[0][-1], default_value)
    np.testing.assert_allclose(results[1][-1], default_value)
    np.testing.assert_allclose(results[2][-10], default_value)
   
def test_padding_before():

    seqs= [
    "accgtc",
    "atpzg-agtagt",
    "ACCGT"
    ]

    mappings= {"a":[0,2],"c":[0,3],"g":[0,5]}
    default_value= np.array([0,-1])
    results= encode(seqs, mappings, default_value= default_value, pad_type= "before", pad_length=-1)

    assert results.shape == (3,5,2)
    np.testing.assert_allclose(results[0][0], [0,3])
    np.testing.assert_allclose(results[-1][0], [0,2])

    mappings= {"a":[0,2],"c":[0,3],"g":[0,5]}
    default_value= np.array([0,-1])
    results= encode(seqs, mappings, default_value= default_value, pad_type= "before", pad_length=-2)
    np.testing.assert_allclose(results[0][6], [0,2])
    np.testing.assert_allclose(results[1][0], [0,2])
    np.testing.assert_allclose(results[1][1], default_value)
    np.testing.assert_allclose(results[-1][0], default_value)

    mappings= {"a":[0,2],"c":[0,3],"g":[0,5]}
    default_value= np.array([0,-1])
    results= encode(seqs, mappings, default_value= default_value, pad_type= "before", pad_length=20)
    assert results.shape == (3,20,2)
    np.testing.assert_allclose(results[0][0], default_value)
    np.testing.assert_allclose(results[1][0], default_value)
    np.testing.assert_allclose(results[2][10], default_value)

def test_pad_value():
    seqs= [
    "accgtc",
    "atpzg-agtagt",
    "ACCGT"
    ]
    mappings= {"a":[0,2],"c":[0,3],"g":[0,5]}
    default_value= np.array([0,-1])
    results= encode(seqs, mappings, default_value= default_value, pad_type= "after", pad_length=-2)
    
    np.testing.assert_allclose(results[0][-1], default_value)

    mappings= {"a":[0,2],"c":[0,3],"g":[0,5]}
    default_value= np.array([0,-1])
    results= encode(seqs, mappings, default_value= default_value, padding_value= [-1,-5], pad_type= "after", pad_length=-2)

    np.testing.assert_allclose(results[0][-1], [-1,-5])
