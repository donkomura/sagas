FROM rust:1.87-slim as builder

RUN apt-get update && apt-get install -y \
    wget \
    xz-utils \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 辞書ファイルをダウンロードして展開
RUN wget https://github.com/daac-tools/vibrato/releases/download/v0.5.0/ipadic-mecab-2_7_0.tar.xz \
    && tar xf ipadic-mecab-2_7_0.tar.xz \
    && rm ipadic-mecab-2_7_0.tar.xz

# Cargo.tomlのみをコピー
COPY Cargo.toml ./

# 依存関係をビルド（キャッシュを活用）
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

COPY src ./src
COPY examples ./examples

RUN cargo build --release --example morphological_analysis
RUN cargo build --release --example basic_usage

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libgcc-s1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/ipadic-mecab-2_7_0 ./ipadic-mecab-2_7_0

COPY --from=builder /app/target/release/examples/morphological_analysis ./
COPY --from=builder /app/target/release/examples/basic_usage ./

CMD ["./morphological_analysis"] 