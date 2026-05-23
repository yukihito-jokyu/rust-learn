# アトリビュート（Attributes）

## 学習内容

- `dead_code` アトリビュート
- `derive` アトリビュート
- マクロの使用
- 条件コンパイル（`cfg`）

## 参考ページ

https://doc.rust-jp.rs/rust-by-example-ja/attribute.html

## 内容

### アトリビュートとは

アトリビュートはコンパイラに対するメタデータで、アイテム（関数、構造体、モジュールなど）に追加情報を与えます。2つの構文があります：

```rust
// 全体アトリビュート（1行）
#[attribute]

// 複数行アトリビュート
#[attribute(arg1, arg2)]

// 外部アトリビュート（アイテムの上に書く）
#[attribute]
fn function() {}

// 内部アトリビュート（アイテムの中に書く）
fn function() {
    #![attribute]
}
```

- `#[...]`：外部アトリビュート（次のアイテムに適用）
- `#![...]`：内部アトリビュート（囲んでいるアイテムに適用）

### `dead_code` アトリビュート

未使用コードの警告を抑制します。

```rust
// この関数は使われていないが、警告を出さない
#[allow(dead_code)]
fn unused_function() {
    println!("This is unused.");
}

// モジュール全体に適用
#[allow(dead_code)]
mod my_module {
    pub fn func1() {}
    pub fn func2() {}
    fn func3() {} // これも警告なし
}

// 個別のフィールドにも適用可能
struct Person {
    name: String,
    #[allow(dead_code)]
    age: u32, // 使われていなくても警告なし
}

fn main() {
    // unused_function() を呼ばないが警告は出ない
}
```

- `#[allow(dead_code)]`：dead_code警告を抑制
- `#[warn(dead_code)]`：警告レベルに設定（デフォルト）
- `#[deny(dead_code)]`：コンパイルエラーにする
- `#[forbid(dead_code)]`：絶対に許可しない（`allow`で上書き不可）

### `derive` アトリビュート

一般的なトレイトの実装を自動生成します。

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();

    // Debug トレイトの自動実装により {:?} で表示可能
    println!("{:?}", p1); // Point { x: 1, y: 2 }

    // PartialEq トレイトの自動実装
    println!("equal: {}", p1 == p2); // equal: true

    // Clone トレイトの自動実装
    let p3 = p1.clone();
    println!("cloned: {:?}", p3);
}
```

#### よく使う derive 可能なトレイト

| トレイト     | 説明                                     |
| ------------ | ---------------------------------------- |
| `Debug`      | `{:?}` でデバッグ出力可能にする          |
| `Clone`      | 明示的な複製（`.clone()`）を可能にする   |
| `Copy`       | 代入時に自動複製される（暗黙のクローン） |
| `PartialEq`  | `==`, `!=` 演算子を可能にする            |
| `Eq`         | 完全な等価性比較                         |
| `PartialOrd` | `<`, `>`, `<=`, `>=` を可能にする        |
| `Ord`        | 完全な順序付け                           |
| `Hash`       | ハッシュ値の計算を可能にする             |
| `Default`    | デフォルト値を生成する                   |

```rust
#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Config {
    name: String,
    value: i32,
    enabled: bool,
}

fn main() {
    // Default トレイト
    let config = Config::default();
    println!("{:?}", config); // Config { name: "", value: 0, enabled: false }

    // Default を使った更新
    let custom = Config {
        name: String::from("test"),
        ..Config::default()
    };
}
```

### マクロの使用

マクロは `!` がついているのが特徴です。

```rust
fn main() {
    // println! マクロ
    println!("Hello");
    println!("formatted: {}", 42);
    println!("debug: {:?}", vec![1, 2, 3]);

    // vec! マクロ
    let v = vec![1, 2, 3, 4, 5];
    println!("vec: {:?}", v);

    // panic! マクロ（プログラムを異常終了）
    // panic!("something went wrong!");

    // assert! マクロ
    assert!(2 + 2 == 4);
    assert_eq!(2 + 2, 4);
    assert_ne!(2 + 2, 5);

    // todo! マクロ（未実装の印）
    // fn not_done() {
    //     todo!("implement later");
    // }

    // unimplemented! マクロ
    // fn not_done() {
    //     unimplemented!();
    // }

    // println! の高度なフォーマット
    println!("{:05}", 42);       // 00042（0埋め、幅5）
    println!("{:>10}", "right"); //      right（右寄せ、幅10）
    println!("{:<10}", "left");  // left      （左寄せ、幅10）
    println!("{:^10}", "center");//   center  （中央寄せ、幅10）
}
```

### 条件コンパイル（`cfg`）

`cfg` アトリビュートを使うと、条件に応じてコードをコンパイルに含めたり除外したりできます。

#### 基本的な使い方

```rust
// OSごとのコード
#[cfg(target_os = "linux")]
fn platform_specific() {
    println!("Running on Linux");
}

#[cfg(target_os = "macos")]
fn platform_specific() {
    println!("Running on macOS");
}

#[cfg(target_os = "windows")]
fn platform_specific() {
    println!("Running on Windows");
}

// アーキテクチャごと
#[cfg(target_arch = "x86_64")]
fn arch_info() {
    println!("64-bit x86");
}

#[cfg(target_arch = "aarch64")]
fn arch_info() {
    println!("ARM64");
}
```

#### 論理演算子

```rust
// all() = AND条件
#[cfg(all(unix, target_pointer_width = "64"))]
fn unix_64bit() {
    println!("64-bit Unix");
}

// any() = OR条件
#[cfg(any(windows, target_os = "macos"))]
fn windows_or_mac() {
    println!("Windows or macOS");
}

// not() = 否定
#[cfg(not(test))]
fn not_in_test() {
    println!("Not running tests");
}
```

#### `cfg!` マクロ

コンパイル時に条件を bool 値として評価します。

```rust
fn main() {
    if cfg!(target_os = "linux") {
        println!("Linux detected at runtime!");
    } else if cfg!(target_os = "macos") {
        println!("macOS detected at runtime!");
    }

    println!("Arch: {}", cfg!(target_arch = "x86_64"));
}
```

#### `cfg_attr` アトリビュート

条件が真のときに別のアトリビュートを適用します。

```rust
// Linuxのときだけ Debug を derive
#[cfg_attr(target_os = "linux", derive(Debug))]
struct PlatformConfig {
    name: String,
}
```

#### よく使う `cfg` 述語

| 述語                   | 例                                | 説明                      |
| ---------------------- | --------------------------------- | ------------------------- |
| `target_os`            | `"linux"`, `"macos"`, `"windows"` | オペレーティングシステム  |
| `target_arch`          | `"x86_64"`, `"aarch64"`           | CPUアーキテクチャ         |
| `target_pointer_width` | `"32"`, `"64"`                    | ポインタ幅                |
| `feature`              | `"my_feature"`                    | Cargoのフィーチャーフラグ |
| `debug_assertions`     | -                                 | デバッグビルドかどうか    |
| `test`                 | -                                 | テスト中かどうか          |
