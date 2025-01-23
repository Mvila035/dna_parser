from .dna_parser import *
from scipy.sparse import csr_matrix
import numpy as np
from sklearn.utils.sparsefuncs_fast import inplace_csr_row_normalize_l2


class NotFittedError(Exception):
    pass

class Tfidf:

    def __init__(self, corpus , kmer, vocabulary= None, n_jobs=1):
        
        #check if corpus a string (path to fasta) or a list of string 
        #check if vocabulary is string (path to file) or a list of string 

        self.vocabulary= vocabulary
        self.corpus= corpus
        self.kmer_size= kmer
        self.idf= None
        self.is_idf_uptodate= False
        self.n_jobs= n_jobs
    
    def set_vocabulary(self, vocabulary):

        self.vocabulary= vocabulary
        self.is_idf_uptodate= False
    
    def set_threads(self, n_jobs):

        self.n_jobs= n_jobs
    
    
    def add_to_corpus(self, new_corpus):
        self.corpus.extend(new_corpus)
        self.is_idf_uptodate= False

    def compute_idf(self, matrix):

        term_freq= (matrix > 0).sum(0)
        self.idf= np.squeeze(np.asarray(np.log(matrix.shape[0]/term_freq)))
        self.is_idf_uptodate= True

    def fit(self):

        #check if vocabulary was set by user if not compile vocabulary !
        if not self.vocabulary:
            self.vocabulary= fit_rust(self.corpus, self.kmer_size, self.n_jobs)

        (val, row, col)= transform_rust(self.corpus, self.vocabulary, self.kmer_size, self.n_jobs)
        matrix_count= csr_matrix(arg1= (val,(row,col)),
                                        shape= (len(self.corpus),len(self.vocabulary)),
                                        dtype= np.float64)

        self.compute_idf(matrix_count)

        

    def transform(self, sequences= None, normalization= "L2"):
        
        #check sequences 
        
        if  not self.is_idf_uptodate:
            raise NotFittedError("This Tidf instance is not fitted. Please use fit()")
        
        if not sequences:
            val, row, col= transform_rust(self.corpus, self.vocabulary, self.kmer_size, self.n_jobs)
            matrix_count= csr_matrix(arg1= (val,(row,col)),
                                        shape= (len(self.corpus),len(self.vocabulary)),
                                        dtype= np.float64)

        else: 
            val, row, col= transform_rust(sequences, self.vocabulary, self.kmer_size, self.n_jobs)
            matrix_count= csr_matrix(arg1= (val,(row,col)),
                                        shape= (len(self.corpus),len(self.vocabulary)),
                                        dtype= np.float64)
        
        matrix_count.data *= np.take(a=self.idf, indices=matrix_count.indices)
        
        if normalization== "L2":
            inplace_csr_row_normalize_l2(matrix_count)
        
        return matrix_count
         
    
    def fit_transform(self, normalization= "L2"):
        
        if not self.vocabulary:
            
            self.vocabulary, val, row, col= fit_transform_rust(self.corpus, self.kmer_size, self.n_jobs)
        
        else:
            val, row, col= transform_rust(self.corpus, self.vocabulary, self.kmer_size, self.n_jobs)
        
        matrix_count= csr_matrix(arg1= (val,(row,col)),
                                        shape= (len(self.corpus),len(self.vocabulary)),
                                        dtype= np.float64)
        self.compute_idf(matrix_count)
    


        matrix_count.data *= np.take(a=self.idf, indices=matrix_count.indices)

        if normalization== "L2":
            inplace_csr_row_normalize_l2(matrix_count)

        return matrix_count

        

        

    
    