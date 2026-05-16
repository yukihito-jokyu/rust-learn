# 16 - マクロ（Macros）

## 学習内容

- マクロの基本概念
- `macro_rules!`による宣言的マクロ
- 指定子（Designators）
- オーバーロード
- 繰り返し（Repetition）
- DRY（Don't Repeat Yourself）
- DSL（Domain Specific Languages）

## 参考ページ

https://doc.rust-jp.rs/rust-by-example-ja/macros.html

## 内容

### マクロの基本

マクロはメタプログラミングの手段であり、コンパイル時にコードを生成します。関数とは異なり、可変長の引数を取ることができ、コンパイル時に展開されます。

Rustのマクロには以下の種類があります：

- **宣言的マクロ**（`macro_rules!`）
- **手続き的マクロ**（deriveマクロ、属性マクロ、関数マクロ）

### macro_rules!による宣言的マクロ

```rust
// 基本的なマクロの定義
macro_rules! say_hello {
    () => {
        println!("こんにちは！");
    };
}

// 引数を取るマクロ
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("関数 {:?} が呼ばれました", stringify!($func_name));
        }
    };
}

fn main() {
    say_hello!();

    create_function!(foo);
    create_function!(bar);

    foo(); // 関数 "foo" が呼ばれました
    bar(); // 関数 "bar" が呼ばれました
}
```

### 指定子（Designators）

マクロの引数の型は「指定子」で指定します。

```rust
macro_rules! print_type {
    // ident: 識別子（変数名、関数名など）
    ($name:ident) => {
        println!("識別子: {}", stringify!($name));
    };
}

macro_rules! inspect {
    // expr: 式
    ($e:expr) => {
        println!("{} = {:?}", stringify!($e), $e);
    };
}

macro_rules! print_result {
    // ブロックも使える
    ($e:expr) => {
        println!("{} = {:?}", stringify!($e), $e);
    };
}

fn main() {
    print_type!(variable_name); // 識別子: variable_name
    inspect!(1 + 2);            // 1 + 2 = 3
    inspect!(String::from("hello")); // String::from("hello") = "hello"

    let x = 5;
    print_result!(x * 2 + 1);   // x * 2 + 1 = 11
}
```

主な指定子の一覧：

| 指定子     | 説明           | 例                                    |
| ---------- | -------------- | ------------------------------------- |
| `ident`    | 識別子         | `x`, `my_func`                        |
| `expr`     | 式             | `1 + 2`, `foo()`                      |
| `ty`       | 型             | `i32`, `String`                       |
| `path`     | パス           | `std::collections::HashMap`           |
| `stmt`     | 文             | `let x = 5;`                          |
| `block`    | ブロック       | `{ ... }`                             |
| `pat`      | パターン       | `Some(x)`                             |
| `literal`  | リテラル       | `42`, `"hello"`                       |
| `meta`     | メタアイテム   | `#[derive(Debug)]` の `derive(Debug)` |
| `tt`       | トークンツリー | 任意のトークン                        |
| `item`     | アイテム       | `fn foo() {}`, `struct Bar;`          |
| `lifetime` | ライフタイム   | `'a`, `'static`                       |
| `vis`      | 可視性修飾子   | `pub`, `pub(crate)`                   |

### オーバーロード

マクロはパターンを複数定義することで、異なる引数形式をオーバーロードできます。

```rust
macro_rules! test {
    // 引数なし
    ($left:expr) => {
        println!("値: {:?}", $left);
    };
    // 2つの引数を比較
    ($left:expr, $right:expr) => {
        println!("{:?} と {:?} を比較: {}", $left, $right, $left == $right);
    };
    // 条件付きテスト
    ($left:expr, $right:expr, $should_eq:expr) => {
        if $should_eq {
            test!($left, $right);
        } else {
            println!("比較をスキップ: {:?}, {:?}", $left, $right);
        }
    };
}

fn main() {
    test!(1);                    // 値: 1
    test!(1, 1);                 // 1 と 1 を比較: true
    test!(1, 2);                 // 1 と 2 を比較: false
    test!(1 + 1, 2, true);       // 2 と 2 を比較: true
    test!(1 + 1, 2, false);      // 比較をスキップ: 2, 2
}
```

### 繰り返し（Repetition）

`$(...)` 構文を使って、引数の繰り返しを処理できます。

```rust
// カンマ区切りの引数を全て表示するマクロ
macro_rules! print_all {
    ($($e:expr),*) => {
        $(println!("{}", $e);)*
    };
}

// 最後のカンマを許可する
macro_rules! vec_macro {
    ($($e:expr),* $(,)?) => {
        {
            let mut v = Vec::new();
            $(v.push($e);)*
            v
        }
    };
}

// 再帰的なマクロで和を計算
macro_rules! sum {
    ($e:expr) => { $e };
    ($e:expr, $($rest:expr),+) => {
        $e + sum!($($rest),+)
    };
}

fn main() {
    print_all!(1, 2, 3, 4, 5);

    let v = vec_macro![1, 2, 3,];
    println!("vec: {:?}", v);

    let total = sum!(1, 2, 3, 4, 5);
    println!("合計: {}", total); // 15
}
```

繰り返しの構文：

- `$(...),*` - 0回以上の繰り返し（カンマ区切り）
- `$(...),+` - 1回以上の繰り返し（カンマ区切り）
- `$(...),?` - 0回または1回
- `$(,)` や `$(;)` のように区切り文字を変更可能

### DRY（Don't Repeat Yourself）

マクロを使って、繰り返しパターンを抽象化できます。

```rust
// 複数の構造体に同じ実装を提供するマクロ
macro_rules! impl_display_for {
    ($t:ty) => {
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}の値: {}", stringify!($t), self.0)
            }
        }
    };
}

struct Km(i32);
struct Miles(i32);

impl_display_for!(Km);
impl_display_for!(Miles);

fn main() {
    let distance_km = Km(100);
    let distance_miles = Miles(62);

    println!("{}", distance_km);    // Kmの値: 100
    println!("{}", distance_miles); // Milesの値: 62
}
```

### DSL（Domain Specific Languages）

マクロを使って、ドメイン固有の言語（DSL）を構築できます。

```rust
// 簡易HTML DSL
macro_rules! html {
    ($tag:ident { $($child:tt)* }) => {
        format!("<{}>{}</{}>", stringify!($tag), html_inner!($($child)*), stringify!($tag))
    };
}

macro_rules! html_inner {
    ($text:literal) => { $text.to_string() };
    ($tag:ident { $($child:tt)* }) => {
        html!($tag { $($child)* })
    };
}

// 簡易テストDSL
macro_rules! test_suite {
    ($($name:ident: $body:expr);* $(;)?) => {
        $(
            #[test]
            fn $name() {
                $body;
            }
        )*
    };
}

fn main() {
    let output = html!(div { p { "こんにちは" } });
    // 注意: この単純な例ではネストは限定的
    println!("{}", output);
}
```
