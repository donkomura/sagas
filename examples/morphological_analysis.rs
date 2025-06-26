use sagas::engine::SearchEngine;
use sagas::document::Document;
use sagas::tokenizer::{MorphologicalTokenizer, FallbackTokenizer};
use sagas::index::InvertedIndex;

fn run_with_morphological_tokenizer() {
    println!("形態素解析トークナイザーを使用します");
    let mut engine = SearchEngine::new(
        MorphologicalTokenizer::new("ipadic-mecab-2_7_0/system.dic.zst").unwrap(),
        InvertedIndex::default()
    );
    
    run_search_example(&mut engine);
}

fn run_with_fallback_tokenizer() {
    println!("辞書ファイルが見つからないため、フォールバックトークナイザーを使用します");
    println!("形態素解析を試すには、vibratoの辞書ファイルをダウンロードしてください:");
    println!("wget https://github.com/daac-tools/vibrato/releases/download/v0.5.2/ipadic-mecab-2_7_0.tar.xz");
    println!("tar xf ipadic-mecab-2_7_0.tar.xz");
    
    let mut engine = SearchEngine::new(FallbackTokenizer::new(), InvertedIndex::default());
    run_search_example(&mut engine);
}

fn run_search_example<T: sagas::tokenizer::Tokenizer>(engine: &mut SearchEngine<T, InvertedIndex>) {
    let documents = vec![
        Document::new(1, "本とカレーの街神保町へようこそ".to_string()),
        Document::new(2, "東京大学でコンピュータサイエンスを学ぶ".to_string()),
        Document::new(3, "自然言語処理の研究をしています".to_string()),
        Document::new(4, "Rustで高速な検索エンジンを実装".to_string()),
        Document::new(5, "機械学習とディープラーニングの違い".to_string()),
        Document::new(6, "プログラミング言語の比較分析".to_string()),
        Document::new(7, "データベース設計のベストプラクティス".to_string()),
        Document::new(8, "Webアプリケーションのセキュリティ".to_string()),
        Document::new(9, "クラウドコンピューティングの活用方法".to_string()),
        Document::new(10, "DevOpsとCI/CDパイプラインの構築".to_string()),
    ];
    
    for doc in documents {
        engine.add_document(doc);
    }
    
    println!("検索エンジンにドキュメントを追加しました");
    println!();
    
    let queries = vec![
        "プログラミング",
        "言語",
        "研究",
        "大学",
        "東京",
        "自然言語",
        "機械学習",
        "データベース",
        "Web",
        "クラウド",
        "DevOps",
        "Rust",
        "コンピュータ",
        "サイエンス",
        "セキュリティ",
    ];
    
    for query in queries {
        let results = engine.search(query);
        if results.is_empty() {
            println!("検索語 '{}': 結果なし", query);
        } else {
            println!("検索語 '{}': ドキュメントID {:?}", query, results);
        }
    }
    
    println!();
    println!("形態素解析の効果:");
    println!("- 日本語の文章が適切に単語に分割される");
    println!("- 助詞や接続詞が除外され、重要な名詞や動詞が抽出される");
    println!("- より正確な検索結果が得られる");
}

fn main() {
    // 形態素解析トークナイザーを試す（辞書ファイルが利用できない場合はフォールバック）
    match MorphologicalTokenizer::new("ipadic-mecab-2_7_0/system.dic.zst") {
        Ok(_) => run_with_morphological_tokenizer(),
        Err(_) => run_with_fallback_tokenizer(),
    }
} 