pub trait Tokenizer {
    fn tokenize(&self, text: &str) -> Vec<String>;
}

pub struct SimpleTokenizer;

impl Tokenizer for SimpleTokenizer {
    fn tokenize(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|t| t.to_lowercase())
            .filter(|t| !t.is_empty())
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

    #[test]
    fn test_tokenizer_japanese() {
        let tokenizer = SimpleTokenizer;
        let tokens = tokenizer.tokenize("こんにちは 世界");
        assert_eq!(tokens, vec!["こんにちは", "世界"]);
    }

    #[test]
    fn test_tokenizer_mixed_languages() {
        let tokenizer = SimpleTokenizer;
        let tokens = tokenizer.tokenize("Hello こんにちは World 世界");
        assert_eq!(tokens, vec!["hello", "こんにちは", "world", "世界"]);
    }

    #[test]
    fn test_tokenizer_unicode_characters() {
        let tokenizer = SimpleTokenizer;
        let tokens = tokenizer.tokenize("café résumé naïve");
        assert_eq!(tokens, vec!["café", "résumé", "naïve"]);
    }

    #[test]
    fn test_tokenizer_multiple_spaces() {
        let tokenizer = SimpleTokenizer;
        let tokens = tokenizer.tokenize("  hello   world  ");
        assert_eq!(tokens, vec!["hello", "world"]);
    }

    #[test]
    fn test_tokenizer_japanese_sentence() {
        let tokenizer = SimpleTokenizer;
        // 日本語の文章は空白がないため、1つのトークンになってしまう
        let tokens = tokenizer.tokenize("こんにちは世界です");
        assert_eq!(tokens, vec!["こんにちは世界です"]);
        
        // 空白で区切られた場合は分割される
        let tokens = tokenizer.tokenize("こんにちは 世界です");
        assert_eq!(tokens, vec!["こんにちは", "世界です"]);
    }
} 