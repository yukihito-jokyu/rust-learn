// TODO: Classic構造体を定義して使ってください
// ヒント: struct Person { name: String, age: u32, }

// TODO: Tuple構造体を定義して使ってください
// ヒント: struct Color(u8, u8, u8);

// TODO: Unit構造体を定義して使ってください
// ヒント: struct Unit;

// TODO: Enumを定義してパターンマッチングを試してください
// ヒント: enum WebEvent { PageLoad, KeyPress(char), Click { x: i64, y: i64 } }
// ヒント: matchで分岐

// TODO: 定数を宣言してください
// ヒント: const MAX: u32 = 100;
// ヒント: static LANGUAGE: &str = "Rust";

// Classic構造体
struct Person {
    name: String,
    age: u32,
}

// Tuple構造体
struct Color(u8, u8, u8);
struct Point(f64, f64);

// Unit構造体
// フィールドがない構造体
struct Unit;

// メソッドの定義
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // コンストラクタ
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // 正方形を作る関連関数
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    // メソッド(&selfを取る)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // ミュータブルなメソッド
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

// Enum
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum WebEvent {
    PageLoad,
    KeyPress(char),
    Click { x: i64, y: i64 },
    Paste(String),
}

// useでバリアントをインポート
use WebEvent::*;

fn inspect(event: WebEvent) {
    match event {
        PageLoad => println!("ページが読み込まれました"),
        KeyPress(c) => println!("キー '{}' が押されました", c),
        Click { x, y } => println!("クリック位置: ({}. {})", x, y),
        Paste(s) => println!("貼り付け: {}", s),
    }
}

fn main() {
    // ここにコードを書いてください
    // インスタンス化
    let person = Person { name: String::from("山田"), age: 32 };

    println!("名前: {}, 年齢: {}", person.name, person.age);

    // イミュータブルなインスタンス化
    let mut person = Person { name: String::from("佐藤"), age: 25 };
    person.age = 26;

    println!("名前: {}, 年齢: {}", person.name, person.age);

    // インスタンス化
    let red = Color(255, 0, 0);
    let origin = Point(0.0, 0.0);

    // アクセス
    println!("赤の緑成分: {}", red.1);
    println!("原点のx座標: {}", origin.0);

    // 分解
    let Color(r, g, b) = red;
    println!("R: {}, G: {}, B: {}", r, g, b);

    // インスタンス化
    let unit = Unit;

    let rect = Rectangle::new(10, 20);
    println!("面積: {}", rect.area());
    let mut sq = Rectangle::square(5);
    println!("面積: {}", sq.area());
    sq.scale(4);
    println!("面積の変更: {}", sq.area());

    let events = [PageLoad, KeyPress('x'), Click { x: 100, y: 200 }, Paste(String::from("hello"))];

    for event in events {
        inspect(event);
    }

    // 定数
    const MAX_POINTS: u32 = 100_000;
}
