from .dna_parser import *


def onehot_encoding(sequences, pad_type= "after", pad_length= 0, n_jobs= 1):

    return onehot_encoding_rust(sequences, pad_type, pad_length, n_jobs)



def ordinal_encoding(sequences, pad_type= "after", pad_length= 0, n_jobs= 1):

    return ordinal_encoding_rust(sequences, pad_type, pad_length, n_jobs)


def random_seq(length, nb_of_seq, seq_type= "dna", n_jobs= 1):

    return random_seq_rust(length, nb_of_seq, seq_type, n_jobs)