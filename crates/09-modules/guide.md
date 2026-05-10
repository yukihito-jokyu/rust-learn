# モジュール（Modules）

## 学習内容
- 可視性（`pub`）
- 構造体の可視性
- `use` 宣言
- `super` と `self`
- モジュールの分割（ファイルへの分離）

## 参考ページ
https://doc.rust-jp.rs/rust-by-example-ja/mod.html

## 内容

### モジュールの基本

Rustのモジュールシステムは、コードを論理的な単位に分割するための仕組みです。`mod` キーワードでモジュールを定義します。

```rust
mod english {
    pub fn greet() {
        println!("Hello!");
    }

    fn secret() {
        println!("This is private.");
    }
}

fn main() {
    // モジュールの公開関数にアクセス
    english::greet(); // OK
    // english::secret(); // エラー：非公開関数にはアクセスできない
}
```

- `mod name { ... }` でモジュールを定義
- デフォルトではすべて **非公開**（private）
- `pub` キーワードで **公開**（public）にする

### 可視性（Visibility）

Rustの可視性は以下のレベルがあります：

```rust
mod outer {
    pub mod inner {
        pub fn public_function() {
            println!("公開関数");
        }

        pub(crate) fn crate_visible() {
            println!("クレート内公開");
        }

        fn private_function() {
            println!("非公開関数");
        }
    }

    pub fn try_access() {
        inner::public_function();  // OK: 公開
        inner::crate_visible();    // OK: 同じクレート内
        inner::private_function(); // OK: 親モジュールからはアクセス可能（Rust 2024以降）
    }
}

fn main() {
    outer::inner::public_function(); // OK
    outer::inner::crate_visible();   // OK: 同じクレート内
    // outer::inner::private_function(); // エラー：非公開
}
```

- `pub`：どこからでもアクセス可能
- `pub(crate)`：同じクレート内からアクセス可能
- `pub(super)`：親モジュールからアクセス可能
- `pub(in path)`：指定したパス内からアクセス可能
- デフォルト（なし）：現在のモジュールと子孫のみ

### 構造体の可視性

構造体のフィールドも可視性を制御できます。構造体自体を `pub` にしても、フィールドは別途 `pub` にする必要があります。

```rust
mod shapes {
    pub struct Point {
        pub x: i32,
        pub y: i32,
        _z: i32, // 非公開フィールド（先頭に _ をつけて未使用警告を抑制）
    }

    impl Point {
        // 公開コンストラクタ（非公開フィールドも初期化できる）
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y, _z: 0 }
        }
    }
}

fn main() {
    let p = shapes::Point::new(1, 2);
    println!("x: {}, y: {}", p.x, p.y); // OK: 公開フィールド
    // p._z; // エラー：非公開フィールド
}
```

- 構造体を `pub` にしてもフィールドは非公開のまま
- フィールドごとに個別に `pub` を指定する必要がある
- 非公開フィールドを持つ構造体は、コンストラクタメソッドで初期化する

### `use` 宣言

`use` を使うと、モジュールのパスを短く記述できます。

```rust
mod geometry {
    pub mod circle {
        pub fn area(radius: f64) -> f64 {
            std::f64::consts::PI * radius * radius
        }

        pub fn circumference(radius: f64) -> f64 {
            2.0 * std::f64::consts::PI * radius
        }
    }
}

// use でパスを短縮
use geometry::circle::area;
use geometry::circle::circumference;

// まとめてインポート
use geometry::circle::{area as circle_area, circumference as circle_circum};

// すべてインポート（非推奨だが可能）
// use geometry::circle::*;

fn main() {
    println!("area: {}", area(5.0));
    println!("circumference: {}", circumference(5.0));
    println!("area (alias): {}", circle_area(3.0));
}
```

- `use path::to::item` でインポート
- `as` でエイリアスを設定
- `{}` で複数アイテムをまとめてインポート
- `*` で全アイテムをインポート（推奨されない）

### `super` と `self`

`super` は親モジュール、`self` は現在のモジュールを指します。

```rust
mod network {
    pub fn connect() {
        println!("Connecting...");
    }

    pub mod server {
        pub fn start() {
            // super で親モジュールの関数を呼ぶ
            super::connect();
            println!("Server started");
        }

        pub fn status() {
            // self で同じモジュール内の関数を呼ぶ
            self::start();
        }
    }
}

fn main() {
    network::server::start();
    // 出力:
    // Connecting...
    // Server started
}
```

- `super`：親モジュールを参照（ファイルシステムの `..` に似ている）
- `self`：現在のモジュールを参照（ファイルシステムの `.` に似ている）

### モジュールの分割（ファイルへの分離）

大きくなったモジュールは別のファイルに分割できます。

#### インラインからファイルへ

インラインで定義したモジュール：
```rust
// main.rs
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}
```

これをファイルに分割：
```
src/
├── main.rs
└── math.rs       （または math/mod.rs）
```

```rust
// src/math.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```rust
// src/main.rs
mod math; // ファイルからモジュールを読み込む

fn main() {
    println!("{}", math::add(1, 2));
}
```

#### サブモジュールの分割

```
src/
├── main.rs
└── network/
    ├── mod.rs     （または network.rs）
    └── server.rs
```

```rust
// src/network/mod.rs
pub mod server; // サブモジュールを宣言

pub fn connect() {
    println!("Connecting...");
    server::start();
}
```

```rust
// src/network/server.rs
pub fn start() {
    println!("Server started");
}
```

- `mod name;` で外部ファイルからモジュールを読み込む
- `name.rs` または `name/mod.rs` のどちらでも可能
- サブモジュールはディレクトリで管理
