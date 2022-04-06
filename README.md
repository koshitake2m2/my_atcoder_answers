# my atcoder answers
AtCoderの回答, 及び, 競プロで便利なライブラリ集.

AtCoderのプロフィール: [https://atcoder.jp/users/takeshi33333](https://atcoder.jp/users/takeshi33333)

回答で使用している言語は以下の通り.
- C++
- Python3
- Scala
- Rust

# C++ (GCC)

## vscode & コマンドライン

### 環境構築

#### 対象環境
- MacBook Air (M1, 2020)
- vscode

#### gccのインストール & bits/stdc++.hの配置

```bash
brew install gcc

# シンボリックリンクの設定
ln -s /opt/homebrew/bin/g++-11 /opt/homebrew/bin/g++

# stdc++.hを探す
find /opt/homebrew -name "stdc++.h"
## 出力例: /opt/homebrew/Cellar/gcc/11.2.0/include/c++/11.1.0/aarch64-apple-darwin20/bits/stdc++.h

# /opt/homebrew/include 配下に bits/stdc++.h を追加する
mkdir /opt/homebrew/include/bits
cp /opt/homebrew/Cellar/gcc/11.2.0/include/c++/11.1.0/aarch64-apple-darwin20/bits/stdc++.h /opt/homebrew/include/bits
```

その他参考
- [AP1 - 付録1.コードテストの使い方](https://atcoder.jp/contests/APG4b/tasks/APG4b_ak)
  - 「手元のコンピュータでプログラムを書きたい場合」

#### vscodeの設定

brew でインストールした gcc を vscode で使えるようにするための設定.

1. c_cpp_properties.json の includePath に `/opt/homebrew/include/**` を追加する.
2. c_cpp_properties.json の compilerPath に `/opt/homebrew/bin/g++` を追加する.


### 実行

```bash
../../bin/myg++ a.cpp
```

# Python3

## vscode

### 環境構築

#### 対象環境
- MacBook Air (M1, 2020)
- vscode

#### pythonインストール
- anyenvでよしなに

#### pip
- flake8

#### vscodeのプラグイン
- Python


# Scala

## コマンドライン環境

### 環境構築

#### 対象環境
- MacBook Air (M1, 2020)
- vscode

#### scala関連インストール

```bash
brew install coursier/formulas/coursier && cs setup
```

### 実行

```bash
../bin/myscalac a.scala
```

## IntelliJ IDEA

### 環境構築
build.sbtをよしなにロードする

### 実行
`object Main extends App` を実行する

### 提出
🚨 注意: AtCoder提出時には必ず `package ...` を削除すること. CEの原因になるため.


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
