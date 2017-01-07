# Study Rust

公式

https://www.rust-lang.org/en-US/

## 環境構築

### インストール
```
$ curl https://sh.rustup.rs -sSf | sh
```

1) Proceed with installation (default) を選択

.zshrcに追記
```
source $HOME/.cargo/env
```

### Vim

rustfmtを入れる。

```
$ cargo install rustfmt
```

https://github.com/rust-lang/rust.vim をインストール

.vimrcに追記
```
let g:rustfmt_autosave = 1
```

## 用語の整理

* Cargo
  * ビルドシステム兼パッケージマネージャのこと
* Racer
  * オートコンプリートツールのこと
* rustfmt
  * コード整形（フォーマッタ）のこと

## メモ

### コンパイル

```
$ rustc main.rs
```

### cargoを使った環境

プロジェクトの作成
```
$ cargo new hello_world --bin
```

ビルド
```
$ cargo build
```

実行
```
$ cargo run
```
