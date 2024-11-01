from .dna_parser import *
from scipy.sparse import csr_matrix


class NotFittedError(Exception):
    pass

class Tfidf:

    def __init__(self, corpus , kmer, vocabulary= None):
        
        #check if corpus a string (path to fasta) or a list of string 
        #check if vocabulary is string (path to file) or a list of string 

        self.vocabulary= vocabulary
        self.corpus= corpus
        self.kmer_size= kmer
        self.df= None
        self.idf= None
        self.is_df_uptodate= False
        self.is_idf_uptodate= False
    
    def set_vocabulary(self, vocabulary):

        #check if vocabulary is string (path to file) or a list of string or dict<str,int>
        #transform voc to 
        self.vocabulary= vocabulary
        self.is_df_uptodate= False
        self.is_idf_uptodate= False
    

    def set_corpus(self, corpus):
        self.corpus= corpus
        self.is_df_uptodate= False
        self.is_idf_uptodate= False
    
    def add_to_corpus(self, new_corpus):
        self.corpus= self.corpus.extend(new_corpus)
        self.is_df_uptodate= False
        self.is_idf_uptodate= False

    def fit(self):

        #check if vocabulary was set by user if not compile vocabulary !
        #add warning if no vocabulary is provided 
        #add error if no corpus is provided
        if not self.vocabulary:
            self.vocabulary= dict()
            self.vocabulary= map_vocabulary_rust(self.corpus, self.vocabulary, self.kmer_size)
            self.is_voc_uptodate= True


        self.df, self.idf= fit_idf_rust(self.corpus, self.vocabulary, self.kmer_size)
        
        self.is_df_uptodate= True
        self.is_idf_uptodate= True

    def transform(self, sequences= None):
        
        #check sequences 
        
        if not self.is_df_uptodate or not self.is_idf_uptodate:
            raise NotFittedError("This Tidf instance is not fitted. Please use fit()")
        
        if not sequences:
            data, rows, cols, shape= transform_idf_rust(self.corpus, self.vocabulary, self.idf, self.kmer_size)

        else: 
            data, rows, cols, shape= transform_idf_rust(sequences, self.vocabulary, self.idf, self.kmer_size)


        return csr_matrix((data,(rows,cols)), shape= shape)
    
    def fit_transform(self, sequences= None):
        
        self.fit()
        return self.transform(sequences)
    