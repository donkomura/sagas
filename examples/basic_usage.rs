use sagas::engine::SearchEngine;
use sagas::document::Document;
use sagas::tokenizer::SimpleTokenizer;
use sagas::index::InvertedIndex;

fn main() {
    let mut engine = SearchEngine::new(SimpleTokenizer, InvertedIndex::default());
    
    let documents = vec![
        Document::new(1, "Rust is a systems programming language".to_string()),
        Document::new(2, "Python is a high-level programming language".to_string()),
        Document::new(3, "Rust book teaches systems programming".to_string()),
        Document::new(4, "JavaScript is a web programming language".to_string()),
        Document::new(5, "Rust and Python are both popular languages".to_string()),
        Document::new(6, "Rustはシステムプログラミング言語です".to_string()),
        Document::new(7, "Pythonは高級プログラミング言語です".to_string()),
        Document::new(8, "JavaScriptはWebプログラミング言語です".to_string()),
        Document::new(9, "RustとPythonは両方とも人気のある言語です".to_string()),
        Document::new(10, "プログラミング language programming".to_string()),
    ];
    
    for doc in documents {
        engine.add_document(doc);
    }
    
    println!("検索エンジンにドキュメントを追加しました");
    
    let queries = vec![
        "rust",
        "programming",
        "rust programming",
        "python",
        "javascript",
        "nonexistent",
        "Rust",
        "プログラミング",
        "言語",
        "システム",
        "language",
    ];
    
    for query in queries {
        let results = engine.search(query);
        if results.is_empty() {
            println!("検索語 '{}': 結果なし", query);
        } else {
            println!("検索語 '{}': ドキュメントID {:?}", query, results);
        }
    }
} 