# クレート（Crates）

## 学習内容
- クレートの種類（バイナリクレートとライブラリクレート）
- ライブラリクレートの作成
- 外部クレートの利用
- `extern crate` 宣言

## 参考ページ
https://doc.rust-jp.rs/rust-by-example-ja/crates.html

## 内容

### クレートとは

クレート（crate）はRustのコンパイル単位です。2種類あります：

- **バイナリクレート**：実行可能ファイルを生成（`main` 関数が必要）
- **ライブラリクレート**：他のクレートから利用できるコードを提供（`main` 関数不要）

```bash
# バイナリクレートの作成
cargo new my_binary
# -> src/main.rs が作成される

# ライブラリクレートの作成
cargo new --lib my_library
# -> src/lib.rs が作成される
```

### ライブラリクレートの作成

```rust
// src/lib.rs（ライブラリクレートのルート）
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}
```

- `src/lib.rs` がライブラリクレートのルート
- `pub` をつけたアイテムが外部に公開される
- モジュールも `pub` で公開可能

### 外部クレートの利用

#### Cargo.toml に依存関係を追加

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"       # ランダム数生成
regex = "1"        # 正規表現
serde = { version = "1", features = ["derive"] }  # シリアライズ
```

#### コードでの利用

Rust 2018エディション以降では、`extern crate` 宣言なしで `use` だけで外部クレートを利用できます。

```rust
// Rust 2018以降：use だけで OK
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n: i32 = rng.gen_range(1..=100);
    println!("Random number: {}", n);
}
```

Rust 2015エディションでは `extern crate` が必要でした：

```rust
// Rust 2015：extern crate が必要
extern crate rand;

use rand::Rng;
```

### バイナリクレートからライブラリクレートを利用

プロジェクト内にライブラリとバイナリの両方を持つことができます：

```
my_project/
├── Cargo.toml
└── src/
    ├── lib.rs     （ライブラリクレート）
    └── main.rs    （バイナリクレート）
```

```rust
// src/lib.rs
pub fn helper() -> String {
    String::from("Hello from library!")
}
```

```rust
// src/main.rs
use my_project::helper; // 自クレートのライブラリを利用

fn main() {
    println!("{}", helper());
}
```

### 外部クレートのリネーム

Cargo.tomlでパッケージ名を変更して利用できます：

```toml
[dependencies]
my_rand = { package = "rand", version = "0.8" }
```

```rust
// コード内では my_rand として参照
use my_rand::Rng;
```

### バージョンの指定方法

```toml
[dependencies]
# セマンティックバージョニング
rand = "0.8"                      # >=0.8.0, <0.9.0
rand = "^0.8"                     # 上と同じ
rand = "~0.8.5"                   # >=0.8.5, <0.9.0
rand = ">=0.8.0, <0.9.0"         # 範囲指定

# 特定バージョン
rand = "=0.8.5"                   # ピン留め

# ワイルドカード
rand = "*"                        # 任意のバージョン（非推奨）

# Gitリポジトリから
# my_lib = { git = "https://github.com/user/repo" }

# ローカルパスから
# my_lib = { path = "../my_lib" }
```
