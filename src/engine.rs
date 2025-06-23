use crate::document::Document;
use crate::index::{Index, InvertedIndex};
use crate::tokenizer::{Tokenizer, SimpleTokenizer};

pub struct SearchEngine<T: Tokenizer, I: Index = InvertedIndex> {
    tokenizer: T,
    index: I,
}

impl<T: Tokenizer, I: Index> SearchEngine<T, I> {
    pub fn new(tokenizer: T, index: I) -> Self {
        Self {
            tokenizer,
            index,
        }
    }

    pub fn add_document(&mut self, doc: Document) {
        let terms = self.tokenizer.tokenize(&doc.content);
        self.index.add(doc.id, &terms);
    }

    pub fn search(&self, query: &str) -> Vec<u32> {
        let terms = self.tokenizer.tokenize(query);
        let set = self.index.query(&terms);
        set.into_iter().collect()
    }
}

impl Default for SearchEngine<SimpleTokenizer, InvertedIndex> {
    fn default() -> Self {
        Self::new(SimpleTokenizer, InvertedIndex::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_search_engine() {
        let _engine = SearchEngine::new(SimpleTokenizer, InvertedIndex::default());
    }

    #[test]
    fn test_add_and_search_single_document() {
        let mut engine = SearchEngine::new(SimpleTokenizer, InvertedIndex::default());
        let doc = Document::new(1, "hello world".to_string());
        
        engine.add_document(doc);
        let results = engine.search("hello");
        
        assert_eq!(results, vec![1]);
    }

    #[test]
    fn test_add_and_search_multiple_documents() {
        let mut engine = SearchEngine::new(SimpleTokenizer, InvertedIndex::default());
        let doc1 = Document::new(1, "hello world".to_string());
        let doc2 = Document::new(2, "hello rust".to_string());
        let doc3 = Document::new(3, "world rust".to_string());
        
        engine.add_document(doc1);
        engine.add_document(doc2);
        engine.add_document(doc3);
        
        let results = engine.search("hello world");
        assert_eq!(results, vec![1]);
        
        let results = engine.search("hello");
        let mut expected = vec![1, 2];
        expected.sort();
        let mut actual = results;
        actual.sort();
        assert_eq!(actual, expected);
        
        let results = engine.search("rust");
        let mut expected = vec![2, 3];
        expected.sort();
        let mut actual = results;
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_search_nonexistent_term() {
        let mut engine = SearchEngine::new(SimpleTokenizer, InvertedIndex::default());
        let doc = Document::new(1, "hello world".to_string());
        
        engine.add_document(doc);
        let results = engine.search("nonexistent");
        
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_empty_query() {
        let mut engine = SearchEngine::new(SimpleTokenizer, InvertedIndex::default());
        let doc = Document::new(1, "hello world".to_string());
        
        engine.add_document(doc);
        let results = engine.search("");
        
        assert!(results.is_empty());
    }

    #[test]
    fn test_case_insensitive_search() {
        let mut engine = SearchEngine::new(SimpleTokenizer, InvertedIndex::default());
        let doc = Document::new(1, "Hello World".to_string());
        
        engine.add_document(doc);
        let results = engine.search("hello");
        
        assert_eq!(results, vec![1]);
    }
} 