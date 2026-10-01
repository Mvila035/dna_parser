from .dna_parser import *
from scipy.sparse import csr_matrix
import numpy as np

class WrongTfidfConfig(Exception):
    pass

class NotFittedError(Exception):
    pass

class UnextendableCorpus(Exception):
    pass

class NoReader(Exception):
    pass

class Tfidf:

    def __init__(self, corpus, kmer, vocabulary=None, smooth_idf=True, original_idf=False, n_jobs=1, reader_fn=None, format=None):
        
        if isinstance(corpus, str) and reader_fn is None:
            raise WrongTfidfConfig("If the corpus is a str, it should be a path to a file, and a function to read it (reader_fn) should be provided")

        self.set_vocabulary(vocabulary)
        self.corpus = corpus
        self.kmer_size = kmer

        self.idf = None
        self.smooth_idf= smooth_idf
        self.original_idf= original_idf
        self.n_jobs = n_jobs
        
        self.reader_fn = reader_fn
        self.format = format

    def set_vocabulary(self, vocabulary):
        
        if vocabulary:
            self.vocabulary = Vocabulary(vocabulary)

        else:
            self.vocabulary = None

        self.is_idf_uptodate = False

    def set_threads(self, n_jobs):
        self.n_jobs = n_jobs

    def add_to_corpus(self, new_corpus):

        if isinstance(new_corpus, str):
            raise TypeError("The new corpus cannot be a str. If you want to add a single sequence pass it as list[str]")

        if hasattr(self.corpus, "extend") and callable(getattr(self.corpus, "extend")):
            self.corpus.extend(new_corpus)
        elif isinstance(self.corpus, list):
            self.corpus.extend(new_corpus)
        elif isinstance(self.corpus, set) or isinstance(self.corpus, tuple) :
            self.corpus = list(self.corpus) + list(new_corpus)
        else:
            raise UnextendableCorpus(("The corpus cannot be extended."
                                    "A corpus can only be extended if it is a collection of sequences (such as list[str], set[str], tuple[str])\n"
                                    "Check that your initial corpus is not a path to a file."))

        self.is_idf_uptodate = False

    def _get_sequence_source(self, sequences):
        """
        If sequences is a file path (string) and a reader_fn is provided, 
        invoke the reader to generate a fresh iterator. Otherwise, return sequences directly.
        """
        if isinstance(sequences, str) and self.reader_fn is not None:
            if self.format:
                return self.reader_fn(sequences, self.format)
            return self.reader_fn(sequences)

        if isinstance(sequences, str) and self.reader_fn is None:
            raise NoReader("A string (path/to/file) was passed but no reader function is specified.\nIf you want to encode a single sequence wrap it in a list -> ['agatatc'] ")
        return sequences

    def _reset_if_needed(self, sequences):
        """Resets native stateful readers (like Rust SequenceReader) if they support it."""
        if hasattr(sequences, "reset") and callable(getattr(sequences, "reset")):
            sequences.reset()


    def fit(self):
        seq_source = self._get_sequence_source(self.corpus)

        if not self.vocabulary:
            self.vocabulary = get_vocab(seq_source, self.kmer_size)
            self._reset_if_needed(seq_source)
            
            # If we just exhausted an iterator from reader_fn, we must spawn a fresh one for the transform step
            seq_source = self._get_sequence_source(self.corpus)

        self.idf= fit_rust(seq_source, self.vocabulary, self.kmer_size, self.smooth_idf, self.original_idf, self.n_jobs)
        
        self.is_idf_uptodate= True
        self._reset_if_needed(seq_source)

    def transform(self, sequences=None, l2_norm=True):
        if not self.is_idf_uptodate:
            raise NotFittedError("This Tfidf instance is not fitted. Please use fit() or fit_transform()")

        target_sequences = self.corpus if sequences is None else sequences
        seq_source = self._get_sequence_source(target_sequences)

        val, col, indptr = transform_rust(seq_source, self.vocabulary, self.kmer_size,
                                        l2_norm, self.smooth_idf, self.original_idf,
                                        self.n_jobs)
        self._reset_if_needed(seq_source)

        tfidf_csr = csr_matrix(
            arg1=(val, col, indptr),
            shape=(len(indptr)-1, len(self.vocabulary)),
            dtype=np.float64
        )

        return tfidf_csr

    def fit_transform(self, l2_norm=True):
        seq_source = self._get_sequence_source(self.corpus)

        if not self.vocabulary:
            self.vocabulary, val, col, indptr, self.idf = fit_transform_rust(seq_source, self.kmer_size,
                                                                            l2_norm, self.smooth_idf, self.original_idf,
                                                                            n_jobs= self.n_jobs)
        else:
            val, col, indptr, self.idf = fit_transform_with_voc(seq_source, self.vocabulary, self.kmer_size,
                                            l2_norm, self.smooth_idf, self.original_idf,
                                            n_jobs= self.n_jobs)

        self._reset_if_needed(seq_source)
        self.is_idf_uptodate=True

        tfidf_csr = csr_matrix(
            arg1=(val, col, indptr),
            shape=(len(indptr)-1, len(self.vocabulary)),
            dtype=np.float64
        )

        return tfidf_csr
