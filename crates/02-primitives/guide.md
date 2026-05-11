# プリミティブ型（Primitives）

## 学習内容

- 整数型（i8, i16, i32, i64, i128, u8, u16, u32, u64, u 128）
- 浮動小数点型（f32, f64）
- 真偽型（bool）
- 文字型（char）
- タプル
- 配列

## 参考ページ

https://doc.rust-jp.rs/rust-by-example-ja/primitives.html

## 内容

### 整数型

Rustの整数型には符号付き（i系）と符号なし（u系）があります。

```rust
fn main() {
    // 符号付き整数
    let a: i8 = -128;          // -128 〜 127
    let b: i16 = 32767;        // -32768 〜 32767
    let c: i32 = 2_147_483_647; // アンダースコアで区切れる
    let d: i64 = 123_456_789;
    let e: i128 = 123_456_789_012_345;

    // 符号なし整数
    let f: u8 = 255;           // 0 〜 255
    let g: u16 = 65535;
    let h: u32 = 4_294_967_295;
    let i: u64 = 123_456_789;
    let j: u128 = 123_456_789_012_345;

    // 型サフィックスによる指定
    let k = 42i32;
    let l = 42u8;

    // アーキテクチャ依存のサイズ
    let m: isize = -1;  // ポインタのサイズと同じ（符号付き）
    let n: usize = 1;   // ポインタのサイズと同じ（符号なし）

    println!("a = {}, f = {}, m = {}, n = {}", a, f, m, n);
}
```

| 型      | サイズ             | 範囲                |
| ------- | ------------------ | ------------------- |
| `i8`    | 1バイト            | -128 〜 127         |
| `u8`    | 1バイト            | 0 〜 255            |
| `i16`   | 2バイト            | -32768 〜 32767     |
| `u16`   | 2バイト            | 0 〜 65535          |
| `i32`   | 4バイト            | 約-21億 〜 約21億   |
| `u32`   | 4バイト            | 0 〜 約42億         |
| `i64`   | 8バイト            | 約-922京 〜 約922京 |
| `u64`   | 8バイト            | 0 〜 約1844京       |
| `i128`  | 16バイト           | 非常に大きな範囲    |
| `u128`  | 16バイト           | 非常に大きな範囲    |
| `isize` | アーキテクチャ依存 | ポインタサイズ      |
| `usize` | アーキテクチャ依存 | ポインタサイズ      |

### 浮動小数点型

```rust
fn main() {
    let x: f32 = 3.14;       // 単精度（32ビット）
    let y: f64 = 2.71828;    // 倍精度（64ビット）— デフォルト

    // 演算
    let sum = x as f64 + y;
    let diff = y - x as f64;
    let product = x as f64 * y;
    let quotient = y / x as f64;
    let remainder = y % x as f64;

    println!("x = {}, y = {}", x, y);
    println!("和 = {}, 差 = {}", sum, diff);
    println!("積 = {}, 商 = {}", product, quotient);
    println!("余り = {}", remainder);
}
```

- `f32` --- 単精度浮動小数点数（IEEE 754準拠）
- `f64` --- 倍精度浮動小数点数（デフォルト、IEEE 754準拠）

### 真偽型

```rust
fn main() {
    let t: bool = true;
    let f: bool = false;

    // 演算
    println!("t && f = {}", t && f);   // 論理積 (AND)
    println!("t || f = {}", t || f);   // 論理和 (OR)
    println!("!t = {}", !t);           // 否定 (NOT)
}
```

- `bool`型は `true` または `false` の値をとる
- サイズは1バイト

### 文字型

```rust
fn main() {
    let c: char = 'z';
    let z: char = '\u{1F600}'; // Unicode絵文字

    println!("c = {}, z = {}", c, z);

    // charはUnicodeスカラー値（4バイト）
    // 文字列の1文字とは限らない（書記素クラスタに注意）
}
```

- `char`型はUnicodeスカラー値を表す
- サイズは4バイト
- シングルクォート `'` で囲む（ダブルクォート `"` は文字列）

### タプル

```rust
fn main() {
    // タプルの作成
    let tuple: (i32, f64, char) = (500, 6.4, 'a');

    // インデックスによるアクセス
    println!("1番目: {}", tuple.0);
    println!("2番目: {}", tuple.1);
    println!("3番目: {}", tuple.2);

    // 分解（デストラクト）
    let (x, y, z) = tuple;
    println!("x = {}, y = {}, z = {}", x, y, z);

    // ネストしたタプル
    let nested = ((1, 2), (3, 4));
    println!("nested.0.1 = {}", nested.0.1);

    // タプルのデバッグ出力
    println!("tuple = {:?}", tuple);
}
```

- タプルは異なる型の値をまとめる
- インデックスは `.` 演算子でアクセス（`tuple.0`, `tuple.1`）
- 分解（デストラクト）が可能
- 長さは固定

### 配列

```rust
fn main() {
    // 配列の作成
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // 全要素を同じ値で初期化
    let zeros = [0; 10]; // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    // インデックスによるアクセス
    println!("arr[0] = {}", arr[0]);
    println!("arr[4] = {}", arr[4]);

    // 配列の長さ
    println!("len = {}", arr.len());

    // 配列はスタックに確保される
    // 長さは固定で変更不可（ただし要素自体はmutなら変更可能）
    let mut mutable_arr = [1, 2, 3];
    mutable_arr[0] = 100;
    println!("mutable_arr = {:?}", mutable_arr);

    // 範囲外アクセスはパニック
    // println!("{}", arr[10]); // 実行時エラー（パニック）
}
```

- 配列は同じ型の要素の固定長リスト
- スタックに確保される
- 長さはコンパイル時に決まる
- `[T; N]` で型と長さを指定
- `[value; count]` で同じ値の初期化が可能
