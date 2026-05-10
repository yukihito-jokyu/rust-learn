# ジェネリクス（Generics）

## 学習内容
- ジェネリック関数
- ジェネリック実装
- トレイト境界
- 複数境界（`+`）
- `where` 句
- 関連型（Associated Types）
- PhantomData

## 参考ページ
https://doc.rust-jp.rs/rust-by-example-ja/generics.html

## 内容

### ジェネリクスとは

ジェネリクスは、型をパラメータとして受け取る仕組みです。コードの重複を減らし、様々な型に対応する汎用的な関数や構造体を作成できます。

### ジェネリック関数

型パラメータを使って、様々な型に対応する関数を定義できます。

```rust
// T は型パラメータ
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest char: {}", largest(&chars));
}
```

- `<T>` で型パラメータを宣言
- `T: PartialOrd` はトレイト境界（比較可能な型のみ）
- 異なる型で同じ関数を使い回せる

### ジェネリック構造体

構造体のフィールドの型もジェネリクスで指定できます。

```rust
// 単一の型パラメータ
struct Point<T> {
    x: T,
    y: T,
}

// 複数の型パラメータ
struct Pair<T, U> {
    first: T,
    second: U,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    let pair = Pair { first: 1, second: 2.5 };
    println!("pair: ({}, {})", pair.first, pair.second);
}
```

### ジェネリック実装

構造体に対してジェネリックな `impl` ブロックを定義できます。

```rust
struct Point<T> {
    x: T,
    y: T,
}

// T を持つ全ての Point に適用される impl
impl<T: std::fmt::Display> Point<T> {
    fn display(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

// 特定の型にのみ適用される impl
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    p1.display(); // (1, 2)

    let p2 = Point { x: 3.0, y: 4.0 };
    p2.display(); // (3, 4)
    println!("distance: {}", p2.distance_from_origin()); // 5.0
}
```

### トレイト境界

ジェネリック型パラメータに「どのトレイトを実装しているか」を指定できます。

```rust
use std::fmt::Display;

// インラインでトレイト境界を指定
fn print_it<T: Display>(value: T) {
    println!("{}", value);
}

fn main() {
    print_it(42);       // i32
    print_it("hello");  // &str
    print_it(3.14);     // f64
}
```

#### よく使うトレイト境界

| トレイト境界 | 意味 |
|---|---|
| `Display` | `{}` で表示可能 |
| `Debug` | `{:?}` で表示可能 |
| `Clone` | `.clone()` で複製可能 |
| `Copy` | 代入時に自動複製 |
| `PartialOrd` | 比較演算子が使える |
| `ToString` | `.to_string()` が使える |
| `Default` | `T::default()` が使える |

### 複数境界（`+`）

複数のトレイト境界を `+` で組み合わせられます。

```rust
use std::fmt::{Display, Debug};

fn compare_and_print<T: Display + PartialOrd>(a: T, b: T) {
    if a > b {
        println!("{} > {}", a, b);
    } else if a < b {
        println!("{} < {}", a, b);
    } else {
        println!("{} == {}", a, b);
    }
}

fn duplicate<T: Clone>(value: T) -> (T, T) {
    (value.clone(), value)
}

fn main() {
    compare_and_print(3, 5);       // 3 < 5
    compare_and_print(10, 2);      // 10 > 2
    compare_and_print("a", "a");   // a == a

    let pair = duplicate(String::from("hello"));
    println!("pair: ({}, {})", pair.0, pair.1);
}
```

### `where` 句

トレイト境界が複雑な場合、`where` 句を使うと読みやすくなります。

```rust
use std::fmt::Display;

// インライン境界（読みにくい）
fn some_function<T: Display + Clone, U: Clone + Display>(t: &T, u: &U) -> String {
    format!("{} {}", t, u)
}

// where 句（読みやすい）
fn some_function_better<T, U>(t: &T, u: &U) -> String
where
    T: Display + Clone,
    U: Clone + Display,
{
    format!("{} {}", t, u)
}

fn main() {
    let result = some_function_better(&"hello", &42);
    println!("{}", result);
}
```

- 関数シグネチャが長くなるときに `where` 句を使う
- `where` 句は `<>` の後ろ、`{` の前に書く
- 型パラメータとトレイト境界の対応が明確になる

### 関連型（Associated Types）

トレイト内で型を関連付けることができます。

```rust
trait Container {
    type Item; // 関連型

    fn get(&self) -> &Self::Item;
    fn set(&mut self, item: Self::Item);
}

struct NumberContainer {
    value: i32,
}

impl Container for NumberContainer {
    type Item = i32; // 関連型を具体的な型に指定

    fn get(&self) -> &Self::Item {
        &self.value
    }

    fn set(&mut self, item: Self::Item) {
        self.value = item;
    }
}

fn main() {
    let mut c = NumberContainer { value: 42 };
    println!("value: {}", c.get());
    c.set(100);
    println!("value: {}", c.get());
}
```

### PhantomData

`PhantomData<T>` はサイズがゼロの型で、型パラメータが使われていないことをコンパイラに伝えるために使います。

```rust
use std::marker::PhantomData;

// 型パラメータ T を宣言しているが、フィールドでは使っていない
struct UnusedType<T> {
    data: String,
    _phantom: PhantomData<T>, // コンパイラに「Tを使う」と伝える
}

// 単位を型レベルで区別する例
struct Meters;
struct Kilometers;

struct Distance<Unit> {
    value: f64,
    _phantom: PhantomData<Unit>,
}

impl<Unit> Distance<Unit> {
    fn new(value: f64) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }
}

fn main() {
    let _unused: UnusedType<i32> = UnusedType {
        data: String::from("hello"),
        _phantom: PhantomData,
    };

    let d1: Distance<Meters> = Distance::new(1000.0);
    let d2: Distance<Kilometers> = Distance::new(1.0);

    // d1 と d2 は異なる型なので混同できない
    println!("meters: {}", d1.value);
    println!("kilometers: {}", d2.value);

    // let _ = d1 + d2; // エラー：異なる型
}
```

- `PhantomData<T>` はサイズがゼロ（実行時のオーバーヘッドなし）
- 未使用の型パラメータのコンパイルエラーを防ぐ
- 型レベルでデータを区別するのに便利
