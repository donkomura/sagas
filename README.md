# Sagas - シンプルな全文検索エンジン

Rustで実装されたシンプルで拡張可能な全文検索エンジンです。

## 機能

- ドキュメントの追加とインデックス化
- 複数語によるAND検索
- 大文字小文字を区別しない検索
- 拡張可能なアーキテクチャ

## 使用方法

```rust
use sagas::engine::SearchEngine;
use sagas::document::Document;
use sagas::tokenizer::SimpleTokenizer;
use sagas::index::InvertedIndex;

fn main() {
    let mut engine = SearchEngine::new(SimpleTokenizer, InvertedIndex::default());
    
    engine.add_document(Document::new(1, "Rust programming".to_string()));
    engine.add_document(Document::new(2, "Python programming".to_string()));
    
    let results = engine.search("rust"); // [1]
    println!("{:?}", results);
}
```

## 実行例

```bash
cargo run --example basic_usage
```

## テスト

```bash
cargo test
```