# cli-todo - タスク管理CLI

## 概要

`todo` は、コマンドラインからタスクを管理するシンプルなTodoアプリです。タスクの追加、一覧表示、完了、削除の基本操作に加え、JSON形式での永続化を行います。

## コマンド仕様

```bash
# タスクの追加
todo add "タスクの内容"

# タスクの一覧表示
todo list

# タスクを完了にする
todo done <id>

# タスクの削除
todo delete <id>
```

### サブコマンド

| サブコマンド | 引数 | 説明 |
|-------------|------|------|
| `add` | `<task>` | 新しいタスクを追加する |
| `list` | なし | 全タスクを一覧表示する |
| `done` | `<id>` | 指定したIDのタスクを完了にする |
| `delete` | `<id>` | 指定したIDのタスクを削除する |

## 要件

1. **CLI引数解析**: `clap` クレートを使用してサブコマンドを持つCLIを構築する
2. **データ構造**: タスクを表す構造体（`Task`）を定義する
   - `id`: タスクの一意な識別子（整数）
   - `title`: タスクの内容（文字列）
   - `done`: 完了フラグ（真偽値）
3. **JSON永続化**: `serde` と `serde_json` を使用してタスクをJSONファイルに保存・読み込みする
   - 保存先: `~/.todo.json` またはカレントディレクトリの `todo.json`
4. **一覧表示のフォーマット**: ID、完了状態、タイトルを見やすく表示する
5. **エラーハンドリング**:
   - 存在しないIDが指定された場合のエラー
   - ファイルの読み書きエラー
   - 不正な引数のエラー

## 学べるトピック

| トピック | 詳細 |
|----------|------|
| 構造体（Struct） | `struct Task` の定義、`Debug`、`Display` トレイトの実装 |
| Enum | サブコマンドの表現、`Option<T>` と `Result<T, E>` の活用 |
| シリアライズ（serde） | `#[derive(Serialize, Deserialize)]`、JSONの読み書き |
| ファイル読み書き | `std::fs` モジュール、`read_to_string`、`write` |
| CLI引数解析（clap） | `clap::Parser`、サブコマンド、引数の定義とバリデーション |
| テスト | ユニットテスト、統合テスト、`#[cfg(test)]` |
| コレクション | `Vec<Task>` の操作、イテレータ、フィルタリング |
| エラーハンドリング | カスタムエラー型、`thiserror` クレート（発展） |

## 使用するクレート

| クレート | 用途 | Cargo.toml の記述 |
|----------|------|-------------------|
| `clap` | CLI引数解析（derive API） | `clap = { version = "4", features = ["derive"] }` |
| `serde` | シリアライズ/デシリアライズのトレイト | `serde = { version = "1", features = ["derive"] }` |
| `serde_json` | JSONの読み書き | `serde_json = "1"` |
| `anyhow` | エラーハンドリング（発展） | `anyhow = "1"` |

### Cargo.toml の依存関係例

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## 実装ステップ

1. **Step 1**: `clap` でCLI引数のパーサーを定義する（サブコマンド: add, list, done, delete）
2. **Step 2**: `Task` 構造体を定義し、`Serialize` / `Deserialize` を導出する
3. **Step 3**: JSONファイルからのタスク読み込み関数を実装する
4. **Step 4**: JSONファイルへのタスク保存関数を実装する
5. **Step 5**: `add` コマンドを実装する（タスクの追加と保存）
6. **Step 6**: `list` コマンドを実装する（タスクの一覧表示）
7. **Step 7**: `done` コマンドを実装する（タスクの完了フラグ更新）
8. **Step 8**: `delete` コマンドを実装する（タスクの削除）
9. **Step 9**: エラーハンドリングを整える
10. **Step 10**: テストを書く

## データ構造のイメージ

```rust
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u64,
    title: String,
    done: bool,
}
```

## 保存されるJSONファイルのイメージ

```json
[
  { "id": 1, "title": "Rustの学習", "done": false },
  { "id": 2, "title": "Todoアプリを作る", "done": true },
  { "id": 3, "title": "テストを書く", "done": false }
]
```

## 発展課題

- タスクに優先度（高/中/低）を追加する
- タスクに作成日時を追加する
- `list` にフィルタリングオプションを追加する（完了/未完了一覧）
- タスクの編集（`edit` コマンド）を追加する
- 複数のリスト（プロジェクト）をサポートする
- `colored` クレートで出力に色を付ける
- `dirs` クレートで設定ディレクトリ（`~/.config/todo/`）に保存する
