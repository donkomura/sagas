# Sagas - シンプルな全文検索エンジン

Rustで実装されたシンプルで拡張可能な全文検索エンジンです。

## 機能

- ドキュメントの追加とインデックス化
- 複数語によるAND検索
- 大文字小文字を区別しない検索
- 日本語形態素解析対応（vibrato使用）

## 使用方法

### Docker（推奨）

```bash
# ビルド
docker build -t sagas .

# 実行
docker run --rm sagas ./morphological_analysis  # 形態素解析の例
docker run --rm sagas ./basic_usage             # 基本的な例
```

### ローカル

```bash
# 辞書ファイルのダウンロード
wget https://github.com/daac-tools/vibrato/releases/download/v0.5.0/ipadic-mecab-2_7_0.tar.xz
tar xf ipadic-mecab-2_7_0.tar.xz

# 実行
cargo run --example basic_usage
cargo run --example morphological_analysis
```

## テスト

```bash
cargo test
```