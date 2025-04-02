# This lab is used for RUST BERT
 ## 1. Some definition

 Candle - Candle is an open source machine learning library for Rust that provides access to state-of-the-art models like T5 and BERT.

 Rust-Bert - Rust-Bert is a Rust library for natural language processing based on the Hugging Face Transformers library. It provides easy access to pre-trained NLP models.

## 2. Step

2.1 Create cargo new ....

2.2 run

```bash
cargo run -- "My name is An, Msc in AI "
```

Result:

[Entity { word: "An", score: 0.9708765745162964, label: "PER", offset: Offset { begin: 11, end: 13 } }, Entity { word: "AI", score: 0.8534272909164429, label: "ORG", offset: Offset { begin: 22, end: 24 } }]