from .dna_parser import *


def make_kmers(sequences, window_size= 3, stride= 3, drop_remainder= False, n_jobs= 1):

    return make_kmers_rust(sequences, window_size, stride, drop_remainder, n_jobs)

