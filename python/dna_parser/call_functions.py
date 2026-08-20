from .dna_parser import *


def make_kmers(sequences, kmer_size=3, stride=3, drop_remainder=False, n_jobs=1):

    seqs = make_kmers_rust(sequences, kmer_size, stride, drop_remainder)
    results = [seq.view(f'S{kmer_size}').reshape(-1).astype(str).tolist() for seq in seqs]

    if not drop_remainder:
        for kmer_list in results:
            if kmer_list:
                kmer_list[-1] = kmer_list[-1].rstrip('\x00')

    return results
