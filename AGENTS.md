# Rust学習プロジェクト

## Nix開発環境

以下のツールはNix dev shell内でのみ利用可能です。
ターミナルでは `direnv allow` 済みであれば自動的に有効になります。

AIエージェント（Claude Code等）から実行する場合は、必ず `nix develop --command` を前置してください。

```bash
nix develop --command <command>
```

## プロジェクト構造

Cargoワークスペース構成で管理する。

```
rust-learn/
├── Cargo.toml              # ワークスペースルート
├── crates/
│   ├── 01-hello/           # 各セクションごとのcrate
│   │   ├── Cargo.toml
│   │   ├── guide.md        # 学習ガイド（Rust by Exampleの内容）
│   │   └── src/
│   │       └── main.rs     # TODOコメント付き雛形（ユーザーが記述）
│   ├── 02-primitives/
│   ├── ...
│   ├── cli-minigrep/       # 成果物: ファイル検索ツール
│   ├── cli-todo/           # 成果物: タスク管理CLI
│   └── cli-logparse/       # 成果物: ログ解析ツール
```

## 構文学習

- Rust by Example（https://doc.rust-jp.rs/rust-by-example-ja/）のセクション順に全セクション学習する
- クレートの命名はセクション番号付き（`01-hello`, `02-primitives` 等）
- 各セクションに `guide.md` を配置し、Rust by Exampleの該当ページ内容を記載する
- `main.rs` はセクション内容に合わせたTODOコメント付きの雛形とし、ユーザーが自分で記述する

### guide.mdのフォーマット

```markdown
# <セクション名>

## 学習内容
- <トピック1>
- <トピック2>
- ...

## 参考ページ
<Rust by Exampleの該当URL>

## 内容
<Rust by Exampleの該当セクションの内容・コード例を記載>
```

### main.rsのフォーマット

```rust
fn main() {
    // TODO: <セクションのトピック1に対応する課題>
    // TODO: <セクションのトピック2に対応する課題>
    // TODO: <セクションのトピック3に対応する課題>
}
```

セクションの内容に合わせてTODOの数や構成を調整する。

## 成果物（CLIツール）

構文学習完了後、以下の3つのCLIツールを制作する。Rustの主要機能を網羅的に学ぶことを目的とする。

1. **cli-minigrep（ファイル検索ツール）** — 文字列処理、ファイルI/O、イテレータ、エラーハンドリング、正規表現
2. **cli-todo（タスク管理CLI）** — 構造体、Enum、シリアライズ（serde）、ファイル読み書き、CLI引数解析（clap）、テスト
3. **cli-logparse（ログ解析ツール）** — 正規表現、コレクション、トレイト、ジェネリクス、並行処理、非同期

## ビルド・実行方法

### 特定のセクションを実行

```bash
nix develop --command cargo run -p <crate名>
```

### 特定のセクションをビルド

```bash
nix develop --command cargo build -p <crate名>
```

### ワークスペース全体をビルド

```bash
nix develop --command cargo build --workspace
```

### テスト

```bash
nix develop --command cargo test -p <crate名>
```

### リント・フォーマット

```bash
nix develop --command cargo clippy -- -D warnings
nix develop --command cargo fmt
```

### Taskfile経由（CI用途）

```bash
nix develop --command task ci
```
