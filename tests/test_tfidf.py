import pytest
from dna_parser import Tfidf
import numpy


DNA_SEQUENCES = [
    "accgcc",
    "aCCGcC",
    "ACCGCC",
    "accagt"

]

def test_caps():

    tfidf= Tfidf(DNA_SEQUENCES, kmer=3)
    tfidf.fit()
    encoding= tfidf.transform()
    encoding= encoding.toarray()
    

    for col in encoding.T :

        assert col[0] == col[1]
        assert col[1] == col[2]
    

def test_shape():

    tfidf= Tfidf(DNA_SEQUENCES, kmer=3)
    tfidf.fit()
    encodings= tfidf.transform()
    encodings= encodings.toarray()
    
    assert encodings.shape == (4,3)

def test_tfidf():

    expected= numpy.array([[-0.11157178, 0., 0.], [-0.11157178, 0., 0.],[-0.11157178, 0., 0.],[-0.11157178, 0., 0.34657359]]) 

    tfidf= Tfidf(DNA_SEQUENCES, kmer=3)
    encodings= tfidf.fit_transform()
    encodings= encodings.toarray()
    
    for iy, ix in numpy.ndindex(encodings.shape):

        assert numpy.isclose(encodings[iy, ix], expected[iy, ix])



   



