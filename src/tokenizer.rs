pub trait Tokenizer {
    fn tokenize(&self, text: &str) -> Vec<String>;
}

pub struct SimpleTokenizer;

impl Tokenizer for SimpleTokenizer {
    fn tokenize(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|t| t.to_ascii_lowercase())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokenizer() {
        let tokenizer = SimpleTokenizer;
        let tokens = tokenizer.tokenize("Hello World Rust");
        assert_eq!(tokens, vec!["hello", "world", "rust"]);
    }

    #[test]
    fn test_tokenizer_empty() {
        let tokenizer = SimpleTokenizer;
        let tokens = tokenizer.tokenize("");
        assert_eq!(tokens, Vec::<String>::new());
    }

    #[test]
    fn test_tokenizer_case_insensitive() {
        let tokenizer = SimpleTokenizer;
        let tokens = tokenizer.tokenize("RUST BOOK");
        assert_eq!(tokens, vec!["rust", "book"]);
    }
} 