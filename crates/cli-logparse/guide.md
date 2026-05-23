# cli-logparse - ログ解析ツール

## 概要

`logparse` は、ログファイルを解析・フィルタリングし、統計情報を出力するCLIツールです。大量のログファイルを効率的に処理するため、並行処理と非同期I/Oを活用します。

## コマンド仕様

```bash
# 基本的なログ解析
logparse <logfile>

# パターンでフィルタリング
logparse <logfile> --filter <pattern>

# 統計情報を表示
logparse <logfile> --stats

# フィルタリングと統計を組み合わせる
logparse <logfile> --filter "ERROR" --stats
```

### 引数・オプション

| 引数/オプション      | 説明                                                       |
| -------------------- | ---------------------------------------------------------- |
| `<logfile>`          | 解析対象のログファイルパス                                 |
| `--filter <pattern>` | 正規表現パターンでログ行をフィルタリングする               |
| `--stats`            | ログレベル別の集計、時間帯別の集計などの統計情報を表示する |

## 要件

1. **CLI引数解析**: `clap` クレートを使用して引数とオプションを定義する
2. **ログファイルのパース**:
   - 一般的なログフォーマット（例: `2024-01-15 10:30:45 [ERROR] Something went wrong`）をパースする
   - タイムスタンプ、ログレベル、メッセージを抽出する
3. **フィルタリング**: `regex` クレートを使用してパターンマッチングを行う
4. **統計情報の集計**:
   - ログレベル（DEBUG / INFO / WARN / ERROR）別の出現回数
   - 時間帯別の出現回数
   - エラーメッセージの頻出ランキング
5. **並行処理による高速化**:
   - `rayon` を使った並行での行処理
   - 大容量ファイルのストリーミング処理
6. **エラーハンドリング**:
   - ファイルが存在しない場合のエラー
   - 不正な正規表現パターンのエラー

## 学べるトピック

| トピック       | 詳細                                                                   |
| -------------- | ---------------------------------------------------------------------- |
| 正規表現       | `regex` クレート、`Regex::new`、キャプチャグループ、パターンマッチング |
| コレクション   | `HashMap`、`Vec`、`BTreeMap` を使ったデータ集計、ソート                |
| トレイト       | `FromStr` トレイトの実装、`Display` トレイト、トレイト境界             |
| ジェネリクス   | ジェネリックなパーサー、汎用的なフィルター関数                         |
| 並行処理       | `rayon` による並行イテレータ、`par_iter()`、`par_lines()`              |
| 非同期         | `tokio` による非同期ファイルI/O（発展課題）                            |
| Enum           | ログレベルの表現、パターンマッチング                                   |
| 標準ライブラリ | `std::collections`、`std::time`、`std::io::BufRead`                    |

## 使用するクレート

| クレート     | 用途                             | Cargo.toml の記述                                  |
| ------------ | -------------------------------- | -------------------------------------------------- |
| `clap`       | CLI引数解析（derive API）        | `clap = { version = "4", features = ["derive"] }`  |
| `regex`      | 正規表現によるフィルタリング     | `regex = "1"`                                      |
| `rayon`      | 並行処理（データ並列性）         | `rayon = "1"`                                      |
| `serde`      | ログデータのシリアライズ（発展） | `serde = { version = "1", features = ["derive"] }` |
| `serde_json` | JSON形式での出力（発展）         | `serde_json = "1"`                                 |
| `tokio`      | 非同期ランタイム（発展課題）     | `tokio = { version = "1", features = ["full"] }`   |
| `anyhow`     | エラーハンドリング               | `anyhow = "1"`                                     |
| `chrono`     | 日時のパースとフォーマット       | `chrono = "0.4"`                                   |

### Cargo.toml の依存関係例

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
regex = "1"
rayon = "1"
anyhow = "1"
chrono = "0.4"
```

### 発展課題用の追加依存関係

```toml
[dependencies]
# 上記に加えて:
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
```

## 実装ステップ

1. **Step 1**: `clap` でCLI引数のパーサーを定義する（`logfile` 引数、`--filter`、`--stats` オプション）
2. **Step 2**: ログ行のパーサーを実装する（`LogEntry` 構造体と `FromStr` トレイト）
3. **Step 3**: ログレベルを表す `LogLevel` Enum を定義する
4. **Step 4**: フィルタリング処理を実装する（`regex` を使ったパターンマッチ）
5. **Step 5**: 統計情報の集計を実装する（`HashMap` でログレベル別にカウント）
6. **Step 6**: `rayon` を使って並行処理で高速化する
7. **Step 7**: 結果を見やすくフォーマットして出力する
8. **Step 8**: エラーハンドリングを整える（`anyhow`）
9. **Step 9**: テストを書く
10. **Step 10（発展）**: `tokio` で非同期ファイル読み込みを実装する

## データ構造のイメージ

```rust
#[derive(Debug)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug)]
struct LogEntry {
    timestamp: chrono::NaiveDateTime,
    level: LogLevel,
    message: String,
}
```

## ログフォーマットの例

```
2024-01-15 10:30:45 [ERROR] Database connection failed
2024-01-15 10:30:46 [INFO] Retrying connection...
2024-01-15 10:30:47 [WARN] High memory usage detected
2024-01-15 10:30:48 [INFO] Connection restored
2024-01-15 10:30:50 [DEBUG] Cache hit for key=user:123
2024-01-15 10:31:00 [ERROR] Timeout on API request to /api/users
```

## 出力のイメージ

### フィルタリング結果

```
[ERROR] 2024-01-15 10:30:45 Database connection failed
[ERROR] 2024-01-15 10:31:00 Timeout on API request to /api/users
```

### 統計情報

```
=== Log Statistics ===

Total lines: 1250

By Level:
  DEBUG:    450 (36.0%)
  INFO:     520 (41.6%)
  WARN:     180 (14.4%)
  ERROR:    100 (8.0%)

Top Errors:
  1. "Database connection failed" (12 occurrences)
  2. "Timeout on API request" (8 occurrences)
  3. "Permission denied" (5 occurrences)
```

## 発展課題

- 複数ログファイルの一括解析（ワイルドカード対応）
- JSON形式のログ（構造化ログ）のパース対応
- `tokio` による非同期でのファイル読み込み
- リアルタイムでのログ監視（`tail -f` のような動作）
- 解析結果をJSON/CSVで出力するオプション
- Webサーバーのアクセスログ（Apache/Nginx形式）に対応
- ログの異常検知（エラー率の急増などを自動検出）
- `indicatif` クレートでプログレスバーを表示
