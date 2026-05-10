# 15 - トレイト（Traits）

## 学習内容
- トレイトの定義と実装
- デフォルトメソッド
- トレイト境界（Trait Bounds）
- `derive`属性
- 演算子オーバーロード
- `Drop`トレイト
- `Iterator`トレイト
- トレイトオブジェクトと動的ディスパッチ

## 参考ページ
https://doc.rust-jp.rs/rust-by-example-ja/trait.html

## 内容

### トレイトの定義と実装

トレイトは、ある型が実装しなければならないメソッドの集合を定義します。他の言語の「インターフェース」に似ています。

```rust
// トレイトの定義
trait Animal {
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // デフォルト実装を提供することもできる
    fn talk(&self) {
        println!("{}は「{}」と言った！", self.name(), self.noise());
    }
}

struct Dog { name: &'static str }
struct Cat { name: &'static str }

// トレイトの実装
impl Animal for Dog {
    fn name(&self) -> &'static str { self.name }
    fn noise(&self) -> &'static str { "ワンワン！" }
}

impl Animal for Cat {
    fn name(&self) -> &'static str { self.name }
    fn noise(&self) -> &'static str { "ニャー！" }
    // デフォルトメソッドをオーバーライドすることも可能
    fn talk(&self) {
        println!("{}は甘えています！", self.name());
    }
}

fn main() {
    let dog = Dog { name: "ポチ" };
    let cat = Cat { name: "ミケ" };

    dog.talk(); // ポチは「ワンワン！」と言った！
    cat.talk(); // ミケは甘えています！
}
```

### トレイト境界（Trait Bounds）

ジェネリック型パラメータにトレイト境界を指定することで、そのトレイトを実装している型のみを受け入れるようにできます。

```rust
// ジェネリック関数にトレイト境界を指定
fn print_animal<T: Animal>(animal: &T) {
    println!("{}の鳴き声: {}", animal.name(), animal.noise());
}

// 複数のトレイト境界
fn debug_display<T: std::fmt::Debug + std::fmt::Display>(item: &T) {
    println!("Display: {}", item);
    println!("Debug: {:?}", item);
}

// where句を使ったより読みやすい書き方
fn some_function<T, U>(t: &T, u: &U)
where
    T: std::fmt::Display + Clone,
    U: std::fmt::Debug + Clone,
{
    println!("T: {}, U: {:?}", t, u);
}
```

### derive属性

よく使われるトレイトは`#[derive]`属性を使って自動的に実装できます。

```rust
// PartialEq, Debug, Clone, Copyなどをderive
#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 1.0, y: 2.0 };

    // Debugトレイトのおかげで {:?} で表示できる
    println!("{:?}", p1); // Point { x: 1.0, y: 2.0 }

    // PartialEqトレイトのおかげで比較できる
    println!("等しいか: {}", p1 == p2); // true

    // Cloneトレイトのおかげで複製できる
    let p3 = p1.clone();
}
```

### 演算子オーバーロード

`std::ops`モジュールのトレイトを実装することで、演算子をオーバーロードできます。

```rust
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
struct Vector2D {
    x: f64,
    y: f64,
}

// + 演算子のオーバーロード
impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// * 演算子のオーバーロード（スカラー倍）
impl Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, scalar: f64) -> Vector2D {
        Vector2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

fn main() {
    let v1 = Vector2D { x: 1.0, y: 2.0 };
    let v2 = Vector2D { x: 3.0, y: 4.0 };

    let sum = v1 + v2;
    println!("和: {:?}", sum); // Vector2D { x: 4.0, y: 6.0 }

    let scaled = v1 * 2.0;
    println!("スカラー倍: {:?}", scaled); // Vector2D { x: 2.0, y: 4.0 }
}
```

### Dropトレイト

`Drop`トレイトを実装すると、値がスコープを抜けるときにカスタムクリーンアップコードを実行できます。デストラクタに相当します。

```rust
struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("リソース「{}」を解放しています...", self.name);
    }
}

fn main() {
    {
        let r1 = Resource { name: String::from("ファイルA") };
        let r2 = Resource { name: String::from("ファイルB") };
        println!("リソースを作成しました");
        // r2が先にドロップされ、次にr1がドロップされる（逆順）
    }
    println!("スコープ終了");
}
```

### Iteratorトレイト

`Iterator`トレイトを実装することで、カスタムイテレータを作成できます。

```rust
// フィボナッチ数列のイテレータ
struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        let current = self.curr;
        self.curr = self.next;
        self.next = new_next;
        Some(current)
    }
}

fn main() {
    // フィボナッチ数列の最初の10個を取得
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("フィボナッチ数列: {:?}", fibs);
    // [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
}
```

### トレイトオブジェクトと動的ディスパッチ

`dyn Trait`を使うことで、異なる型を同じトレイトオブジェクトとして扱えます。

```rust
trait Draw {
    fn draw(&self);
}

struct Circle { radius: f64 }
struct Rectangle { width: f64, height: f64 }

impl Draw for Circle {
    fn draw(&self) {
        println!("半径 {} の円を描画", self.radius);
    }
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("{}x{} の長方形を描画", self.width, self.height);
    }
}

fn main() {
    // &dyn Trait を使って異なる型を格納
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 3.0, height: 4.0 }),
    ];

    for shape in shapes {
        shape.draw();
    }
}
```
