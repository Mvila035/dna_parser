from .dna_parser import *
from scipy.sparse import csr_matrix
import numpy as np
from sklearn.utils.sparsefuncs_fast import inplace_csr_row_normalize_l2


class NotFittedError(Exception):
    pass


class Tfidf:

    def __init__(self, corpus, kmer, vocabulary=None, n_jobs=1, reader_fn=None, format=None):
        self.vocabulary = vocabulary
        self.corpus = corpus
        self.kmer_size = kmer
        self.idf = None
        self.is_idf_uptodate = False
        self.n_jobs = n_jobs
        
        
        self.reader_fn = reader_fn
        self.format = format

    def set_vocabulary(self, vocabulary):
        self.vocabulary = vocabulary
        self.is_idf_uptodate = False

    def set_threads(self, n_jobs):
        self.n_jobs = n_jobs

    def add_to_corpus(self, new_corpus):
        if hasattr(self.corpus, "extend") and callable(getattr(self.corpus, "extend")):
            self.corpus.extend(new_corpus)
        elif isinstance(self.corpus, list):
            self.corpus.extend(new_corpus)
        else:
            self.corpus = list(self.corpus) + list(new_corpus)
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
        return sequences

    def _get_n_rows(self, sequences, row_indices):
        """Safely gets the row dimension. Bypasses __len__ if the input is a string path."""
        if isinstance(sequences, str):
            if len(row_indices) > 0:
                return int(np.max(row_indices)) + 1
            return 0
            
        if hasattr(sequences, "__len__"):
            return len(sequences)
        if len(row_indices) > 0:
            return int(np.max(row_indices)) + 1
        return 0

    def _reset_if_needed(self, sequences):
        """Resets native stateful readers (like Rust SequenceReader) if they support it."""
        if hasattr(sequences, "reset") and callable(getattr(sequences, "reset")):
            sequences.reset()

    def compute_idf(self, matrix):
        term_freq = (matrix > 0).sum(0)
        self.idf = np.squeeze(np.asarray(np.log(matrix.shape[0] / term_freq)))
        self.is_idf_uptodate = True

    def fit(self):
        seq_source = self._get_sequence_source(self.corpus)

        if not self.vocabulary:
            self.vocabulary = fit_rust(seq_source, self.kmer_size, self.n_jobs)
            self._reset_if_needed(seq_source)
            
            # If we just exhausted an iterator from reader_fn, we must spawn a fresh one for the transform step
            seq_source = self._get_sequence_source(self.corpus)

        val, row, col = transform_rust(seq_source, self.vocabulary, self.kmer_size, self.n_jobs)
        self._reset_if_needed(seq_source)

        n_rows = self._get_n_rows(self.corpus, row)
        matrix_count = csr_matrix(
            arg1=(val, (row, col)),
            shape=(n_rows, len(self.vocabulary)),
            dtype=np.float64
        )

        self.compute_idf(matrix_count)

    def transform(self, sequences=None, normalization="L2"):
        if not self.is_idf_uptodate:
            raise NotFittedError("This Tfidf instance is not fitted. Please use fit()")

        target_sequences = self.corpus if sequences is None else sequences
        seq_source = self._get_sequence_source(target_sequences)

        val, row, col = transform_rust(seq_source, self.vocabulary, self.kmer_size, self.n_jobs)
        self._reset_if_needed(seq_source)

        n_rows = self._get_n_rows(target_sequences, row)
        matrix_count = csr_matrix(
            arg1=(val, (row, col)),
            shape=(n_rows, len(self.vocabulary)),
            dtype=np.float64
        )

        matrix_count.data *= np.take(a=self.idf, indices=matrix_count.indices)

        if normalization == "L2":
            inplace_csr_row_normalize_l2(matrix_count)

        return matrix_count

    def fit_transform(self, normalization="L2"):
        seq_source = self._get_sequence_source(self.corpus)

        if not self.vocabulary:
            self.vocabulary, val, row, col = fit_transform_rust(seq_source, self.kmer_size, self.n_jobs)
        else:
            val, row, col = transform_rust(seq_source, self.vocabulary, self.kmer_size, self.n_jobs)

        self._reset_if_needed(seq_source)

        n_rows = self._get_n_rows(self.corpus, row)
        matrix_count = csr_matrix(
            arg1=(val, (row, col)),
            shape=(n_rows, len(self.vocabulary)),
            dtype=np.float64
        )

        self.compute_idf(matrix_count)

        matrix_count.data *= np.take(a=self.idf, indices=matrix_count.indices)

        if normalization == "L2":
            inplace_csr_row_normalize_l2(matrix_count)

        return matrix_count