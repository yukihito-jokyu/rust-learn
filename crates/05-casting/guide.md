# 型キャスト（Casting）

## 学習内容
- 型キャスト（`as`キーワード）
- リテラルと型注釈
- 型推論（Type Inference）
- 型エイリアス（`type`キーワード）

## 参考ページ
https://doc.rust-jp.rs/rust-by-example-ja/types/cast.html

## 内容

### 型キャスト（`as`）

Rustでは `as` キーワードを使ってプリミティブ型間のキャストを行います。

```rust
fn main() {
    // 整数型間のキャスト
    let a: i32 = 42;
    let b: u8 = a as u8;
    println!("i32 -> u8: {} -> {}", a, b); // 42 -> 42

    // 丸めに注意: 大きな値のキャスト
    let c: i32 = 300;
    let d: u8 = c as u8;
    println!("i32 -> u8: {} -> {}", c, d); // 300 -> 44（オーバーフロー）

    // 符号付き -> 符号なし
    let e: i8 = -1;
    let f: u8 = e as u8;
    println!("i8 -> u8: {} -> {}", e, f); // -1 -> 255

    // 浮動小数点数 -> 整数（小数部分は切り捨て）
    let g: f64 = 3.99;
    let h: i32 = g as i32;
    println!("f64 -> i32: {} -> {}", g, h); // 3.99 -> 3

    // 整数 -> 浮動小数点数
    let i: i32 = 42;
    let j: f64 = i as f64;
    println!("i32 -> f64: {} -> {}", i, j); // 42 -> 42

    // char から u8
    let k: char = 'A';
    let l: u8 = k as u8;
    println!("char -> u8: '{}' -> {}", k, l); // 'A' -> 65

    // u8 から char
    let m: u8 = 65;
    let n: char = m as char;
    println!("u8 -> char: {} -> '{}'", m, n); // 65 -> 'A'
}
```

#### キャスト時の注意点

```rust
fn main() {
    // オーバーフローの例
    // u8 の範囲: 0〜255
    let big: u16 = 1000;
    let small: u8 = big as u8;
    println!("1000 as u8 = {}", small); // 232（1000 % 256）

    // 負数のキャスト
    let negative: i8 = -10;
    let unsigned: u8 = negative as u8;
    println!("-10 as u8 = {}", unsigned); // 246

    // bool は整数にキャスト可能
    let t: bool = true;
    let f: bool = false;
    println!("true as u8 = {}", t as u8);   // 1
    println!("false as u8 = {}", f as u8);  // 0
}
```

### リテラルと型注釈

数値リテラルには型サフィックスや型注釈で型を指定できます。

```rust
fn main() {
    // 型サフィックス
    let a = 42i32;      // i32型
    let b = 42u8;       // u8型
    let c = 3.14f32;    // f32型
    let d = 3.14f64;    // f64型

    // 型注釈
    let e: i64 = 42;    // i64型
    let f: u32 = 42;    // u32型

    // リテラルの記法
    let dec = 1_000;        // 10進数（アンダースコアOK）
    let hex = 0xff;         // 16進数
    let oct = 0o77;         // 8進数
    let bin = 0b1111_0000;  // 2進数
    let byte = b'A';        // バイトリテラル（u8のみ）

    println!("10進数: {}", dec);     // 1000
    println!("16進数: {}", hex);     // 255
    println!("8進数: {}", oct);      // 63
    println!("2進数: {}", bin);      // 240
    println!("バイト: {}", byte);    // 65

    // リテラルの型が未確定な場合、制約から推論される
    let x = 42;     // 型が未確定
    let y: i32 = x; // ここでxの型がi32に確定
}
```

#### 数値リテラルの接頭辞

| 接頭辞 | 基数 | 例 |
|--------|------|----|
| なし | 10進数 | `1_000` |
| `0x` | 16進数 | `0xff` |
| `0o` | 8進数 | `0o77` |
| `0b` | 2進数 | `0b1111_0000` |
| `b` | バイト | `b'A'`（u8のみ） |

### 型推論（Type Inference）

Rustのコンパイラは文脈から変数の型を推論します。

```rust
fn main() {
    // 型注釈なしでも推論される
    let x = 5;          // i32（整数リテラルのデフォルト）
    let y = 3.14;       // f64（浮動小数点リテラルのデフォルト）
    let z = "hello";    // &str

    // 使用箇所から推論される
    let mut v = Vec::new(); // この時点では型は未確定
    v.push(42);             // i32がpushされた → Vec<i32> と推論
    println!("{:?}", v);

    // 関数の戻り値から推論
    fn make_string() -> String {
        String::from("hello")
    }
    let s = make_string(); // Stringと推論

    // 型注釈が必要な場合
    // let ambiguous = Default::default(); // エラー！型が推論できない
    let explicit: i32 = Default::default(); // OK
    println!("explicit = {}", explicit); // 0
}
```

#### 推論のルール

- 整数リテラルのデフォルト: `i32`
- 浮動小数点リテラルのデフォルト: `f64`
- コンパイラは変数の使用箇所から型を決定
- 型が一つに決まらない場合は型注釈が必要

### 型エイリアス（`type`）

`type` キーワードで既存の型に新しい名前をつけられます。

```rust
// 基本的な型エイリアス
type Age = u32;
type Score = i64;

// ジェネリクスを使った型エイリアス
type IntPair = (i32, i32);
type FloatList = Vec<f64>;

// 複雑な型をシンプルに
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    let age: Age = 25;
    let score: Score = 100;
    let pair: IntPair = (1, 2);
    let list: FloatList = vec![1.0, 2.0, 3.0];

    println!("年齢: {}", age);
    println!("スコア: {}", score);
    println!("ペア: {:?}", pair);
    println!("リスト: {:?}", list);
}
```

#### `type` の特徴

- 新しい型を作るのではなく、既存の型の別名
- 型注釈がシンプルになる
- ドキュメンテーションの役割も果たす
- ジェネリクスと組み合わせ可能
