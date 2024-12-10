from .dna_parser import *


def onehot_encoding(sequences, pad_type= "after", pad_length= -2, n_jobs= 1):

    return onehot_encoding_rust(sequences, pad_type, pad_length, n_jobs)


def ordinal_encoding(sequences, pad_type= "after", pad_length= -2, n_jobs= 1):

    return ordinal_encoding_rust(sequences, pad_type, pad_length, n_jobs)

def cross_encoding(sequences, pad_type= "after", pad_length= -2, n_jobs= 1):
    
    return cross_encoding_rust(sequences, pad_type, pad_length, n_jobs)


def random_seq(length, nb_of_seq, seq_type= "dna", n_jobs= 1):

    return random_seq_rust(length, nb_of_seq, seq_type, n_jobs)

def zcurve_encoding(sequences, pad_type= "after", pad_length= -2, n_jobs= 1):

    return zcurve_encoding_rust(sequences, pad_type, pad_length, n_jobs)

def chaos_encoding(sequences, pad_type= "after", pad_length= -2, n_jobs= 1):

    return chaos_encoding_rust(sequences, pad_type, pad_length, n_jobs)

def eiip_encoding(sequences, pad_type= "after", pad_length= -2, n_jobs= 1):

    return eiip_encoding_rust(sequences, pad_type, pad_length, n_jobs)

def dna_encoding(sequences, pad_type= "after", pad_length= -2, n_jobs= 1):

    return chaos_encoding_rust(sequences, pad_type, pad_length, n_jobs)

def dna_walk(sequences, pad_type= "after", pad_length= -2, n_jobs= 1):

    return dna_walk_rust(sequences, pad_type, pad_length, n_jobs)

def fickett_score(sequences, n_jobs= 1):
    
    return fickett_score_rust(sequences, n_jobs)

def atomic_encoding(sequences, pad_type= "after", pad_length= -2, n_jobs= 1):

    return atomic_encoding_rust(sequences, pad_type, pad_length, n_jobs)