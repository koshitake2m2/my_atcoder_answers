# Rust

### 環境構築

#### 対象環境
- MacBook Air (M1, 2020)
- vscode

#### インストール

```bash
brew install rustup-init
rustup-init
exec $SHELL -l
cat <<-EOF >> $HOME/.cargo/config
[net]
git-fetch-with-cli = true
EOF

cargo install cargo-edit
cargo install cargo-watch
rustup component add rls rust-src rust-analysis
cargo install evcxr_repl
```

### Cargo.toml

ABCコンテストの問題数は参考によると以下らしい。 (参考: [A - New Generation ABC](https://atcoder.jp/contests/abc214/tasks/abc214_a))

> 1 回目から 125 回目までは 4 問
> 126 回目から 211 回目までは 6 問
> 212 回目から 214 回目までは 8 問

そのため、あらかじめCargo.tomlにbinを追加しておく。こうすることで問題ごとに実行可能なファイルを作成することができる。


```bash
❯ for n in {001..125}; do echo "# abc$n"; for a in {a..d}; do echo "[[bin]]
name = \"abc${n}_$a\"
path = \"src/abc$n/$a.rs\""; done; echo; done | pbcopy

❯ for n in {126..211}; do echo "# abc$n"; for a in {a..f}; do echo "[[bin]]
name = \"abc${n}_$a\"
path = \"src/abc$n/$a.rs\""; done; echo; done | pbcopy

❯ for n in {212..300}; do echo "# abc$n"; for a in {a..h}; do echo "[[bin]]
name = \"abc${n}_$a\"
path = \"src/abc$n/$a.rs\""; done; echo; done | pbcopy

```

## vscode

- rustのプラグインをインストールする

```bash
# 保存時コンパイル&実行
cargo watch -x run
# 保存時フォーマット
cargo watch -- cargo fmt
```

## IntelliJ IDEA

- rustのプラグインをインストールする
  - [Rust - IntelliJ IDEs Plugin | Marketplace](https://plugins.jetbrains.com/plugin/8182-rust)

- Settings / Preferences / Languages & Frameworks / Rust / Rustfmt
  - [x] Use rustfmt instead of built-in formatter
  - [x] Run rustfmt on Save

- コンテスト毎に行うこと
  1. Cargo.tomlにbinが定義されていなければ定義すること. 実行可能ファイルにするため. 
  2. ファイル作成ごとに `Refresh Cargo Projects` をすること. 実行可能にするため.
