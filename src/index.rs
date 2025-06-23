use std::collections::{HashMap, HashSet};

pub trait Index {
    /// Adds a document's terms to the index.
    /// 
    /// # Arguments
    /// * `doc_id` - The unique identifier of the document
    /// * `terms` - The tokenized terms from the document content
    fn add(&mut self, doc_id: u32, terms: &[String]);

    /// Queries the index for documents containing all the specified terms.
    /// 
    /// # Arguments
    /// * `terms` - The search terms to look for
    /// 
    /// # Returns
    /// A set of document IDs that contain all the specified terms
    fn query(&self, terms: &[String]) -> HashSet<u32>;
}

/// An in-memory inverted index implementation
pub struct InvertedIndex {
    map: HashMap<String, HashSet<u32>>, // term -> {doc_id}
}

impl Index for InvertedIndex {
    fn add(&mut self, doc_id: u32, terms: &[String]) {
        for term in terms {
            self.map.entry(term.clone()).or_default().insert(doc_id);
        }
    }

    fn query(&self, terms: &[String]) -> HashSet<u32> {
        if terms.is_empty() {
            return HashSet::new();
        }
        
        let mut result = self.map.get(&terms[0]).cloned().unwrap_or_default();
        
        for term in &terms[1..] {
            if let Some(set) = self.map.get(term) {
                result = result.intersection(set).copied().collect();
            } else {
                return HashSet::new();
            }
        }
        
        result
    }
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn all_doc_ids(&self) -> HashSet<u32> {
        let mut doc_ids = HashSet::new();
        for doc_set in self.map.values() {
            doc_ids.extend(doc_set);
        }
        doc_ids
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl Default for InvertedIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_document() {
        let mut index = InvertedIndex::new();
        let terms = vec!["hello".to_string(), "world".to_string()];
        
        index.add(1, &terms);
        
        assert_eq!(index.len(), 2);
        assert!(index.map.contains_key("hello"));
        assert!(index.map.contains_key("world"));
    }

    #[test]
    fn test_query_single_term() {
        let mut index = InvertedIndex::new();
        let terms1 = vec!["hello".to_string(), "world".to_string()];
        let terms2 = vec!["hello".to_string(), "rust".to_string()];
        
        index.add(1, &terms1);
        index.add(2, &terms2);
        
        let result = index.query(&["hello".to_string()]);
        assert_eq!(result, HashSet::from([1, 2]));
    }

    #[test]
    fn test_query_multiple_terms() {
        let mut index = InvertedIndex::new();
        let terms1 = vec!["hello".to_string(), "world".to_string()];
        let terms2 = vec!["hello".to_string(), "rust".to_string()];
        let terms3 = vec!["world".to_string(), "rust".to_string()];
        
        index.add(1, &terms1);
        index.add(2, &terms2);
        index.add(3, &terms3);
        
        let result = index.query(&["hello".to_string(), "world".to_string()]);
        assert_eq!(result, HashSet::from([1]));
    }

    #[test]
    fn test_query_nonexistent_term() {
        let mut index = InvertedIndex::new();
        let terms = vec!["hello".to_string(), "world".to_string()];
        
        index.add(1, &terms);
        
        let result = index.query(&["nonexistent".to_string()]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_query_empty_terms() {
        let index = InvertedIndex::new();
        let result = index.query(&[]);
        assert!(result.is_empty());
    }
} 