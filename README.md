# my atcoder answers
AtCoderの回答

# C++

## bits/stdc++.h の設定

#### 環境
- m1 mac
- vscode

#### gccのインストール & bits/stdc++.hの配置

```bash
$ brew install gcc

# シンボリックリンクの設定
$ ln -s /opt/homebrew/bin/g++-11 /opt/homebrew/bin/g++

# stdc++.hを探す
$ find /opt/homebrew -name "stdc++.h"
/opt/homebrew/Cellar/gcc/11.2.0/include/c++/11.1.0/aarch64-apple-darwin20/bits/stdc++.h

# /opt/homebrew/include 配下に bits/stdc++.h を追加する
$ mkdir /opt/homebrew/include/bits
$ cp /opt/homebrew/Cellar/gcc/11.2.0/include/c++/11.1.0/aarch64-apple-darwin20/bits/stdc++.h /opt/homebrew/include/bits
```

#### vscode

brew でインストールした gcc を vscode で使えるようにするための設定.

1. c_cpp_properties.json の includePath に "/opt/homebrew/include/**" を追加する.
2. c_cpp_properties.json の compilerPath に "/opt/homebrew/bin/g++" を追加する.


## 実行

```
../bin/mygpp.bash a.cpp
```

# Scala

```
brew install coursier/formulas/coursier && cs setup
```

## 実行

```
../bin/myscalac.bash a.scala
```

