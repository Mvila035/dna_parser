import pytest
from dna_parser import eiip_encoding
import numpy

DNA_SEQUENCES = [
    "acc",
    "atpzg-",
    "ACC"
]


def test_shape():

    results= eiip_encoding([DNA_SEQUENCES[0]])[0]

    assert results.shape == (3,)


def test_eiip():

    expected= numpy.array([0.1260, 0.1340, 0.1340])
    results= eiip_encoding([DNA_SEQUENCES[0]])[0]

    for index, val in enumerate(results):

        assert numpy.isclose(val, expected[index])

    

def test_caps():
    
    results1= eiip_encoding([DNA_SEQUENCES[0]])[0]
    results2= eiip_encoding([DNA_SEQUENCES[-1]])[0]

    test_matrix= results1 == results2

    for index, x in numpy.ndenumerate(test_matrix):
        
        assert x



#for now only ACGT are maps any other char result in 0 vec
def test_unexpected_char():

    expected= numpy.array([0.1260, 0.1335, 0, 0, 0.0806, 0 ])
    results= eiip_encoding([DNA_SEQUENCES[1]])[0]

    for index, val in enumerate(results):

        assert numpy.isclose(val, expected[index])
