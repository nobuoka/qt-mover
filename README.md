qt-mover
==========

## これは何？

Qiita:Team のエクスポートデータの引っ越しプログラム。
Rust で書かれている。

## 使い方

### 事前準備

* Qiita:Team からのエクスポートデータを data ディレクトリに配置
  (data/articles と data/groups と data/projects の 3 ディレクトリが存在する状態)
* Rust 環境

### 実行

```
cargo run
```

結果は output ディレクトリに出力される。
