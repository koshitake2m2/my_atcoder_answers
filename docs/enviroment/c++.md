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
