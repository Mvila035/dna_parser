import pytest
from dna_parser import fickett_score
import numpy

DNA_SEQUENCES = [
    "accgacatg",
    "accztapatgaa",
    "ACCGACATG"
]


def test_shape():

    results= fickett_score(DNA_SEQUENCES)

    assert results.shape == (3,)


def test_fickett_score():

    expected_score= 0.7148
    result= fickett_score([DNA_SEQUENCES[0]])[0]

    assert numpy.isclose(result, expected_score)

    
    

def test_caps():
    
    result1= fickett_score([DNA_SEQUENCES[0]])[0]
    result2= fickett_score([DNA_SEQUENCES[-1]])[0]

    assert result1 == result2


#for now only ACGT are maps any other char result in 0 vec
def test_unexpected_char():

    expected_score= 0.3458
    results= fickett_score([DNA_SEQUENCES[1]])[0]

    assert numpy.isclose(expected_score, results)
