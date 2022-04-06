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
