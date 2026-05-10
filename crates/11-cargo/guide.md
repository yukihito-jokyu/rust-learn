# Cargo

## 学習内容
- Cargo.tomlの設定
- 依存関係の管理
- テストの書き方
- ビルドプロファイル

## 参考ページ
https://doc.rust-jp.rs/rust-by-example-ja/cargo.html

## 内容

### Cargoとは

CargoはRustのビルドシステム兼パッケージマネージャです。以下のことができます：

- プロジェクトの作成（`cargo new`）
- ビルド（`cargo build`）
- 実行（`cargo run`）
- テスト（`cargo test`）
- ドキュメント生成（`cargo doc`）
- 依存関係の管理

### Cargo.tomlの設定

`Cargo.toml` はプロジェクトの設定ファイルです。TOML形式で記述します。

```toml
[package]
name = "my_project"           # プロジェクト名
version = "0.1.0"             # バージョン（セマンティックバージョニング）
edition = "2021"              # Rustエディション（2015, 2018, 2021, 2024）
authors = ["Your Name <you@example.com>"]
description = "A sample project"
license = "MIT"
repository = "https://github.com/user/repo"

[dependencies]
# 本番依存関係
serde = { version = "1", features = ["derive"] }
rand = "0.8"

[dev-dependencies]
# 開発時のみの依存関係（テスト、ベンチマーク用）
tempfile = "3.0"

[build-dependencies]
# ビルドスクリプト用の依存関係
# cc = "1.0"
```

### 依存関係の管理

#### バージョン指定

```toml
[dependencies]
# セマンティックバージョニング
rand = "0.8"                  # >=0.8.0, <0.9.0
rand = "^0.8"                 # 上と同じ（キャレットはデフォルト）
rand = "~0.8.5"               # >=0.8.5, <0.9.0（チルダ）
rand = ">=0.8.0, <0.9.0"     # 範囲指定
rand = "=0.8.5"               # 厳密なバージョン指定
rand = "*"                    # 任意のバージョン（非推奨）
```

#### 様々なソースからの依存関係

```toml
[dependencies]
# crates.ioから
rand = "0.8"

# Gitリポジトリから
# my_lib = { git = "https://github.com/user/repo" }
# my_lib = { git = "https://github.com/user/repo", branch = "main" }
# my_lib = { git = "https://github.com/user/repo", tag = "v1.0.0" }
# my_lib = { git = "https://github.com/user/repo", rev = "abc123" }

# ローカルパスから
# my_lib = { path = "../my_lib" }

# フィーチャーの有効化
# serde = { version = "1", features = ["derive"] }

# リネーム
# my_rand = { package = "rand", version = "0.8" }
```

#### Cargo.lock

- `Cargo.lock` は依存関係の正確なバージョンを記録するファイル
- バイナリプロジェクトでは **コミットすべき**
- ライブラリプロジェクトでは **コミットしない**（利用者がバージョンを解決する）

### テスト

#### 単体テスト

ソースファイル内に `#[cfg(test)]` モジュールとして記述します。

```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_add_not_equal() {
        assert_ne!(add(2, 3), 6);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Some(5));
        assert_eq!(divide(10, 0), None);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn test_overflow() {
        panic!("overflow");
    }

    #[test]
    #[ignore] // cargo test -- --ignored で実行
    fn expensive_test() {
        // 時間のかかるテスト
    }
}
```

#### 結合テスト

`tests/` ディレクトリに配置します。

```
my_project/
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```

```rust
// tests/integration_test.rs
use my_project::add;

#[test]
    fn it_works() {
    assert_eq!(add(1, 1), 2);
}
```

#### ドキュメンテーションテスト

ドキュメンテーションコメント内のコード例もテストされます。

```rust
/// 2つの数値を足し算します。
///
/// # Examples
///
/// ```
/// use my_project::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### ビルドプロファイル

```toml
[profile.dev]
opt-level = 0          # 最適化なし（コンパイル速い）
debug = true           # デバッグ情報あり
lto = false            # リンク時最適化なし
codegen-units = 256    # 並列コンパイル単位（多いほど速い）

[profile.release]
opt-level = 3          # 最大最適化
debug = false          # デバッグ情報なし
lto = true             # リンク時最適化あり
codegen-units = 1      # 最適化重視
strip = true           # デバッグシンボルを削除

# カスタムプロファイル
[profile.release-lto]
inherits = "release"
lto = "fat"
opt-level = 3
```

#### プロファイルオプション

| オプション | 説明 |
|---|---|
| `opt-level` | `0`（なし）~ `3`（最大）、`"s"`/`"z"`（サイズ最適化） |
| `debug` | `true`/`false`、または `0`~`2` のレベル |
| `lto` | `true`/`false`/`"thin"`/`"fat"` |
| `codegen-units` | 1以上の整数（少ないほど最適化される） |
| `strip` | `true`/`false` |
| `panic` | `"unwind"`/`"abort"` |

### Cargoコマンド一覧

| コマンド | 説明 |
|---|---|
| `cargo new name` | 新しいプロジェクトを作成 |
| `cargo init` | 既存のディレクトリでプロジェクト初期化 |
| `cargo build` | コンパイル（デバッグビルド） |
| `cargo build --release` | リリースビルド |
| `cargo run` | ビルドして実行 |
| `cargo test` | テストを実行 |
| `cargo check` | コンパイルチェック（バイナリ生成なし、高速） |
| `cargo doc` | ドキュメントを生成 |
| `cargo doc --open` | ドキュメントを生成してブラウザで開く |
| `cargo update` | 依存関係を更新（Cargo.lockを更新） |
| `cargo clean` | ビルド成果物を削除 |
| `cargo clippy` | Lintチェック |
| `cargo fmt` | コードフォーマット |
