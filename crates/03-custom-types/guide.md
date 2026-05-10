# カスタム型（Custom Types）

## 学習内容
- 構造体（Classic C構造体、Tuple構造体、Unit構造体）
- Enum（列挙型）とパターンマッチング
- 定数（`const`、`static`）
- `use`によるエイリアス

## 参考ページ
https://doc.rust-jp.rs/rust-by-example-ja/custom_types.html

## 内容

### 構造体（struct）

Rustには3種類の構造体があります。

#### Classic（名前付きフィールド）構造体

```rust
// 定義
struct Person {
    name: String,
    age: u32,
}

// インスタンス化
let person = Person {
    name: String::from("山田"),
    age: 30,
};

// フィールドへのアクセス
println!("名前: {}, 年齢: {}", person.name, person.age);

// ミュータブルなインスタンス
let mut person = Person {
    name: String::from("佐藤"),
    age: 25,
};
person.age = 26; // mutが必要
```

#### Tuple（タプル）構造体

```rust
// 定義（フィールド名がない）
struct Color(u8, u8, u8);
struct Point(f64, f64);

// インスタンス化
let red = Color(255, 0, 0);
let origin = Point(0.0, 0.0);

// アクセス
println!("赤の緑成分: {}", red.1);
println!("原点のx座標: {}", origin.0);

// 分解
let Color(r, g, b) = red;
println!("R: {}, G: {}, B: {}", r, g, b);
```

#### Unit（ユニット）構造体

```rust
// フィールドがない構造体
struct Unit;

// インスタンス化（データを持たない）
let unit = Unit;
```

ユニット構造体は、型レベルでのマーカーとして使われます。トレイトを実装する際などに有用です。

### メソッドの定義

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 関連関数（selfを取らない）--- コンストラクタとしてよく使われる
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // 正方形を作る関連関数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // メソッド（&selfを取る）
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // ミュータブルなメソッド
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

let rect = Rectangle::new(10, 20);
println!("面積: {}", rect.area());

let sq = Rectangle::square(5);
println!("正方形の面積: {}", sq.area());
```

### Enum（列挙型）

```rust
// 基本的なEnum
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// データを持つEnum
enum WebEvent {
    PageLoad,                          // データなし
    KeyPress(char),                    // タプルのようなデータ
    Click { x: i64, y: i64 },         // 構造体のようなデータ
    Paste(String),                     // Stringを持つ
}

// useでバリアントをインポート
use WebEvent::*;

fn inspect(event: WebEvent) {
    match event {
        PageLoad => println!("ページが読み込まれました"),
        KeyPress(c) => println!("キー '{}' が押されました", c),
        Click { x, y } => println!("クリック位置: ({}, {})", x, y),
        Paste(s) => println!("貼り付け: \"{}\"", s),
    }
}

let events = [
    WebEvent::PageLoad,
    WebEvent::KeyPress('x'),
    WebEvent::Click { x: 100, y: 200 },
    WebEvent::Paste(String::from("hello")),
];

for event in events {
    inspect(event);
}
```

#### Enumの便利な使い方

```rust
// Option型（標準ライブラリ）
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

match divide(10.0, 3.0) {
    Some(result) => println!("結果: {}", result),
    None => println!("ゼロ除算です"),
}
```

### 定数

```rust
// const: コンパイル時に評価される定数（型注釈必須）
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159265358979;

// static: プログラム全体の寿命を持つ変数（型注釈必須）
static LANGUAGE: &str = "Rust";

// static mut: グローバルな可変変数（unsafeが必要）
// static mut COUNTER: u32 = 0;

fn main() {
    println!("最大ポイント: {}", MAX_POINTS);
    println!("言語: {}", LANGUAGE);
}
```

#### `const` と `static` の違い

| 特徴 | `const` | `static` |
|------|---------|----------|
| 評価タイミング | コンパイル時 | 実行時 |
| メモリ位置 | インライン展開 | 固定アドレス |
| 可変性 | 不可 | `static mut`で可能（unsafe） |
| 型注釈 | 必須 | 必須 |
| 参照の取得 | 不可 | 可能 |

### `use`によるエイリアス

```rust
enum Status {
    Ok,
    Error,
}

// useで型にエイリアスを作る
use Status::{Ok, Error};

// または全てインポート
use Status::*;

// typeキーワードで型エイリアス
type Name = String;

let name: Name = String::from("太郎");
println!("{}", name);
```
